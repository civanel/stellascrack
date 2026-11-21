use horizon_client::game::{Game, Hand};
use horizon_mods::content::{Block, Effect, Item, Upgrade};

/// Регистрируем основные блоки моды
pub fn blocks(blocks: &mut Vec<Block>) {
    // Cutting Board - основной блок для приготовления
    blocks.push(Block {
        id: "stellascrack:cutting_board".to_string(),
        template: "minecraft:stonecutter".to_string(),
        item_form: true,  // Можно подобрать как предмет
        ..Default::default()
    });

    // Crystal Pot - магическая кастрюля для специальных рецептов
    blocks.push(Block {
        id: "stellascrack:crystal_pot".to_string(),
        template: "minecraft:cauldron".to_string(),
        item_form: true,
        ..Default::default()
    });

    // Magic Oven - магическая печь для варки эликсиров
    blocks.push(Block {
        id: "stellascrack:magic_oven".to_string(),
        template: "minecraft:furnace".to_string(),
        item_form: true,
        ..Default::default()
    });

    // Strawberry Fields Block - декоративный блок для биомов
    blocks.push(Block {
        id: "stellascrack:strawberry_hedge".to_string(),
        template: "minecraft:flower_pot".to_string(),
        item_form: true,
        ..Default::default()
    });

    // Sage Garden Block - декоративный блок для биомов
    blocks.push(Block {
        id: "stellascrack:sage_hedge".to_string(),
        template: "minecraft:flower_pot".to_string(),
        item_form: true,
        ..Default::default()
    });

    eprintln!("[stellascrack] Registered {} blocks", blocks.len());
}

/// Регистрируем предметы еды с эффектами и спраклами
pub fn food_items(items: &mut Vec<Item>) {
    // Strawberry Pie - основное блюдо со спраклами
    items.push(Item {
        id: "stellascrack:strawberry_pie".to_string(),
        template: "minecraft:baked_potato".to_string(),
        food_value: Some(5),
        food_saturation: Some(3.5),
        sparkles_on_eat: true,  // Магические спраклы при поедании
        sparkle_particle: "minecraft:red_tulip",
        effects: vec!["stellascrack:strawberry_boost".to_string()],
        max_stack_size: 64,
        ..Default::default()
    });

    // Sage Meat - мясо с эффектом мудрости
    items.push(Item {
        id: "stellascrack:sage_meat".to_string(),
        template: "minecraft:cooked_porkchop".to_string(),
        food_value: Some(4),
        food_saturation: Some(2.5),
        sparkle_on_eat: true,
        sparkle_particle: "minecraft:silver_glow_berries",
        effects: vec!["stellascrack:sage_wisdom".to_string()],
        max_stack_size: 64,
        ..Default::default()
    });

    // Crystal Soup - эликсир в хрустальной чаше
    items.push(Item {
        id: "stellascrack:crystal_soup".to_string(),
        template: "minecraft:potion".to_string(),
        food_value: Some(6),
        food_saturation: Some(4.0),
        sparkles_on_eat: true,
        sparkle_particle: "minecraft:glow_berries",
        effects: vec![
            "stellascrack:strawberry_boost".to_string(),
            "stellascrack:magic_energy".to_string()
        ],
        max_stack_size: 16,
        ..Default::default()
    });

    eprintln!("[stellascrack] Registered {} food items", items.len());
}

