# 🍓 Stella's Crack Food - Гайд на русском

## Что это за мод?

Магический фарминг-мод для Horizon Minecraft с:
- ✨ **Спраклами и эффектами** при поедании еды
- 🌍 **Интеграцией с биомами** (клубничные поля, садовые грядки)  
- 🔪 **Улучшениями блоков** (серебряная доска, хрустальная кастрюля)
- 🎮 **Геймплеем как Farmers Delight + Hands-On Delight**

## 📦 Установите на Windows

### Шаг 1: Подготовьте зависимости

```powershell
# Создайте папки для Horizon и horizon-mixins
mkdir $env:USERPROFILE\Documents\horizon
cd $env:USERPROFILE\Documents\horizon
git clone https://github.com/horizon-rust/horizon.git .
..\.git config core.autocrlf input

# Подготовьте horizon-mixins (ВАЖНО! Версия)
cd ..
git clone https://github.com/horizon-rust/horizon-mixins.git horizon-mixins
cd horizon-mixins
git checkout d9c83d7096b7ea5b83678da2efdfd6161f74997e  # Точная версия!
cargo fetch --locked

cd ..\..
```

### Шаг 2: Соберите мод

```powershell
# Перейдите в папку мода
cd F:\stellascrack-food

# Проверьте, что компилируется (важно!)
python3 tools\horizon.py check --horizon C:\Users\Stella\Documents\horizon
python3 tools\horizon.py check --horizon C:\Users\Stella\Documents\horizon
F:\Horizon Launcher\horizon-launcher.exe

# Полная сборка с оптимизацией
python3 tools\horizon.py build --horizon C:\Users\Stella\Documents\horizon
```

**Или просто используйте скрипт:**
```powershell
build.bat
```

### Шаг 3: Запустите Horizon с модами

#### Вариант А: Через run_horizon.bat (простой)

Откройте PowerShell в папке `F:\stellascrack-food` и выполните:
```powershell
.\run_horizon.bat
```

Скрипт сам подставит путь к вашему Horizon и запустит игру.

#### Вариант Б: Ручной запуск

1. Откройте Horizon Launcher или укажите путь
2. Путь к клиенту: `C:\Users\Stella\AppData\Roaming\Horizon\instances\StellaScrack\resources\bin\horizon-client.exe`
3. Добавьте аргументы: `--mods C:\Users\Stella\AppData\Roaming\Horizon\instances\StellaScrack\mods\`

## 🎮 Как проверить что мод работает?

### В творческом режиме (Creative Mode):

1. Откройте инвентарь (`/` или кнопка)
2. Перейдите в вкладку **Blocks & Items** (кнопка E)
3. Найдите предметы с префиксом `stellascrack:`:
   - `Cutting Board` 📋
   - `Silver Cutting Board` ✨ 
   - `Crystal Cooking Pot` 💎
   - `Magic Oven` 🔥
   - `Magical Strawberry Pie` 🍰 (EPIC!)
   - `Wisdom Sage Meat` 🍖

### Если не видите в creative:

Используйте команду `/give`:
```commands
/give @p stellascrack:silver_cutting_board 1
/give @p stellascrack:strawberry_pie 1  
/give @p stellascrack:crystal_pot_deluxe 1
```

**Если предмет появился - мод работает!** ✅

### В выживании (Survival Mode):

1. **Сначала создайте Cutting Board:**
   - Откройте craft table
   - Рецепт: 3 дубовых пня + красная пыль
   - Получите `stellascrack:cutting_board`

2. **Поставьте блок и готовьте еду** (рецепты в README)

3. **Съешьте Strawberry Pie:**
   - Должны появиться ✨красные спраклы✨ вокруг вас
   - Здоровье восстанавливается
   - Скорость бега +20%

## 🍽️ Рецепты еды

### Strawberry Pie (Клубничный пирог)
**Крафт на Cutting Board:**
```
B P B  ← Верх:草莓(клубника)
S S S  ← Средний слой клубники  
B C B  ← Низ с кремом в центре
```

**Эффекты:**
- ✨ Красные тюльпаны (sparkles)
- +3 здоровья за тик
- +20% скорость бега и прыжков
- 10 минут длится эффект

### Sage Meat (Мясо мудрости)
**Крафт на Cutting Board с мечом медь:**
```
S M S  ← Sage вокруг мяса
M T M  ← Сверху torch для приготовления  
S M S  ← Еще sage снизу
```

**Эффекты:**
- ✨ Серебряные светящиеся ягоды (sparkles)
- +50% к добыче ресурсов в Sage Garden биома
- 15 минут длится эффект

### Crystal Soup (Хрустальный суп)
**Крафт в Crystal Pot с магической палочкой:**
```
C B C  ← Хрустальная чаша с жидкостью
C W C  ← С ингредиентами  
C M C  ← Магические кристаллы сверху
```

**Эффекты:**
- ✨ Многоцветные спраклы (glow_berries, end_crystals)
- Защита I + регенерация маны
- Только в хрустальной чаше!

## 🌍 Биомы и бонусы

### Strawberry Fields (Клубничное поле)
**Что дает:**
- Еда работает лучше на 50%! 
- Все эффекты усилены
- Спраклы видны дальше

### Sage Garden (Садовая грядка)  
**Что дает:**
- Эффективность варки +50%
- Sage Wisdom работает дольше
- Лучше для приготовления sage мяса

## 🛠️ Улучшения (Upgrades)

### Silver Cutting Board (Серебряная доска)
**Крафт в craft table с алмазным топором:**
```
 D       ← Diamond (алмаз)
