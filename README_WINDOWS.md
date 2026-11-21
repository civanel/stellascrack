# Stella's Crack Food - Horizon Mod (Windows Version)

A magical farming mod for Horizon Minecraft with biome-based cooking, custom effects, sparkles, and upgrades. Inspired by Farmers Delight and Hands-On Delight!

## 🎮 Installation on Windows

### Step 1: Prerequisites
- **Horizon Minecraft** installed and running
- **Rust 1.95.0** or newer
- **Python 3.12+** 
- Git for Windows (optional, for Horizon cloning)

### Step 2: Get Horizon and Horizon-Mixins Sources

You need the official Horizon sources to build native mods:

```powershell
# Clone Horizon (if not already done)
cd $env:USERPROFILE
git clone https://github.com/horizon-rust/horizon.git horizon
cd ..

# Clone horizon-mixins and checkout exact revision
git clone https://github.com/horizon-rust/horizon-mixins.git horizon-mixins
cd horizon-mixins
git checkout d9c83d7096b7ea5b83678da2efdfd6161f74997e  # Exact revision
cargo fetch --locked
```

### Step 3: Build the Mod

Open PowerShell in this directory and run:

```powershell
cd F:\stellascrack-food

# Fetch dependencies (first time only)
cargo fetch --locked

# Validate your mod compiles correctly
python3 tools\horizon.py check --horizon ..\..\Users\Stella\Documents\horizon

# Build the Horizon client with your mod
python3 tools\horizon.py build --horizon ..\..\Users\Stella\Documents\horizon
```

This will create: `F:\stellascrack-food\target\horizon-cargo\fast\horizon-client.exe`

### Step 4: Prepare Resources and Mods Folder

For Horizon, you need to prepare resources:

```powershell
# Navigate to your Horizon installation directory
cd $env:APPDATA\Horizon\instances\StellaScrack

# Create mods folder if it doesn't exist
if (-not (Test-Path "mods")) { New-Item -ItemType Directory -Path "mods" }

# Copy the built client to replace the existing one
Copy-Item ..\target\horizon-cargo\fast\horizon-client.exe .\resources\bin\
```

**Note**: Horizon expects resources in a specific location. You may need to:
1. Use `horizon-prepare` tool if available, OR
2. Manually create the structure at `$env:APPDATA\Horizon\instances\StellaScrack\resources\`

### Step 5: Launch Horizon with Your Mod

```powershell
# Run Horizon from your mods folder
cd $env:APPDATA\Horizon\instances\StellaScrack

.\resources\bin\horizon-client.exe --mods .\mods\
```

**OR** use the Horizon Launcher GUI:
1. Open Horizon Launcher
2. Select instance "StellaScrack"  
3. Go to Mods tab → Add mod from file
4. Browse and select your built `.hnmod` or run the client directly

### Step 6: In-Game Verification

Once in game:

1. **Creative Inventory Check:**
   - Open creative inventory (`/`) 
   - Look for `stellascrack` tab/category
   - You should see: cutting_board, crystal_pot, magic_oven, etc.
   - If you don't see it there, check the `/give` command below

2. **Using Creative Mode:**
   ```
   /give @p stellascrack:silver_cutting_board 1
   /give @p stellascrack:strawberry_pie 1
   /give @p stellascrack:crystal_pot_deluxe 1
   ```

3. **Survival Mode Testing:**
   - Craft the cutting board using the recipe
   - Place it to cook magical foods
   - Eat food and watch for sparkles! ✨

## 🎯 Key Features

### ✨ Sparkle Effects
- Strawberry Pie → Red tulip sparkles + health regeneration
- Sage Meat → Silver glow berry sparkles + mining speed
- Crystal Soup → Multi-color magic sparkles + mana regen

### 🌍 Biome Integration  
- **Strawberry Fields**: Food effects boosted by 50%
- **Sage Garden**: Cooking efficiency increased, sage wisdom effect active

### ⚡ Upgrades System
- Silver Cutting Board (3x durability)
- Deluxe Crystal Pot (larger capacity)
- Magic Oven (elixir cooking)

## 📦 Recipe Overview

| Item | Crafting Method | Special Requirements |
|------|----------------|---------------------|
| Cutting Board | Crafting Table | Oak logs + Redstone |
| Silver Cutting Board | Craft Table | Diamond, Netherite, Crystals |
| Crystal Pot | Craft Table | Glass + End Crystals |
| Magic Oven | Craft Table | Nether Bricks + Crystals |

## 🍽️ Cooking Recipes

### Strawberry Pie (on Cutting Board)
- Requires: Strawberries, Cream, Bottom layer
- Effects: Strawberry Boost (10 min)
- Sparkles: Red tulip particles ✨

### Sage Meat (on Cutting Board)  
- Requires: Raw meat, Sage leaves, Torch for cooking
- Effects: Sage Wisdom (15 min)
- Sparkles: Silver glow berries ✨

## 🔧 Troubleshooting

### Error: "horizon-mixins not found"
```powershell
# Set the environment variable
$env:HORIZON_MIXINS="C:\path\to\horizon-mixins"
python3 tools\horizon.py build --horizon C:\path\to\horizon
```

### Error: "cargo fetch failed"
- Make sure you're connected to the internet for first run
- Or use `--offline` flag after initial fetch

### Creative Inventory Tab Not Showing
- The items are registered as creative-only in survival
- Use `/give @p stellascrack:block` commands for testing
- Check if the mod actually compiled: look at `target\horizon-cargo\fast\horizon-client.exe`

### Mod Doesn't Appear in Launcher
1. Make sure you built with the correct Horizon version
2. The `.hnmod` file should be in `$env:APPDATA\Horizon\instances\StellaScrack\mods\`
3. Launch Horizon from that same mods folder

## 📝 Development Notes

This mod uses **native Horizon hooks**:
- `block_display` - Register new blocks/items
- `break_block` - Intercept block interactions  
- Custom effects system with particle sparkles

The runtime code is compiled directly into Horizon, ensuring:
- Maximum performance (no mod loader overhead)
- Full integration with biome logic
- Direct access to game internals for cooking mechanics

## 🚀 Future Ideas

Want more? Consider adding:
- Seasonal events (holidays affect cooking)
- Weather-based recipes (rain = better soup quality)
- Multi-tier upgrade system
- Cooking skill progression
- Biome-specific exclusive dishes

---

**Happy Crafting!** ✨🔪🌾
