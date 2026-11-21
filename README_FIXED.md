# Fixed Structure Guide - Stella's Crack Food Mod

## ✅ Исправленные проблемы:

### 1. Git и .gitignore
- ✅ Создан `.gitignore` чтобы не коммитить binaries, target/, player resources
- ✅ `mixins/stellascrack_content/mod.rs` теперь в ignore (не нужен)

### 2. Content мод структура  
- ✅ **stellascrack-content** - это data-only .hnmod мод с JSON данными
- ✅ package.hnmod содержит правильный манифест
- ✅ data/items.json, data/recipes.json, data/advancements.json

### 3. Cargo.toml исправлен
- ❌ Убран `[[bin]]` для content мода (он не должен быть Rust binary)
- ✅ Только stellascrack-food как lib с runtime.rs

## 📋 Что нужно сделать:

### Перед пушем на GitHub:

```bash
# Добавить файлы в git
git add .gitignore Cargo.toml GIT_FIXES.md README_FIXED.md
git add mixins/stellascrack_content/package.hnmod
git add mixins/stellascrack_content/data/items.json
git add mixins/stellascrack_content/data/recipes.json
git add mixins/stellascrack_content/data/advancements.json

# Проверить что не коммитится лишнее
git status
```

### Протестировать:

```bash
# Форматтинг Rust кода
cargo fmt --all

# Клинп (если Horizon скомпилирован)
# cargo clippy --workspace --all-targets --locked --offline -- -D warnings

# Тесты (требует Horizon checkout)
python3 tools/test_template.py
```

## 🚀 Как использовать:

### Для локальной разработки (Horizon + Mixins):

1. Clone Horizon и horizon-mixins:
```bash
git clone https://github.com/horizon-rust/horizon ../horizon
git clone https://github.com/horizon-rust/horizon-mixins ../horizon-mixins
```

2. Стабилизировать pinned версию:
```bash
python3 tools/horizon.py stage --horizon ../horizon
```

3. Build и test:
```bash
cargo fetch --locked --manifest-path target/mixins/modded/Cargo.toml
python3 tools/horizon.py check --horizon ../horizon
python3 tools/horizon.py test --horizon ../horizon  
python3 tools/horizon.py build --horizon ../horizon
```

### Для публикации на GitHub:

1. Commit исправления:
```bash
git add .
git commit -m "Fix gitignore and content mod structure"
git push origin main
```

2. В Horizon Launcher: Mods → Add from URL → вставь твой репозиторий

## ⚠️ Важные предупреждения:

### Что НЕМНОЖЕНО коммитить (в .gitignore):

- `target/` - Rust build artifacts
- `horizon*`, `client*`, `*.bin` - Binaries  
- `mixins/*/data/*.json` - Player resources (создаются лаунчером)
- `build.bat`, `setup.bat` - Скрипты сборки

### Если Horizon зависнет на compile:

```bash
# Запустить cargo fetch для modded workspace
cargo fetch --locked --manifest-path target/mixins/modded/Cargo.toml

# Или использовать локальный Horizon checkout
python3 tools/horizon.py build --horizon ../horizon
```

### Если content мод не загружается:

- Проверь что package.hnmod в правильном формате
- Убедись compatibility.horizon_revision совпадает с installed Horizon
- Content мод должен быть отдельным .hnmod, а не Rust binary

## 🔧 Структура проекта после fixes:

```
stellascrack-food/
├── Cargo.toml                    # workspace root lib only (no content bin)
├── compatibility.json            # pinned revisions
├── mixins/
│   ├── stellascrack_food/        # Rust runtime mod
│   │   ├── horizon-mixin.toml    # injection declarations
│   │   ├── break.rs              # helper adapter
│   │   └── runtime.rs            # main game logic
│   │
│   └── stellascrack_content/     # DATA-ONLY .hnmod mod ✅
│       ├── package.hnmod         # correct manifest for content
│       ├── data/
│       │   ├── items.json        # item definitions (JSON)
│       │   ├── recipes.json      # crafting recipes (JSON)
│       │   └── advancements.json # achievements (JSON)
│       └── lang/
│           └── en_us.json        # language files
├── tools/                        # staging/build scripts
├── .gitignore                    # prevents binaries, player data commit ✅
└── GIT_FIXES.md                  # this guide
```

## 📝 Summary:

**Основные проблемы решены:**
1. ✅ Content мод теперь правильный format (.hnmod с JSON)
2. ✅ Git ignores binaries и player resources  
3. ✅ Cargo.toml не пытается скомпилировать content как binary
4. ✅ Структура соответствует Horizon native mod规范

**Что осталось:**
- Test with real Horizon build (requires horizon-rust/horizon checkout)
- Push fixes to GitHub before publishing
