@echo off
REM Установка зависимостей для Stella's Crack Food mod

set HORIZON_PATH=F:\Horizon Launcher\horizon
set MIXINS_PATH=F:\Horizon Launcher

echo ====================================
echo  Installing Dependencies...
echo ====================================
echo.

if not exist "%MIXINS_PATH%" (
    echo [INFO] Creating mixins directory...
    mkdir "%MIXINS_PATH%"
)

if not exist "%MIXINS_PATH\.git" (
    echo [INFO] Cloning horizon-mixins...
    git clone https://github.com/horizon-rust/horizon-mixins.git "%MIXINS_PATH%"
    
    if %ERRORLEVEL% NEQ 0 (
        echo [ERROR] Git clone failed!
        pause
        exit /b 1
    )
) else (
    echo [OK] horizon-mixins already exists
)

cd "%MIXINS_PATH%"

echo [INFO] Checking out exact revision...
git checkout d9c83d7096b7ea5b83678da2efdfd6161f74997e --quiet

if %ERRORLEVEL% NEQ 0 (
    echo [WARN] Revision not found, using latest...
) else (
    echo [OK] Exact revision checked out
)

echo [INFO] Fetching dependencies...
cargo fetch --locked

if %ERRORLEVEL% EQU 0 (
    echo.
    echo ====================================
    echo  Dependencies Installed!
    echo ====================================
    echo.
    echo Next step: Run build.bat to compile the mod
    pause
) else (
    echo [ERROR] cargo fetch failed! Check the error above.
    pause
)

cd ..\..

endlocal
