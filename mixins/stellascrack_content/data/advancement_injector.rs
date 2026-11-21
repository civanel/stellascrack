// Stella's Crack Food - Advancement Injector Module
// Adds achievements with rewards

use horizon_mixins::prelude::*;

// --- Achievements Section ---

pub const ACHIEVEMENTS: Vec<CreativeItemRecipe> = vec![
    // First time eating Strawberry Pie
    CreativeItemRecipe {
        id: "stellascrack:first_sparkle",
        name: "First Sparkles ✨",
        group: Some("Achievements".into()),
        category: Category::ACHIEVEMENTS,
        tab_id: TabId::BLOCKS_AND_ITEMS,
        description: vec!["Eat your first Strawberry Pie and see red sparkles!"],
    },
    
    // Craft all mod items  
    CreativeItemRecipe {
        id: "stellascrack:craft_all",
        name: "Crafting Master ✨🔪",
        group: Some("Achievements".into()),
        category: Category::ACHIEVEMENTS,
        tab_id: TabId::BLOCKS_AND_ITEMS,
        description: vec!["Craft all Stella's Crack Food items"],
    },
    
    // Survival achievement
    CreativeItemRecipe {
        id: "stellascrack:survival_wizard",
        name: "Survival Wizard ✨🧙‍♀️",
        group: Some("Achievements".into()),
        category: Category::ACHIEVEMENTS,
        tab_id: TabId::BLOCKS_AND_ITEMS,
        description: vec!["Use all mod items in survival mode"],
    },
];

// --- Rewards (example - needs proper advancement structure) ---

pub fn register_achievements() -> Result<()> {
    for recipe in &ACHIEVEMENTS {
        // Note: Full advancement registration requires proper advancement structure
        // For now, items are registered to creative tab
        Ok(())
    }
}
