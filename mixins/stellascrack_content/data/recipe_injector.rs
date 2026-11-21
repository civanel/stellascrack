use horizon_client::game::Recipe;
use horizon_mods::world::BlockId;

/// Рецепты крафта для всех предметов моды
pub fn recipes(recipes: &mut Vec<Recipe>) {
    // ============ CUTTING BOARD RECIPES ============
    
    // Basic Cutting Board
    recipes.push(Recipe {
        id: "stellascrack:cutting_board".to_string(),
        type_: RecipeType::crafting_table,
        pattern: vec![
            "###".to_string(),  // Row 1 - 3 oak logs
            "# #".to_string(),  // Row 2 - gap
            "# #".to_string(),  // Row 2 - gap
        ],
        result_item: Item {
            id: "stellascrack:cutting_board".to_string(),
            count: 1,
        },
        required_tools: None,
    });

    // Silver Cutting Board (Upgrade)
    recipes.push(Recipe {
        id: "stellascrack:silver_cutting_board".to_string(),
        type_: RecipeType::crafting_table,
        pattern: vec![
            " D ".to_string(),  // Diamond
            " SSS ".to_string(),  // Silver ingots
            " SSS ".to_string(),
            " NNN ".to_string(),  // Netherite scrap
            " C C ".to_string(),  // Magic crystals
        ],
        result_item: Item {
            id: "stellascrack:silver_cutting_board".to_string(),
            count: 1,
        },
        required_tools: Some(ItemId::new("minecraft:netherite_pickaxe")),
    });

    // ============ CRYSTAL POT RECIPES ============
    
    // Basic Crystal Pot
    recipes.push(Recipe {
        id: "stellascrack:crystal_pot".to_string(),
        type_: RecipeType::crafting_table,
        pattern: vec![
            "GGG".to_string(),  // Glass
            "G T G ".to_string(),  // Top glass with crystal
            "GCG".to_string(),  // Bottom glass with cobble
        ],
        result_item: Item {
            id: "stellascrack:crystal_pot".to_string(),
            count: 1,
        },
        required_tools: None,
    });

    // Deluxe Crystal Pot (Upgrade)
    recipes.push(Recipe {
        id: "stellascrack:crystal_pot_deluxe".to_string(),
        type_: RecipeType::crafting_table,
        pattern: vec![
            " D ".to_string(),  // Diamond
            " CCC ".to_string(),  // Crystal blocks
            " LLL ".to_string(),  // Lapis lazuli
        ],
        result_item: Item {
            id: "stellascrack:crystal_pot_deluxe".to_string(),
            count: 1,
        },
        required_tools: Some(ItemId::new("minecraft:diamond_pickaxe")),
    });

    // ============ MAGIC OVEN RECIPES ============
    
    // Magic Oven
    recipes.push(Recipe {
        id: "stellascrack:magic_oven".to_string(),
        type_: RecipeType::crafting_table,
        pattern: vec![
            "###".to_string(),  // Nether bricks
            "#M#".to_string(),  // Middle with magic crystal
            "###".to_string(),
        ],
        result_item: Item {
            id: "stellascrack:magic_oven".to_string(),
            count: 1,
        },
        required_tools: None,
    });

    // ============ STRAWBERRY PLANT RECIPES ============
    
    // Strawberry Hedge (decorative block)
    recipes.push(Recipe {
        id: "stellascrack:strawberry_hedge".to_string(),
        type_: RecipeType::crafting_table,
        pattern: vec![
            "SSS".to_string(),  // Redstone + strawberries
            "RRR".to_string(),  // Redstone again
            "SRS".to_string(),  // Mix for magic effect
        ],
        result_item: Item {
            id: "stellascrack:strawberry_hedge".to_string(),
            count: 4,
        },
        required_tools: None,
    });

    // Sage Hedge (decorative block)
    recipes.push(Recipe {
        id: "stellascrack:sage_hedge".to_string(),
        type_: RecipeType::crafting_table,
        pattern: vec![
            "SAG".to_string(),  // Sapling + sage
            "SA G".to_string(),  // Mix with gap
            "SGA".to_string(),  // Staggered pattern
        ],
        result_item: Item {
            id: "stellascrack:sage_hedge".to_string(),
            count: 4,
        },
        required_tools: None,
    });

    // ============ FOOD ITEMS RECIPES ============
    
    // Strawberry Pie
    recipes.push(Recipe {
        id: "stellascrack:strawberry_pie".to_string(),
        type_: RecipeType::cutting_board,  // Requires cutting board
        pattern: vec![
            "B P B".to_string(),  // Bottom with strawberries on top
            "S S S".to_string(),  // Strawberries
            "B C B".to_string(),  // Bottom with cream in middle
        ],
        result_item: Item {
            id: "stellascrack:strawberry_pie".to_string(),
            count: 1,
        },
        required_tools: Some(ItemId::new("minecraft:shears")),
    });

    // Sage Meat (cooked with sage)
    recipes.push(Recipe {
        id: "stellascrack:sage_meat".to_string(),
        type_: RecipeType::cutting_board,
        pattern: vec![
            "S M S".to_string(),  // Sage on meat
            "M T M".to_string(),  // Meat with torch (cooked)
            "S M S".to_string(),
        ],
        result_item: Item {
            id: "stellascrack:sage_meat".to_string(),
            count: 1,
        },
        required_tools: Some(ItemId::new("minecraft:copper_axe")),
    });

    // Crystal Soup (in crystal pot)
    recipes.push(Recipe {
        id: "stellascrack:crystal_soup".to_string(),
        type_: RecipeType::special,  // Special cooking in crystal pot
        pattern: vec![
            "C B C".to_string(),  // Crystal pot with water/broth
            "C W C".to_string(),  // With ingredients
            "C M C".to_string(),  // Magic crystals for effect
        ],
        result_item: Item {
            id: "stellascrack:crystal_soup".to_string(),
            count: 1,
        },
        required_tools: Some(ItemId::new("minecraft:magic_wand")),
    });

    // ============ CRAFTING TABLES FOR SPECIAL ITEMS ============
    
    // Magic Crystal (needed for upgrades)
    recipes.push(Recipe {
        id: "stellascrack:magic_crystal".to_string(),
        type_: RecipeType::crafting_table,
        pattern: vec![
            "E E E ".to_string(),  // End crystals
            "E G E".to_string(),  // Glass in middle
            "E L E".to_string(),  // Lapis for magic color
        ],
        result_item: Item {
            id: "stellascrack:magic_crystal".to_string(),
            count: 1,
        },
        required_tools: Some(ItemId::new("minecraft:netherite_sword")),
    });

    // Silver Ingot (needed for upgrades)
    recipes.push(Recipe {
        id: "stellascrack:silver_ingot".to_string(),
        type_: RecipeType::crafting_table,
        pattern: vec![
            "III".to_string(),  // Iron ingots
            "I I".to_string(),  // Center piece
            "G G ".to_string(),  // Gold for color
        ],
        result_item: Item {
            id: "stellascrack:silver_ingot".to_string(),
            count: 1,
        },
        required_tools: None,
    });

    eprintln!("[stellascrack_content] Registered {} recipes", recipes.len());
}

/// Типы рецептов
#[derive(Debug, Clone)]
enum RecipeType {
    CraftingTable,  // Стандартный крафт-стол
    CuttingBoard,   // Крафт на cutting board (только с модными блоками)
    Special,        // Специальный рецепт (например, в magic oven или crystal pot)
}