SSS      ← 4x Silver Ingot (серебро)
SSS      
 NNN     ← Netherite Scrap (незерит)
 C C     ← 2x Magic Crystal
```

**Бонусы:**
- В 3 раза прочнее обычной доски
- Готовка на +50% быстрее
- При использовании - strawberry boost эффект!

### Deluxe Crystal Pot (Премиум хрустальная чаша)
**Крафт в craft table с алмазным мачтом:**
```
 D       ← Diamond
CCC      ← 4x Magic Crystal
LLL      ← 6x Lapis Lazuli
```

**Бонусы:**
- Вместимость жидкости +32 (как у cauldron)
- Дает magic energy эффект при использовании
- Лучшие рецепты только в этой чаше!

## 🎯 Как добавить больше контента?

### В runtime.rs (native hooks):
```rust
// Добавьте новые эффекты:
pub fn effects(effects: &mut Vec<Effect>) {
    effects.push(Effect {
        id: "stellascrack:YOUR_EFFECT".to_string(),
        description: Some("Ваше описание".to_string()),
        particles: vec![
            (horizon_mods::world::BlockId::minecraft("block_id"), 20),
        ],
        healing: Some(3.0),
        speed_multiplier: Some(1.5),
    });
}

// Добавьте новые блоки:
pub fn blocks(blocks: &mut Vec<Block>) {
    blocks.push(Block {
        id: "stellascrack:new_block".to_string(),
        template: "minecraft:stonecutter".to_string(),
        item_form: true,  // Можно подобрать как предмет
        ..Default::default()
    });
}
```

### В recipe_injector.rs (рецепты):
```rust
pub fn recipes(recipes: &mut Vec<Recipe>) {
    recipes.push(Recipe {
        id: "stellascrack:new_item".to_string(),
        type_: RecipeType::crafting_table,
        pattern: vec![
            "###".to_string(),  // Row 1
            "# #".to_string(),  // Row 2  
            "# #".to_string(),  // Row 3
        ],
        result_item: Item {
            id: "stellascrack:new_item".to_string(),
            count: 64,
        },
        required_tools: None,
    });
}
```

## 🔧 Решение проблем

### Ошибка: "horizon-mixins не найден"
```powershell
# Укажите путь к horizon-mixins через переменную окружения
$env:HORIZON_MIXINS="C:\Users\Stella\Documents\horizon-mixins"
python3 tools\horizon.py build --horizon C:\Users\Stella\Documents\horizon
```

### Ошибка: "git archive exited 128"  
Проверьте что Horizon git checkout на нужной версии:
```powershell
git -C C:\Users\Stella\Documents\horizon log --oneline | findstr "a9a30a3b6e3c246747748572e7b10c5fc6b9dc5e"
```

### Предметы не в creative tab:
Это должно работать автоматически через `creative_registry` hook. Если нет - проверьте компиляцию:
```powershell
python3 tools\horizon.py check --horizon C:\Users\Stella\Documents\horizon
# Проверьте лог в target/horizon-check.log
```

## 📚 Полный список файлов мода

```
F:/stellascrack-food/
├── Cargo.toml                    # Манифест workspace
├── compatibility.json            # Версии и хэши для проверки
├── README_WINDOWS.md             # Английский гайд
├── README_РУССКИЙ.md             # Этот файл! 🇷🇺
├── build.bat                     # Скрипт сборки (просто запускайте!)
├── run_horizon.bat               # Запуск Horizon с модами
│
├── mixins/
│   ├── stellascrack_food/        # NATIVE Rust код
│   │   ├── horizon-mixin.toml    # 3 hook объявления
│   │   └── runtime.rs            # ВСЯ логика (450 строк!)
│   │
│   └── stellascrack_content/     # КОНТЕНТ-мод (.hnmod)
│       ├── horizon-mixin.toml    # 4 injection объявления
│       └── data/                 # Данные для моды
│           ├── recipe_injector.rs  # 16 рецептов крафта
│           ├── advancement_injector.rs  # 8 достижений
│           ├── boss_injector.rs    # Боссы (заглушка)
│           ├── creative_injector.rs  # ВСЕ предметы в creative!
│           └── lang/en_us.json     # Переводы предметов/эффектов
│
└── tools/
    ├── horizon.py                # Python скрипт сборки
    └── audit/Cargo.toml          # Инструмент для проверки кода
```

## 🚀 Что дальше?

Хотите добавить:
- **Сезонные события** (Рождество, Хэллоуин) → добавьте в `recipe_injector.rs` условия по времени года
- **Погоду** (дождь улучшает суп) → модифицируйте логику в `runtime.rs` функция `cooking_effect()`  
- **Уровни мастерства** (skill progression) → создайте новый эффект с прогрессией уровней

## 📖 Дополнительная документация

- [README для англоязычных пользователей](README_WINDOWS.md)
- [Changelog всех изменений](CHANGELOG.md)
- [Полный отчет о файлах](COMPLETE_SUMMARY.md)

---

**Счастливого крафта!** ✨🔪🌾

Если что-то не работает - проверьте `target/horizon-build.log` для подробностей ошибок!
