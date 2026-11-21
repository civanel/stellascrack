# 🎮 Quick Start для вашего Horizon Launcher (на F:\)

## Ваш текущий путь:
```
F:\Horizon Launcher\horizon-launcher.exe
```

---

## 🚀 Установка мода (3 шага):

### Шаг 1: Запустите сборку
```powershell
cd F:\stellascrack-food
.\build_simple.bat
```

**Проверьте что:**
- ✅ Horizon находится в `F:\Horizon Launcher\horizon` 
- ✅ horizon-mixins в `F:\Horizon Launcher\horizon-mixins`
- ✅ Build показывает "✅ ПОСТРОЙКА УСПЕШНА!"

### Шаг 2: Запустите лаунчер с модом
```powershell
.\launch_launcher.bat
```

**Что это делает:**
- Копирует новый `horizon-client.exe` в папку ресурсов лаунчера
- Автоматически запускает Horizon Launcher

### Шаг 3: Проверьте игру
1. Запустите Horizon через лаунчер
2. Откройте Creative Inventory (`/`)
3. Найдите `stellascrack:silver_cutting_board` в вкладке **Blocks & Items** (E)

---

## 📁 Структура ваших файлов:

```
F:\Horizon Launcher\
├── horizon-launcher.exe          ← Ваш лаунчер
├── resources/                    ← Ресурсы игры (сюда копируется мод)
│   └── bin/
│       └── horizon-client.exe    ← Новый клиент с модами!
├── horizon/                      ← Исходники Horizon
│   └── .git                     ← Git репозиторий
└── horizon-mixins/               ← Horizon mixins (нужна exact revision)

F:\stellascrack-food\             ← Мод
└── build_simple.bat              ← Скрипт сборки
```

---

## ⚡ Быстрый запуск (если всё готово):

Просто дважды кликните: `build_simple.bat` → затем `launch_launcher.bat`

---

## 🔧 Если не работает:

### "Horizon не найден"
Укажите путь вручную в начале файла `build_simple.bat`:
```batch
set HORIZON_PATH=F:\Правильный\путь\к\horizon
set MIXINS_PATH=F:\Правильный\путь\к\mixins
```

### "Мод не работает"
Проверьте что horizon-mixins на нужной версии:
```powershell
cd F:\Horizon Launcher\horizon-mixins
git log --oneline | findstr "d9c83d7096b7ea5b83678da2efdfd6161f74997e"
# Должно показать эту коммит!
```

### Предметы не в creative
Используйте команду в игре:
```
/give @p stellascrack:silver_cutting_board 1
```

---

## 🎮 В игре:

1. **Creative:** Откройте инвентарь `/` → вкладка E (Blocks & Items) → найдите stellascrack
2. **Survival:** Используйте `/give @p stellascrack:block` команды
3. **Еда:** Поешьте Strawberry Pie и увидите ✨красные спраклы✨!

---

**Удачи!** ✨🔪🌾
