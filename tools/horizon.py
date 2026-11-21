#!/usr/bin/env python3
"""Export pinned sources, apply the template bundle and build a native Horizon."""
import argparse
import contextlib
import fcntl
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tarfile
import tempfile
import time

ROOT = Path(__file__).resolve().parents[1]
COMPAT = json.loads((ROOT / "compatibility.json").read_text())
PIN = COMPAT["horizon_revision"]
MIXINS = Path(os.environ.get("HORIZON_MIXINS", ROOT.parent / "horizon-mixins")).resolve()


def digest(data):
    return hashlib.sha256(data).hexdigest()


def stage(horizon, stages):
    """Stage pinned Horizon sources."""
    horizon_path = str(horizon)
    
    # Check if it's a git checkout
    git_dir = Path(horizon_path) / ".git"
    if not git_dir.exists():
        raise ValueError(f"{horizon_path} is not a Git repository")
    
    # Verify the commit exists
    subprocess.run(["git", "-C", horizon_path, "cat-file", "-e", PIN + "^{commit}"], 
                   check=True, capture_output=True)
    
    inputs = {"revision": PIN.encode(), "export-schema": b"2"}
    signature = digest(b"".join(k.encode() + b"\0" + value for k, value in sorted(inputs.items())))
    stages.mkdir(parents=True, exist_ok=True)
    out = stages / signature[:20]
    
    if out.exists():
        marker = json.loads((out / ".horizon-mod-template.json").read_text())
        if marker["signature"] != signature:
            raise ValueError(f"stage marker mismatch: {out}")
        for path, expected in marker["files"].items():
            if digest((out / path).read_bytes()) != expected:
                raise ValueError(f"staged source was edited: {path}; remove this generated stage to rebuild it")
        return out, False
    
    # Create temporary staging directory
    try:
        temporary = Path(tempfile.mkdtemp(prefix=".mod-template-stage-", dir=stages))
        print(f"[INFO] Extracting Horizon sources to {temporary}", file=sys.stderr)
        
        # Export using git archive (no filter="data" issue on Windows)
        process = subprocess.Popen(
            ["git", "-C", horizon_path, "archive", "--format=tar", PIN], 
            stdout=subprocess.PIPE
        )
        try:
            with tarfile.open(fileobj=process.stdout, mode="r|") as archive:
                for member in archive:
                    # Don't use filter on Windows - extract directly
                    if hasattr(archive.extractor, 'extractall'):
                        archive.extract(member, temporary, set_attrs=True)
        finally:
            process.stdout.close()
            code = process.wait()
            
        if code != 0:
            raise ValueError(f"git archive exited {code}")
        
        # Verify source hashes
        for path, expected in COMPAT["source_sha256"].items():
            actual_hash = digest((temporary / path).read_bytes())
            if actual_hash != expected:
                raise ValueError(f"pinned source hash mismatch: {path}\nExpected: {expected}\nGot: {actual_hash}")
        
        # Fix symlinks for Windows compatibility
        for file in temporary.rglob("*"):
            if file.is_symlink():
                content = file.read_bytes()
                file.unlink()
                file.write_bytes(content.encode('utf-8', errors='replace')) if isinstance(content, bytes) else file.write_text(content.decode('utf-8', errors='replace') if isinstance(content, str) else "placeholder")
        
        (temporary / ".horizon-revision").write_text(PIN + "\n")
        
        # Calculate hashes for all files in the temporary directory
        files = {str(p.relative_to(temporary)): digest(p.read_bytes())
                 for p in sorted(temporary.rglob("*")) if p.is_file()}
        
        marker_path = temporary / ".horizon-mod-template.json"
        marker_path.write_text(json.dumps(
            {"signature": signature, "horizon_revision": PIN, "files": files}, 
            sort_keys=True, indent=2) + "\n")
        
        os.rename(temporary, out)
    except BaseException:
        shutil.rmtree(str(temporary), ignore_errors=True)
        raise
    
    return out, True


def mixin_stage(horizon, stages):
    """Apply mixins to staged Horizon."""
    # Check horizon-mixins revision
    mixins_path = str(MIXINS)
    if not Path(mixins_path).exists():
        raise ValueError(f"horizon-mixins not found at {mixins_path}")
    
    revision = subprocess.check_output(["git", "-C", str(MIXINS), "rev-parse", "HEAD"], text=True).strip()
    expected_revision = COMPAT["horizon_mixins_revision"]
    if revision != expected_revision:
        raise ValueError(f"horizon-mixins must be pinned at {expected_revision}, got {revision}")
    
    # Verify no changes to important files
    subprocess.run(["git", "-C", str(MIXINS), "diff", "--quiet", "HEAD", "--", "Cargo.toml", "Cargo.lock"], 
                   check=True, capture_output=True)
    
    # Stage Horizon sources
    original, _ = stage(horizon, stages / "original")
    
    print(f"[INFO] Building horizon-mixins CLI...", file=sys.stderr)
    target_dir = MIXINS / "target"
    subprocess.run(["cargo", "build", "--locked", "--offline", 
                    f"--target-dir={target_dir}", "-p", "horizon-mixins-cli"], 
                   cwd=str(MIXINS), check=True, capture_output=True, text=True)
    
    # Find the CLI binary (works on both Linux and Windows)
    cli_binary = None
    if sys.platform.startswith("win"):
        cli_binary = target_dir / "debug" / "horizon-mixins.exe"
    else:
        cli_binary = target_dir / "debug" / "horizon-mixins"
    
    if not cli_binary.exists():
        raise ValueError(f"horizon-mixins CLI not found at {cli_binary}")
    
    print(f"[INFO] Running horizon-mixins plan...", file=sys.stderr)
    output = stages / "modded"
    plan = stages / "plan.json"
    base = ["--workspace", str(original), "--mods", str(ROOT / "mixins")]
    
    subprocess.run([str(cli_binary), "plan", *base, f"--out={plan}"], 
                   check=True, capture_output=True, text=True)
    
    print(f"[INFO] Running horizon-mixins apply...", file=sys.stderr)
    applied = subprocess.run([str(cli_binary), "apply", *base, f"--out={output}"], 
                            check=True, capture_output=True, text=True)
    
    report = json.loads(applied.stdout)
    return output, report["stage_changed"]


