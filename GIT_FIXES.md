# Git Fixes for stellascrack-mod

## Что было исправлено:

### 1. `.gitignore` создан
- target/ (Rust build artifacts) - **никогда не коммить**
- Binaries (*.exe, *.dll) - **никогда не коммить**
- Player resources data/* - **никогда не коммить**

### 2. `Cargo.toml` исправлен
- Убран `[[bin]]` для stellascrack-content
- Content мод теперь отдельный `.hnmod` файл, а не Rust binary

### 3. Structure of `stellascrack_content/` fixed:
- ✅ package.hnmod - правильный манифест для контент-мода
- ✅ data/items.json - предметы в формате JSON
- ✅ data/recipes.json - рецепты в формате JSON  
- ✅ data/advancements.json - достижения в формате JSON
- ❌ Удалены *_injector.rs файлы (не нужны)

## Что нужно сделать перед пушем:

```bash
# Добавить исправления
git add .gitignore Cargo.toml mixins/stellascrack_content/package.hnmod
git add mixins/stellascrack_content/data/items.json
git add mixins/stellascrack_content/data/recipes.json  
git add mixins/stellascrack_content/data/advancements.json

# Удалить лишние файлы (не коммить)
git rm --cached build.bat setup.bat
```

## Основные проблемы решены:

1. ✅ Content мод теперь правильный data-only формат (.hnmod)
2. ✅ Нет Rust кода в контент-моде (только JSON данные)
3. ✅ .gitignore защищает от коммита binaries и player resources
4. ✅ Cargo.toml не пытается скомпилировать content как binary

## Как пушить:

```bash
git add .
git commit -m "Fix structure and gitignore"
git push origin main
```

## Проверка перед отправкой на GitHub:

1. `cargo fmt --all`
2. `cargo clippy --workspace --all-targets --locked --offline -- -D warnings`  
3. `cargo test --workspace --locked --offline`
4. `python3 tools/test_template.py`

## Для установки мода в Horizon Launcher:

1. Clone репозиторий на GitHub
2. В лаунчере Horizon -> Mods -> Add from URL
3. Вставь URL твоего GitHub репозитория

## Важно!

- Не коммить target/, binaries, player resources
- Content мод (stellascrack-content) и food мод (stellascrack-food) должны быть **одной версии Horizon**
- Все файлы в data/ папках НЕ должны коммититься - они создаются лаунчером при установке
