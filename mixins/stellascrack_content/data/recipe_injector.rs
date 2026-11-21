// Stella's Crack Food - Recipe Injector Module
// Adds all crafting recipes for the mod items

use horizon_mixins::prelude::*;

// --- Silver Cutting Board Recipes ---

pub const CUTTING_BOARD_RECIPE: CraftingRecipe = CraftingRecipe {
    id: "stellascrack:silver_cutting_board",
    name: "Silver Magic Cutting Board",
    group: Some("Blocks & Items".into()),
    input_slots: vec![
        CraftInputSlot::new(0, "minecraft:oak_planks"),
        CraftInputSlot::new(1, "minecraft:silver_block"), // Silver block if available
        CraftInputSlot::new(2, "minecraft:oak_planks"),
        CraftInputSlot::new(3, "minecraft:glass_pane"),
    ],
    output_slot: OutputSlot { item_id: "stellascrack:cutter_board", count: 1 },
};

// --- Crystal Pot Deluxe Recipes ---

pub const CRYSTAL_POT_RECIPE: CraftingRecipe = CraftingRecipe {
    id: "stellascrack:crystal_pot_deluxe",
    name: "Crystal Pot Deluxe",
    group: Some("Blocks & Items".into()),
    input_slots: vec![
        CraftInputSlot::new(0, "minecraft:glass"),
        CraftInputSlot::new(1, "minecraft:quartz_block"),
        CraftInputSlot::new(2, "minecraft:amethyst_shard"),
        CraftInputSlot::new(3, "minecraft:diamond_block"),
    ],
    output_slot: OutputSlot { item_id: "stellascrack:crystal_pot_deluxe", count: 1 },
};

// --- Magic Oven Recipes ---

pub const MAGIC_OVEN_RECIPE: CraftingRecipe = CraftingRecipe {
    id: "stellascrack:magic_oven",
    name: "Magic Elixir Oven",
    group: Some("Blocks & Items".into()),
    input_slots: vec![
        CraftInputSlot::new(0, "minecraft:cobblestone"),
        CraftInputSlot::new(1, "minecraft:lapis_lazuli"),
        CraftInputSlot::new(2, "minecraft:redstone"),
        CraftInputSlot::new(3, "minecraft:end_rod"),
    ],
    output_slot: OutputSlot { item_id: "stellascrack:magic_oven", count: 1 },
};

// --- Strawberry Pie Recipe ---

pub const STRAWBERRY_PIE_RECIPE: CraftingRecipe = CraftingRecipe {
    id: "stellascrack:strawberry_pie",
    name: "Magical Strawberry Pie ✨",
    group: Some("Blocks & Items".into()),
    input_slots: vec![
        CraftInputSlot::new(0, "minecraft:pumpkin"), // Pumpkin pie base
        CraftInputSlot::new(1, "minecraft:strawberry"),
        CraftInputSlot::new(2, "minecraft:sugar"),
        CraftInputSlot::new(3, "minecraft:glow_berries"),
    ],
    output_slot: OutputSlot { item_id: "stellascrack:strawberry_pie", count: 1 },
};

// --- Sage Wisdom Meat Recipe ---

pub const SAGE_MEAT_RECIPE: CraftingRecipe = CraftingRecipe {
    id: "stellascrack:sage_meat",
    name: "Sage Wisdom Meat ✨",
    group: Some("Blocks & Items".into()),
    input_slots: vec![
        CraftInputSlot::new(0, "minecraft:porkchop"),
        CraftInputSlot::new(1, "minecraft:sage"), // Custom herb item
        CraftInputSlot::new(2, "minecraft:red_berry"),
        CraftInputSlot::new(3, "minecraft:glow_berries"),
    ],
    output_slot: OutputSlot { item_id: "stellascrack:sage_meat", count: 1 },
};

// --- Crystal Healing Soup Recipe ---

pub const CRYSTAL_SOUP_RECIPE: CraftingRecipe = CraftingRecipe {
    id: "stellascrack:crystal_soup",
    name: "Crystal Healing Soup ✨",
    group: Some("Blocks & Items".into()),
    input_slots: vec![
        CraftInputSlot::new(0, "minecraft:bowl"), // Or mushroom stew
        CraftInputSlot::new(1, "minecraft:water_bucket"),
        CraftInputSlot::new(2, "minecraft:sage"),
        CraftInputSlot::new(3, "minecraft:end_crystal"),
    ],
    output_slot: OutputSlot { item_id: "stellascrack:crystal_soup", count: 1 },
};

// --- Silver Upgrade Recipe ---

pub const SILVER_UPGRADE_RECIPE: CraftingRecipe = CraftingRecipe {
    id: "stellascrack:silver_upgrade",
    name: "Silver Board Upgrade ✨",
    group: Some("Blocks & Items".into()),
    input_slots: vec![
        CraftInputSlot::new(0, "stellascrack:cutter_board"),
        CraftInputSlot::new(1, "minecraft:iron_ingot"),
        CraftInputSlot::new(2, "minecraft:diamond"),
        CraftInputSlot::new(3, "minecraft:silver_ingot"), // Custom item
    ],
    output_slot: OutputSlot { item_id: "stellascrack:silver_upgrade", count: 1 },
};

// Register all recipes
pub fn register_recipes() -> Result<()> {
    CUTTING_BOARD_RECIPE.register()?;
    CRYSTAL_POT_RECIPE.register()?;
    MAGIC_OVEN_RECIPE.register()?;
    STRAWBERRY_PIE_RECIPE.register()?;
    SAGE_MEAT_RECIPE.register()?;
    CRYSTAL_SOUP_RECIPE.register()?;
    SILVER_UPGRADE_RECIPE.register()?;
    
    Ok(())
}
