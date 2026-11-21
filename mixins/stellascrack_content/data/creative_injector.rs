// Stella's Crack Food - Creative Tab Injector Module
// Adds ALL mod items to Creative Inventory

use horizon_mixins::prelude::*;

// --- Blocks Section ---

pub const CREATIVERECIPES_BLOCKS: Vec<CreativeItemRecipe> = vec![
    // Cutting Board variants
    CreativeItemRecipe {
        id: "stellascrack:cutter_board",
        name: "Magic Cutting Board",
        group: Some("Blocks & Items".into()),
        category: Category::BLOCKS,
    },
    
    CreativeItemRecipe {
        id: "stellascrack:silver_cutting_board",
        name: "Silver Magic Cutting Board ✨",
        group: Some("Blocks & Items".into()),
        category: Category::BLOCKS,
        tab_id: TabId::BLOCKS_AND_ITEMS,
    },
    
    // Crystal Pot variants  
    CreativeItemRecipe {
        id: "stellascrack:crystal_pot_deluxe",
        name: "Crystal Pot Deluxe ✨",
        group: Some("Blocks & Items".into()),
        category: Category::BLOCKS,
        tab_id: TabId::BLOCKS_AND_ITEMS,
    },
    
    // Magic Oven
    CreativeItemRecipe {
        id: "stellascrack:magic_oven",
        name: "Magic Elixir Oven ✨",
        group: Some("Blocks & Items".into()),
        category: Category::BLOCKS,
        tab_id: TabId::BLOCKS_AND_ITEMS,
    },
];

// --- Food Section ---

pub const CREATIVERECIPES_FOOD: Vec<CreativeItemRecipe> = vec![
    // Strawberry Pie with sparkle effect
    CreativeItemRecipe {
        id: "stellascrack:strawberry_pie",
        name: "Magical Strawberry Pie ✨🍰",
        group: Some("Blocks & Items".into()),
        category: Category::FOODS,
        tab_id: TabId::BLOCKS_AND_ITEMS,
        description: vec!["Restores 3 hunger points (+25% saturation)", "Effects: Speed III + Jump Boost II for 3 minutes", "Spawns red sparkles ✨ when eaten"],
    },
    
    // Sage Wisdom Meat with luck boost
    CreativeItemRecipe {
        id: "stellascrack:sage_meat",
        name: "Sage Wisdom Meat ✨🍖",
        group: Some("Blocks & Items".into()),
        category: Category::FOODS,
        tab_id: TabId::BLOCKS_AND_ITEMS,
        description: vec!["Restores 4 hunger points (+20% saturation)", "Effects: Luck II + Haste I for 3 minutes", "Mining speed +50%"],
    },
    
    // Crystal Healing Soup with regeneration
    CreativeItemRecipe {
        id: "stellascrack:crystal_soup",
        name: "Crystal Healing Soup ✨🍲",
        group: Some("Blocks & Items".into()),
        category: Category::FOODS,
        tab_id: TabId::BLOCKS_AND_ITEMS,
        description: vec!["Restores 6 hunger points (max saturation)", "Effects: Regeneration + Strength for 3 minutes"],
    },
    
    // Silver Upgrade item
    CreativeItemRecipe {
        id: "stellascrack:silver_upgrade",
        name: "Silver Board Upgrade ✨",
        group: Some("Blocks & Items".into()),
        category: Category::MISC,
        tab_id: TabId::BLOCKS_AND_ITEMS,
        description: vec!["Upgrades cutter_board with sparkle effect"],
    },
];

// Register all creative items
pub fn register_creative_items() -> Result<()> {
    // Blocks tab
    for recipe in &CREATIVERECIPES_BLOCKS {
        recipe.register()?;
    }
    
    // Food tab  
    for recipe in &CREATIVERECIPES_FOOD {
        recipe.register()?;
    }
    
    Ok(())
}
