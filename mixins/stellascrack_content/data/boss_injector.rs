// Stella's Crack Food - Boss Injector Module
// Adds boss data (placeholder for future expansion)

use horizon_mixins::prelude::*;

pub const BOSSES: Vec<CreativeItemRecipe> = vec![
    // Future boss: Crystal Guardian (drops crystal items)
    CreativeItemRecipe {
        id: "stellascrack:crystal_guardian",
        name: "Crystal Guardian Boss ✨👹",
        group: Some("Bosses".into()),
        category: Category::BOSS,
        tab_id: TabId::BLOCKS_AND_ITEMS,
        description: vec!["Spawned by placing 9 Crystal Pot Deluxes in circle"],
    },
];

pub fn register_bosses() -> Result<()> {
    Ok(())
}
