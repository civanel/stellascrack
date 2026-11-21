# 🚨 DELETE HORIZON-MIXIN.TOML FROM GITHUB!

## Проблема:
Файл `horizon-mixin.toml` в папке `mixins/stellascrack_content/` вызывает ошибку:
```
mixins\stellascrack_content: unsupported package path horizon-mixin.toml
```

## Решение:
Этот файл **ДЛЯ CONTENT-МОДА НЕ ПОТРЕБЕН** и даже ВРЕДИТ!

### Что делать на GitHub:

1. Зайди в репозиторий: https://github.com/civanel/stellascrack
2. Перейди в `mixins/stellascrack_content/`
3. **Удали файл `horizon-mixin.toml`** через Pull Request или Direct Edit → Delete File

### Почему удаляем?

- Content мод (stellascrack-content) - это data-only мод с JSON данными
- `horizon-mixin.toml` используется только для Rust runtime модов
- Лаунчер Horizon не может обработать этот файл в content моде

### После удаления:

Content мод должен работать с файлом `package.hnmod` который содержит:
```toml
[mod]
id = "stellascrack-content"
version = "0.1.0"
display_name = "Stella's Crack Food (Content)"

[injections]
"stellascrack:recipe_injector" = { path = "data/recipes.json", type = "inject" }
"stellascrack:creative_injector" = { path = "data/items.json", type = "inject" }
"stellascrack:advancement_injector" = { path = "data/advancements.json", type = "inject" }

[compatibility]
horizon_revision = "a9a30a3b6e3c246747748572e7b10c5fc6b9dc5e"
```

### JSON файлы должны быть:
- `data/recipes.json` - рецепты крафта
- `data/items.json` - предметы  
- `data/advancements.json` - достижения

Эти файлы **НЕ коммитьте** (они созданы лаунчером при установке мода)

---

## Быстрое решение через GitHub:

1. Открой файл [horizon-mixin.toml](file:///F%3A/stellascrack-food/DELETE_THIS_FILE.md#L2-L2) в браузере
   https://github.com/civanel/stellascrack/blob/main/mixins/stellascrack_content/horizon-mixin.toml

2. Кнопка **Delete File** → **Commit changes**

3. Пуши локально (если нужно):
```bash
cd F:\stellascrack-food
git rm mixins/stellascrack_content/horizon-mixin.toml
git add .
git commit -m "Remove unsupported horizon-mixin.toml from content mod"
git push origin main
```
