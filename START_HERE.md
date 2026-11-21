# 🎮 START HERE - Stella's Crack Food Mod

## ⚡ Быстрый старт (только Windows)

### 1️⃣ Сначала убедитесь что у вас есть Horizon Minecraft:
```powershell
# Проверьте, установлен ли Horizon:
test-path $env:APPDATA\Horizon\instances\StellaScrack

# Если нет - скачайте и установите Horizon с https://horizon.minecraft/
```

### 2️⃣ Подготовьте зависимости (один раз):
```powershell
mkdir $env:USERPROFILE\Documents\horizon
cd $env:USERPROFILE\Documents\horizon
git clone https://github.com/horizon-rust/horizon.git .
cd ..

# Создаём папку для horizon-mixins
git clone https://github.com/horizon-rust/horizon-mixins.git horizon-mixins

# ВАЖНО! Заходим в exact revision (иначе мод не скомпилируется!)
cd horizon-mixins
git checkout d9c83d7096b7ea5b83678da2efdfd6161f74997e
cargo fetch --locked

cd ..\..
```

### 3️⃣ Соберите мод:
```powershell
cd F:\stellascrack-food
build.bat
```

**Если видите "BUILD SUCCESSFUL" - всё работает!** ✅

### 4️⃣ Запустите Horizon с модами:
```powershell
run_horizon.bat
```

## 🎯 Проверка работы (в игре):

### В Creative Mode:
1. Откройте инвентарь `/` или кнопкой E
2. Перейдите в **Blocks & Items**
3. Найдите `stellascrack:silver_cutting_board` и другие предметы

### Если не видите - используйте команду:
```
/give @p stellascrack:silver_cutting_board 1
```

**Если предмет появился - мод работает!** 🎉

## 📖 Полные руководства:

- **🇷🇺 На русском**: [README_РУССКИЙ.md](README_РУССКИЙ.md) ← ЧИТАТЬ ЭТО!
- **🇬🇧 English**: [README_WINDOWS.md](README_WINDOWS.md)  
- **📝 Changelog**: [CHANGELOG.md](CHANGELOG.md)

## 🔧 Решение проблем:

### "Python3 not found" → Установите Python 3.12+ с https://www.python.org/
### "horizon-mixins not found" → Перейдите в `C:\Users\Stella\Documents\horizon-mixins` и проверьте что там есть `.git` папка  
### Предметы не в creative → Проверьте лог: `F:\stellascrack-food\target\horizon-build.log`

## 🎮 Что мод добавляет:

| Категория | Предметов | Эффектов | Улучшений |
|-----------|----------|---------|----------|
| **Блоки** | 6 (cutting_board, crystal_pot и т.д.) | - | 2 уровня |
| **Еда** | 3 (pie, meat, soup) | 4 типа | - |
| **Биомы** | 2 (strawberry_fields, sage_garden) | +50% бонусы | - |

## ✨ Основные фичи:

1. **Спраклы при поедании** - красные тюльпаны, серебряные ягоды и т.д.
2. **Биом-бонусы** - клубничное поле усиливает еду на 50%
3. **Магические эффекты** - лечение, скорость, защита
4. **Улучшения блоков** - серебряная доска в 3 раза прочнее

## 📦 Где посмотреть код:

```
F:/stellascrack-food/
├── mixins/stellascrack_food/runtime.rs    # NATIVE Rust код (450 строк)
└── mixins/stellascrack_content/data/      # Данные (.hnmod)
    ├── recipe_injector.rs  # ВСЕ рецепты крафта
    ├── creative_injector.rs  # ВСЕ предметы в creative tab!
    └── advancement_injector.rs  # Достижения с наградами
```

---

**Наслаждайтесь магией!** ✨🍓🔪
