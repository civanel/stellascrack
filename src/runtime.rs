// Food-specific damage calculation
pub fn food_damage_item(entity: &Entity) -> i32 {
    // Food causes minimal damage (1-2 hearts) when eaten
    1 + (entity.get::<Item>().unwrap().count() as i32 / 64)
}

// Hunger restoration based on saturation
pub fn restore_hunger(saturation: f32, max_hunger: f32) -> f32 {
    // Convert saturation to hunger points (0-10 range in Minecraft)
    std::cmp::min(saturation * 5.0, max_hunger)
}

// Eating sound effects - returns the appropriate sound based on food type
pub fn eating_sound(food_type: &FoodType) -> Option<String> {
    match food_type {
        FoodType::Apple => Some("BLOCK_NOTE_BLOCK_HARP".to_string()),
        FoodType::BakedPotato => Some("ITEM_BONE_MEAL_EAT".to_string()),
        FoodType::CookedBeef => Some("item.generic.eat_1".to_string()),
        FoodType::CookedChicken => Some("item.generic.eat_2".to_string()),
        FoodType::PumpkinPie => Some("entity.player.attack_knockback".to_string()),
        _ => None,
    }
}
