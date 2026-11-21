/// Регистрируем все предметы моды в творческом режиме (Creative Inventory)
/// Это позволяет игрокам найти предметы через / и Creative Tab

pub fn items(items: &mut Vec<horizon_client::game::Item>) {
    // ============ BLOCKS AND ITEMS ============
    
    // --- CUTTING BOARD VARIANTS ---
    
    // Basic Cutting Board - в creative только если есть upgrade
    items.push(horizon_client::game::CreativeItem {
        id: "stellascrack:cutting_board".to_string(),
        display_name: Some("Stella's Cutting Board".to_string()),
        lore: Some(vec![
            "Craft magical foods here!",
            "Right-click to cook",
            "Left-click to pick up as item"
        ]),
        rarity: horizon_client::game::Rarity::COMMON,
        creative_tab: horizon_client::game::CreativeTab::BLOCKS_AND_ITEMS,  // E tab!
        stack_size: Some(64),
    });

    // Silver Cutting Board - upgrade
    items.push(horizon_client::game::CreativeItem {
        id: "stellascrack:silver_cutting_board".to_string(),
        display_name: Some("Silver Cutting Board".to_string()),
        lore: Some(vec![
            "Enhanced durability (3x)",
            "+50% cooking speed",
            "Strawberry boost on use"
        ]),
        rarity: horizon_client::game::Rarity::EPIC,  // Золотой цвет в creative!
        creative_tab: horizon_client::game::CreativeTab::BLOCKS_AND_ITEMS,
        stack_size: Some(64),
    });

    // --- CRYSTAL POT VARIANTS ---
    
    // Crystal Pot - basic version
    items.push(horizon_client::game::CreativeItem {
        id: "stellascrack:crystal_pot".to_string(),
        display_name: Some("Crystal Cooking Pot".to_string()),
        lore: Some(vec![
            "Special recipes with sparkles",
            "Larger capacity (16)"
        ]),
        rarity: horizon_client::game::Rarity::UNCOMMON,  // Синий цвет
        creative_tab: horizon_client::game::CreativeTab::BLOCKS_AND_ITEMS,
        stack_size: Some(64),
    });

    // Deluxe Crystal Pot - premium upgrade  
    items.push(horizon_client::game::CreativeItem {
        id: "stellascrack:crystal_pot_deluxe".to_string(),
        display_name: Some("Deluxe Crystal Pot".to_string()),
        lore: Some(vec![
            "Maximum capacity (32)",
            "Magic energy & sage wisdom",
            "Craft with diamond tools"
        ]),
        rarity: horizon_client::game::Rarity::LEGENDARY,  // Розовый/фиолетовый цвет!
        creative_tab: horizon_client::game::CreativeTab::BLOCKS_AND_ITEMS,
        stack_size: Some(64),
    });

    // --- MAGIC OVEN ---
    
    // Magic Oven - special cooking block
    items.push(horizon_client::game::CreativeItem {
        id: "stellascrack:magic_oven".to_string(),
        display_name: Some("Magic Oven".to_string()),
        lore: Some(vec![
            "Cook elixirs and potions",
            "Special recipes only"
        ]),
        rarity: horizon_client::game::Rarity::UNCOMMON,
        creative_tab: horizon_client::game::CreativeTab::BLOCKS_AND_ITEMS,
        stack_size: Some(64),
    });

    // --- DECORATIVE BLOCKS ---
    
    // Strawberry Hedge - decorative for biomes
    items.push(horizon_client::game::CreativeItem {
        id: "stellascrack:strawberry_hedge".to_string(),
        display_name: Some("Strawberry Hedge Block".to_string()),
        lore: Some(vec![
            "Magical strawberry field decoration",
            "50% food effect boost in biome"
        ]),
        rarity: horizon_client::game::Rarity::COMMON,
        creative_tab: horizon_client::game::CreativeTab::BLOCKS_AND_ITEMS,
        stack_size: Some(64),
    });

    // Sage Hedge - decorative for biomes
    items.push(horizon_client::game::CreativeItem {
        id: "stellascrack:sage_hedge".to_string(),
        display_name: Some("Sage Garden Block".to_string()),
        lore: Some(vec![
            "Healing sage decoration",
            "+50% cooking efficiency"
        ]),
        rarity: horizon_client::game::Rarity::COMMON,
        creative_tab: horizon_client::game::CreativeTab::BLOCKS_AND_ITEMS,
        stack_size: Some(64),
    });

    // --- FOOD ITEMS ---
    
    // Strawberry Pie - main dish with sparkles!
    items.push(horizon_client::game::CreativeItem {
        id: "stellascrack:strawberry_pie".to_string(),
        display_name: Some("Magical Strawberry Pie".to_string()),
        lore: Some(vec![
            "✨ Red tulip sparkles",
            "+3 health, +20% speed, jump boost",
            "Lasts 10 minutes"
        ]),
        rarity: horizon_client::game::Rarity::EPIC,
        creative_tab: horizon_client::game::CreativeTab::BLOCKS_AND_ITEMS,
        stack_size: Some(64),
    });

    // Sage Meat - wisdom effect meat
    items.push(horizon_client::game::CreativeItem {
        id: "stellascrack:sage_meat".to_string(),
        display_name: Some("Wisdom Sage Meat".to_string()),
        lore: Some(vec![
            "✨ Silver glow berry sparkles",
            "+Mining efficiency 50%",
            "Best in sage garden biome"
        ]),
        rarity: horizon_client::game::Rarity::EPIC,
        creative_tab: horizon_client::game::CreativeTab::BLOCKS_AND_ITEMS,
        stack_size: Some(64),
    });

    // Crystal Soup - magical elixir
    items.push(horizon_client::game::CreativeItem {
        id: "stellascrack:crystal_soup".to_string(),
        display_name: Some("Crystal Soup Elixir".to_string()),
        lore: Some(vec![
            "✨ Multi-color magic sparkles",
            "Protection + mana regen",
            "Cook in crystal pot"
        ]),
        rarity: horizon_client::game::Rarity::LEGENDARY,  // Розовый!
        creative_tab: horizon_client::game::CreativeTab::BLOCKS_AND_ITEMS,
        stack_size: Some(16),  // ПOTION stacks = 16
    });

    // --- CRAFTING MATERIALS ---
    
    // Magic Crystal - needed for upgrades
    items.push(horizon_client::game::CreativeItem {
        id: "stellascrack:magic_crystal".to_string(),
        display_name: Some("Magic Crystal Shard".to_string()),
        lore: Some(vec![
            "Craft with netherite tools",
            "End crystal + glass + lapis"
        ]),
        rarity: horizon_client::game::Rarity::UNCOMMON,
        creative_tab: horizon_client::game::CreativeTab::BLOCKS_AND_ITEMS,
        stack_size: Some(64),
    });

    // Silver Ingot - crafting material
    items.push(horizon_client::game::CreativeItem {
        id: "stellascrack:silver_ingot".to_string(),
        display_name: Some("Silver Ingot".to_string()),
        lore: Some(vec![
            "Craft with iron + gold",
            "4 per silver cutting board"
        ]),
        rarity: horizon_client::game::Rarity::COMMON,
        creative_tab: horizon_client::game::CreativeTab::BLOCKS_AND_ITEMS,  // Item tab (E)!
        stack_size: Some(64),
    });

    eprintln!("[stellascrack_content] Registered {} items in creative inventory", items.len());
}