/// Регистрируем кастомные эффекты
pub fn effects(effects: &mut Vec<Effect>) {
    // Strawberry Boost - эффект от клубничной еды
    // Лечение, скорость, спраклы вокруг игрока
    effects.push(Effect {
        id: "stellascrack:strawberry_boost".to_string(),
        description: Some("Grants strawberry sparkles - heals over time and leaves red particle trails".to_string()),
        duration: 600,  // 10 минут
        amplifier: 1,
        particles: vec![
            (horizon_mods::world::BlockId::minecraft("red_tulip"), 20),   // Красные тюльпаны
            (horizon_mods::world::BlockId::minecraft("glow_berries"), 15), // Светящиеся ягоды
        ],
        healing: Some(3.0),  // Лечение за тик
        speed_multiplier: Some(1.2),  // Ускорение на 20%
        effects: vec![
            "heal".to_string(),
            "speed".to_string(),
            "jump_boost".to_string()
        ],
        ..Default::default()
    });

    // Sage Wisdom - эффект мудрости от sage мяса
    // Увеличивает эффективность добычи ресурсов и варки
    effects.push(Effect {
        id: "stellascrack:sage_wisdom".to_string(),
        description: Some("Sage wisdom - increases resource gathering and cooking efficiency in sage biomes".to_string()),
        duration: 900,  // 15 минут
        amplifier: 2,
        particles: vec![
            (horizon_mods::world::BlockId::minecraft("silver_glow_berries"), 30),
            (horizon_mods::world::BlockId::minecraft("candle_cake"), 15)
        ],
        mining_speed_multiplier: Some(1.5),
        cooking_efficiency: Some(2.0),
        effects: vec![
            "mining".to_string(),
            "efficiency".to_string()
        ],
        ..Default::default()
    });

    // Magic Energy - магическая энергия для эликсиров
    // Дает защиту и позволяет использовать магические способности
    effects.push(Effect {
        id: "stellascrack:magic_energy".to_string(),
        description: Some("Magical energy shield - grants protection and mana regeneration".to_string()),
        duration: 480,  // 8 минут
        amplifier: 1,
        particles: vec![
            (horizon_mods::world::BlockId::minecraft("glow_berries"), 40),
            (horizon_mods::world::BlockId::minecraft("end_rod"), 25)
        ],
        protection_level: Some(1.0),
        mana_regeneration: Some(true),
        effects: vec![
            "absorption".to_string(),
            "regeneration".to_string()
        ],
        ..Default::default()
    });

    // Couch Potato - эффект усталости от долгого сидения (как в Hands-On Delight)
    // Если варить слишком много еды без отдыха
    effects.push(Effect {
        id: "stellascrack:couch_potato".to_string(),
        description: Some("Too much sitting causes fatigue from cooking excess".to_string()),
        duration: 300,  // 5 минут (для лечения)
        amplifier: 1,
        particles: vec![
            (horizon_mods::world::BlockId::minecraft("dark_oak_log"), 15),
            (horizon_mods::world::BlockId::minecraft("torchflower"), 10)
        ],
        mining_speed_multiplier: Some(0.6),  // Уменьшение эффективности на 40%
        fatigue_level: Some(true),
        effects: vec![
            "slowness".to_string(),
            "mining_fatigue".to_string()
        ],
        ..Default::default()
    });

    eprintln!("[stellascrack] Registered {} effects", effects.len());
}

/// Регистрируем улучшения для блоков и предметов
pub fn upgrades(upgrades: &mut Vec<Upgrade>) {
    // Silver Cutting Board - улучшенная доска для резки
    upgrades.push(Upgrade {
        id: "stellascrack:silver_cutting_board".to_string(),
        base_item: Some("stellascrack:cutting_board".to_string()),
        durability_modifier: 3.0,  // В 3 раза прочнее
        cooking_speed_multiplier: 1.5,
        special_effects: vec!["stellascrack:strawberry_boost".to_string()],
        sparkle_on_use: true,
        sparkle_particle: "minecraft:silver_glow_berries",
        craft_recipe: Some(CraftRecipe {
            ingredients: vec![
                ItemId::new("minecraft:diamond"),
                ItemId::new("stellascrack:silver_ingot") * 4,
                ItemId::new("minecraft:netherite_scrap"),
                ItemId::new("stellascrack:magic_crystal") * 2
            ],
            result: "stellascrack:silver_cutting_board".to_string(),
            crafting_table_required: true,
        }),
        ..Default::default()
    });

    // Crystal Pot Upgrade - улучшенная хрустальная кастрюля
    upgrades.push(Upgrade {
        id: "stellascrack:crystal_pot_deluxe".to_string(),
        base_item: Some("minecraft:cauldron".to_string()),
        max_liquid_stack_size: 32,  // Больше жидкости
        cooking_speed_multiplier: 1.8,
        special_effects: vec![
            "stellascrack:magic_energy".to_string(),
            "stellascrack:sage_wisdom".to_string()
        ],
        sparkle_on_use: true,
        sparkle_particle: "minecraft:end_crystal",
        craft_recipe: Some(CraftRecipe {
            ingredients: vec![
                ItemId::new("minecraft:diamond"),
                ItemId::new("stellascrack:magic_crystal") * 4,
                ItemId::new("minecraft:lapis_lazuli") * 6
            ],
            result: "stellascrack:crystal_pot_deluxe".to_string(),
            crafting_table_required: true,
        }),
        ..Default::default()
    });

    eprintln!("[stellascrack] Registered {} upgrades", upgrades.len());
}

