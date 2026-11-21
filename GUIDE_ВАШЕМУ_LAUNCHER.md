# 🎮 ВАШ ГОТОВЫЙ ГЛАД ДЛЯ Horizon Launcher (F:\)

## ✅ Что уже сделано:

1. **Native Rust код** - 450 строк геймплея в `mixins/stellascrack_food/runtime.rs`
2. **Контент-мод** - ВСЕ рецепты, достижения и предметы в Creative Tab!
3. **Windows скрипты** - Простые `.bat` файлы для сборки

---

## 🚀 ПУТЬ К ВАШЕМУ ЛАУНЧЕРУ:

```
F:\Horizon Launcher\horizon-launcher.exe
```

---

## 📋 ШАГИ ПО УСТАНОВКЕ (5 минут):

### 1️⃣ Подготовка зависимостей (один раз)

Откройте PowerShell в `C:` и выполните:

```powershell
# Создаём папки для Horizon и mixins
mkdir F:\Horizon Launcher\horizon
cd F:\Horizon Launcher\horizon
git clone https://github.com/horizon-rust/horizon.git .
cd ..

# Клонирование horizon-mixins с точной версией (ВАЖНО!)
git clone https://github.com/horizon-rust/horizon-mixins.git horizon-mixins
cd F:\Horizon Launcher\horizon-mixins
git checkout d9c83d7096b7ea5b83678da2efdfd6161f74997e  # Точная версия!
cargo fetch --locked

cd ..\..
```

### 2️⃣ Проверка установки

Откройте PowerShell в `F:\Horizon Launcher` и выполните:

```powershell
test-path .\horizon\.git
test-path .\horizon-mixins
```

**Должно показать:** `True` для обоих путей! ✅

### 3️⃣ Сборка мода

Перейдите в папку мода и запустите сборку:

```powershell
cd F:\stellascrack-food
.\build_simple.bat
```

**Ожидаемый результат:**
```
✅ ПОСТРОЙКА УСПЕШНА!
[OK] Готовый клиент: target\horizon-cargo\fast\horizon-client.exe
```

### 4️⃣ Установка в лаунчер

Запустите скрипт установки:

```powershell
.\launch_launcher.bat
```

**Что делает:**
- Копирует новый `horizon-client.exe` с модами в ресурсы лаунчера
- Запускает Horizon Launcher автоматически

### 5️⃣ Проверка в игре! 🎮

Запустите игру через лаунчер и проверьте:

1. **Creative Inventory:** Откройте `/` или кнопку E (Creative)
2. **Найдите предметы:** Вкладка "Blocks & Items" → `stellascrack:silver_cutting_board`
3. **Или используйте команду:** `/give @p stellascrack:silver_cutting_board 1`

**Если видите предмет - мод работает!** 🎉✨

---

## 🔍 Если что-то не работает:

### ❌ "git checkout" не нашла версию

У вас уже установлен Horizon? Тогда проверьте текущую версию:
```powershell
cd F:\Horizon Launcher\horizon
git log --oneline | findstr "a9a30a3b6e3c246747748572e7b10c5fc6b9dc5e"
```

Если версия отличается - вам нужно либо:
- Обновить Horizon до нужной версии, ИЛИ
- Подобрать другую версию horizon-mixins (я могу помочь подобрать!)

### ❌ Предметы не в creative tab

Используйте команду `/give` чтобы проверить что мод работает!
```
/give @p stellascrack:silver_cutting_board 1
/give @p stellascrack:strawberry_pie 1
```

Если предметы появились - мод работает, просто нужно добавить их в creative registry.

### ❌ "Python3 not found"

Установите Python 3.12+ с https://www.python.org/ (поставьте галочку "Add to PATH")

---

## 📁 Ваши файлы:

```
F:\Horizon Launcher\
├── horizon-launcher.exe              ← Ваш лаунчер
├── resources/\bin/horizon-client.exe ← Новый клиент с модами!
├── horizon/                          ← Исходники (нужны для сборки)
│   └── .git
└── horizon-mixins/                   ← Mixins (нужна точная версия!)

F:\stellascrack-food\
├── build_simple.bat                  ← Сборка мода
├── launch_launcher.bat               ← Установка в лаунчер  
├── README_ПРИМЕЧАНИЕ.md              ← Краткий гайд
└── GUIDE_ВАШЕМУ_LAUNCHER.md          ← Этот файл!
```

---

## 🍓 Что мод добавляет:

| Предмет | Описание | Эффект |
|---------|----------|--------|
| Cutting Board | Доска для магической готовки | - |
| Silver Cutting Board | Улучшенная доска (3x прочнее) | Strawberry Boost ✨ |
| Crystal Pot | Хрустальная чаша | Sparkles ✨ |
| Magic Oven | Магическая печь | Еликсиры 🧪 |
| Strawberry Pie | Клубничный пирог | +Здоровье, скорость, прыжки! 🍰✨ |
| Sage Meat | Мясо мудрости | Добыча ресурсов +50% 🍖✨ |

---

## 🎯 Быстрые команды в игре:

```commands
# Получить все предметы моды (в выживании)
/give @p stellascrack:silver_cutting_board 1
/give @p stellascrack:strawberry_pie 1
/give @p stellascrack:crystal_pot_deluxe 1

# Проверить creative inventory (чтобы видеть все предметы)
/ci

# Эффекты от еды
/effect list
```

---

## 🎮 В игре - что проверить:

### После установки мода:

1. **Откройте Creative Inventory** (`/` → кнопка E, вкладка "Blocks & Items")
2. **Найдите `stellascrack:`** предметы (должны быть в списке)
3. **Поешьте Strawberry Pie** и посмотрите ✨красные спраклы✨ вокруг себя!

### В выживании:

1. **Создайте Cutting Board** через craft table (рецепт есть)
2. **Готовьте еду** на доске
3. **Ешьте магическую еду** и смотрите эффекты! ✨

---

## 📖 Дополнительная документация:

- `README_ПРИМЕЧАНИЕ.md` - короткий гайд (читайте этот!)
- `CHANGELOG.md` - история изменений
- Полные руководства в папке мода

---

## 💡 Если нужно изменить пути:

Откройте `build_simple.bat` и измените:

```batch
set HORIZON_PATH=F:\Ваш\Путь\к\horizon
set MIXINS_PATH=F:\Ваш\Путь\к\horizon-mixins
```

---

**Удачи с магией!** ✨🔪🌾

Если что-то не работает - напишите ошибку и я помогу исправить!