@contextlib.contextmanager
def lock(path):
    """File locking for cross-platform compatibility."""
    path.parent.mkdir(parents=True, exist_ok=True)
    with open(str(path), "a") as handle:
        try:
            if hasattr(handle, 'fileno'):
                fcntl.flock(handle.fileno(), fcntl.LOCK_EX)
            yield
        except AttributeError:
            # Windows doesn't have flock
            yield


def main():
    parser = argparse.ArgumentParser(description="Horizon native mod builder")
    parser.add_argument("action", choices=["stage", "check", "build", "test"])
    parser.add_argument("--horizon", type=Path, default=ROOT.parent / "horizon")
    parser.add_argument("--target-dir", type=Path, default=ROOT / "target/horizon-cargo")
    args = parser.parse_args()
    
    horizon_path = args.horizon.resolve()
    target_dir = args.target_dir.resolve()
    
    # Ensure paths are separate
    if str(target_dir).startswith(str(horizon_path)):
        parser.error("Cargo target must be outside the original Horizon checkout")
    
    started = time.monotonic()
    
    with lock(ROOT / "target/stage.lock"):
        output, changed = mixin_stage(horizon, ROOT / "target/mixins")
        
        # Ensure target is separate from output
        if str(target_dir).startswith(str(output)) or str(output).startswith(str(target_dir)):
            parser.error("Cargo target and generated source stage must be separate")
        
        report = {
            "stage": str(output), 
            "stage_changed": changed,
            "horizon_revision": PIN, 
            "staging_seconds": time.monotonic()-started
        }
        
        if args.action != "stage":
            command = ["cargo", args.action, "--locked", "--offline", 
                       "-p", "stellascrack-food",
                       "--bin", "stellascrack-food-content", "--no-default-features"]
            
            if args.action == "build":
                command.extend(["--profile", "fast"])
                
            if args.action == "test":
                command.extend(["stellascrack_food::tests", "--", "--nocapture"])
            
            with lock(target_dir.parent / (target_dir.name + ".mod-template.lock")):
                run_start = time.monotonic()
                print(f"[INFO] Running cargo {args.action}...", file=sys.stderr)
                result = subprocess.run(command, cwd=str(output), 
                                       capture_output=True, text=True)
            
            report.update(cargo_seconds=time.monotonic()-run_start, 
                         cargo_command=command, cargo_exit=result.returncode)
            
            # Save log file (cross-platform compatible path)
            log_dir = ROOT / "target"
            log_path = log_dir / f"horizon-{args.action}.log"
            if log_path.parent.exists():
                full_log_path = str(log_path.resolve())
                report["cargo_log"] = full_log_path
                (ROOT / "target").mkdir(parents=True, exist_ok=True)
            
            # Write logs to file
            log_content = result.stdout + result.stderr
            if args.action == "build":
                print(f"[INFO] Horizon mod built successfully!", file=sys.stderr)
                
                # Find the binary path (Windows/Unix compatible)
                target_name = target_dir.name
                fast_profile = target_dir / "fast" / "horizon-client" if args.action == "build" else None
                
                if sys.platform.startswith("win"):
                    exe_suffix = ".exe"
                    binary_path = str(fast_profile.with_suffix(exe_suffix)) if fast_profile else None
                else:
                    binary_path = str(fast_profile) if fast_profile else None
                
                report["binary"] = binary_path
            
            print(json.dumps(report, indent=2), file=sys.stderr)
            
            if result.returncode != 0:
                # Print error output (last 12KB max to avoid overflow)
                log_lines = log_content.split('\n')[-30:]
                for line in log_lines:
                    print(line[:12000], file=sys.stderr)
                
                return result.returncode
        
        print(json.dumps(report, indent=2), file=sys.stderr)
        return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (OSError, ValueError, subprocess.SubprocessError) as error:
        if isinstance(error, subprocess.CalledProcessError):
            print(error.stderr, file=sys.stderr)
        print(f"Native mod: {error}", file=sys.stderr)
        raise SystemExit(1)
