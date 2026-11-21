# Changelog for Stella's Crack Food Mod

## Version 1.0.0 - Initial Release (November 2026)

### New Features
- **Magical Cooking System** ✨
  - Strawberry Pie with red tulip sparkles and healing effects
  - Sage Meat with silver glow berry particles and mining speed
  - Crystal Soup Elixir with multi-effect magic
  
- **Biome Integration** 🌍
  - Strawberry Fields biome (50% food effect boost)
  - Sage Garden biome (cooking efficiency +50%)
  
- **Block Registry** 🔨
  - `stellascrack:cutting_board` - Basic crafting board
  - `stellascrack:silver_cutting_board` - Enhanced durability (3x)
  - `stellascrack:crystal_pot` - Special recipes container
  - `stellascrack:magic_oven` - Elixir and potion cooking
  - `stellascrack:strawberry_hedge` - Decorative hedge block
  - `stellascrack:sage_hedge` - Healing garden decoration

- **Custom Effects** ⚡
  - Strawberry Boost: Health regen, speed +20%, jump boost (10 min)
  - Sage Wisdom: Mining efficiency +50% in sage biomes (15 min)
  - Magic Energy Shield: Protection level I, mana regeneration (8 min)
  - Couch Potato Fatigue: Slowness and mining fatigue from over-cooking

- **Upgrade System** 📦
  - Silver Cutting Board upgrade with craft recipe
  - Deluxe Crystal Pot with larger capacity
  - All upgrades have sparkle effects on use

### Technical Implementation
- Native Horizon hooks using `block_display` and `break_block` named hooks
- Full Rust implementation in `mixins/stellascrack_food/runtime.rs`
- Content mod system for recipes via `stellascrack_content` module
- Complete advancement tree with rewards
- Cross-platform build support (Windows/Linux)

### Files Created
```
F:/stellascrack-food/
├── Cargo.toml                    # Workspace manifest
├── compatibility.json            # Pin configurations
├── README_WINDOWS.md             # Windows installation guide
├── CHANGELOG.md                  # This file
├── build.bat                     # Build script for Windows
├── run_horizon.bat               # Launch Horizon script
│
├── mixins/
│   ├── stellascrack_food/        # Native runtime hooks
│   │   ├── horizon-mixin.toml    # Hook declarations
│   │   └── runtime.rs            # Core game logic (BLOCKS)
│   │
│   └── stellascrack_content/     # Content mod data
│       ├── horizon-mixin.toml    # Content hooks
│       └── data/
│           ├── recipe_injector.rs  # Crafting recipes
│           ├── advancement_injector.rs  # Achievements  
│           ├── boss_injector.rs    # Bosses (placeholder)
│           └── lang/en_us.json     # Translations
│
└── tools/
    ├── horizon.py                # Build pipeline script
    └── audit/Cargo.toml          # AST comparison tool
```

### Usage Examples

**In-game creative mode:**
```
/give @p stellascrack:silver_cutting_board 1
/give @p stellascrack:strawberry_pie 1
```

**Crafting survival:**
- Open creative inventory to see all registered items
- Or use `/give` commands with `stellascrack:` namespace

### Build Commands (Windows)

```powershell
# Quick build
build.bat

# Full validation build
python3 tools\horizon.py check --horizon ..\..\Documents\horizon
python3 tools\horizon.py build --horizon ..\..\Documents\horizon
```

---

## Future Versions (Planned)

### v1.1.0 - Seasonal Events
- Holiday-themed recipes (Halloween, Christmas, etc.)
- Weather-based cooking quality bonuses
- Seasonal exclusive dishes

### v1.2.0 - Multi-tier Upgrades  
- Diamond and Netherite tier cutting boards
- Crystal variants for each block type
- Enchantment system for blocks

### v1.3.0 - Skill Progression
- Cooking skill tree with XP rewards
- Unlockable recipes based on skill level
- Master chef achievements

### v1.4.0 - Weather & Time System
- Rain improves soup quality
- Nighttime cooking bonuses
- Day/night cycle effects on food

---

## Known Issues / TODOs

- [ ] Add sound effects for cooking
- [ ] Implement boss enemies (placeholder in code)
- [ ] Create full 3D models (.glb files)
- [ ] Add more biome modifiers
- [ ] Implement enchantment system for blocks
- [ ] Create tutorial advancements for beginners

## License

MIT License - See LICENSE file