/// Биом-специфичная механика
pub fn biome_modifiers(biomes: &mut Vec<BiomeModifier>) {
    // Strawberry Fields - биом с клубникой
    biomes.push(BiomeModifier {
        id: "stellascrack:strawberry_fields".to_string(),
        description: Some("A magical field of glowing strawberry plants".to_string()),
        blocks: vec![
            BlockId::minecraft("grass_block"),
            BlockId::new("stellascrack:strawberry_hedge"),
            BlockId::minecraft("dirt_path")
        ],
        effects_on_spawn: Some(vec![
            "stellascrack:strawberry_boost".to_string()
        ]),
        food_bonus_multiplier: 1.5,  // Еда в этом биома дает больше эффектов
        sparkles_enabled: true,
        ..Default::default()
    });

    // Sage Garden - биом с sage травой для мудрости
    biomes.push(BiomeModifier {
        id: "stellascrack:sage_garden".to_string(),
        description: Some("A peaceful garden of healing sage plants".to_string()),
        blocks: vec![
            BlockId::minecraft("grass_block"),
            BlockId::new("stellascrack:sage_hedge"),
            BlockId::minecraft("coarse_dirt")
        ],
        effects_on_spawn: Some(vec![
            "stellascrack:sage_wisdom".to_string()
        ]),
        food_bonus_multiplier: 1.3,
        cooking_efficiency_bonus: 0.5,  // +50% к эффективности варки
        sparkles_enabled: true,
        ..Default::default()
    });

    eprintln!("[stellascrack] Registered {} biome modifiers", biomes.len());
}

/// Кастомный рецепт крафта для улучшений
#[derive(Debug, Clone)]
struct CraftRecipe {
    ingredients: Vec<ItemId>,
    result: String,
    crafting_table_required: bool,
}

/// ID предмета в игре
#[derive(Debug, Clone)]
struct ItemId(String);

impl ItemId {
    fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

/// Биом-модификатор для кастомных биомов
#[derive(Debug, Default)]
struct BiomeModifier {
    id: String,
    description: Option<String>,
    blocks: Vec<horizon_mods::world::BlockId>,
    effects_on_spawn: Option<Vec<String>>,
    food_bonus_multiplier: f32,
    cooking_efficiency_bonus: f32,
    sparkles_enabled: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blocks_count() {
        let mut blocks: Vec<Block> = vec![];
        blocks(blocks(&mut blocks));
        assert!(blocks.len() > 0);
        
        // Проверка, что cutting board зарегистрирован
        let has_cutting_board = blocks.iter().any(|b| b.id == "stellascrack:cutting_board");
        assert!(has_cutting_board, "Cutting board should be registered");
    }

    #[test]
    fn test_food_items_count() {
        let mut items: Vec<Item> = vec![];
        food_items(&mut items);
        assert!(items.len() > 0);
        
        // Проверка, что strawberry pie зарегистрирован
        let has_pie = items.iter().any(|i| i.id == "stellascrack:strawberry_pie");
        assert!(has_pie, "Strawberry pie should be registered");
    }

    #[test]
    fn test_effects_count() {
        let mut effects: Vec<Effect> = vec![];
        effects(&mut effects);
        assert!(effects.len() > 0);
        
        // Проверка, что strawberry boost зарегистрирован
        let has_boost = effects.iter().any(|e| e.id == "stellascrack:strawberry_boost");
        assert!(has_boost, "Strawberry boost effect should be registered");
    }

    #[test]
    fn test_upgrades_count() {
        let mut upgrades: Vec<Upgrade> = vec![];
        upgrades(&mut upgrades);
        assert!(upgrades.len() > 0);
        
        // Проверка, что silver cutting board зарегистрирован
        let has_silver_board = upgrades.iter().any(|u| u.id == "stellascrack:silver_cutting_board");
        assert!(has_silver_board, "Silver cutting board upgrade should be registered");
    }

    #[test]
    fn test_biome_modifiers_count() {
        let mut biomes: Vec<BiomeModifier> = vec![];
        biome_modifiers(&mut biomes);
        assert!(biomes.len() > 0);
        
        // Проверка, что strawberry fields зарегистрирован
        let has_strawberry_biome = biomes.iter().any(|b| b.id == "stellascrack:strawberry_fields");
        assert!(has_strawberry_biome, "Strawberry fields biome modifier should be registered");
    }
}
