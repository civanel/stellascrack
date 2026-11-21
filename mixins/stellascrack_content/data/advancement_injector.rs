/// Достижения для моды - награды за выполнение заданий

pub fn advancements(advancements: &mut Vec<horizon_client::game::Advancement>) {
    // ============ MAIN CATEGORY ============
    
    let main_category = horizon_client::game::Category {
        id: "stellascrack_main".to_string(),
        title: "Stella's Magic Kitchen",
        description: "Discover the secrets of magical cooking and farming",
        icon: Some("stellascrack:silver_cutting_board"),
    };

    // ============ STARTING ADVANTAGES ============
    
    let first_crafting = horizon_client::game::Advancement {
        id: "first_cutting_board".to_string(),
        title: "First Cutting Board",
        description: "Craft your first stellascrack cutting board",
        reward: Some(horizon_client::game::Reward {
            items: vec![Item {
                id: "stellascrack:silver_cutting_board".to_string(),
                count: 1,
            }],
        }),
        display_conditions: DisplayConditions {
            show_toast: true,
            announce_to_chat: true,
            award_achievements: true,
        },
    };

    let first_food = horizon_client::game::Advancement {
        id: "first_food".to_string(),
        title: "First Bite",
        description: "Cook and eat your first magical meal",
        reward: Some(horizon_client::game::Reward {
            items: vec![Item {
                id: "stellascrack:strawberry_pie".to_string(),
                count: 1,
            }],
        }),
    };

    // ============ COOKING MASTERY ============
    
    let master_cook = horizon_client::game::Advancement {
        id: "master_cook".to_string(),
        title: "Master Cook",
        description: "Cook all magical dishes in the stellascrack kitchen",
        reward: Some(horizon_client::game::Reward {
            items: vec![Item {
                id: "stellascrack:crystal_pot_deluxe".to_string(),
                count: 1,
            }],
        }),
    };

    let sparkles_mastery = horizon_client::game::Advancement {
        id: "sparkles_master".to_string(),
        title: "Sparkle Master",
        description: "Unlock all sparkle effects by eating magical foods",
        reward: Some(horizon_client::game::Reward {
            items: vec![Item {
                id: "stellascrack:magic_crystal".to_string(),
                count: 1,
            }],
        }),
    };

    // ============ BIOME DISCOVERY ============
    
    let strawberry_fields = horizon_client::game::Advancement {
        id: "strawberry_fields".to_string(),
        title: "Strawberry Fields",
        description: "Visit the magical strawberry fields biome",
        reward: None,
    };

    let sage_garden = horizon_client::game::Advancement {
        id: "sage_garden".to_string(),
        title: "Sage Wisdom",
        description: "Discover the peaceful sage garden biome",
        reward: None,
    };

    // ============ UPGRADE MASTERY ============
    
    let upgrade_mastery = horizon_client::game::Advancement {
        id: "upgrade_master".to_string(),
        title: "Upgrade Master",
        description: "Craft all stellascrack upgrades and improvements",
        reward: Some(horizon_client::game::Reward {
            items: vec![Item {
                id: "stellascrack:silver_cutting_board".to_string(),
                count: 1,
            }],
        }),
    };

    // ============ EFFECT MASTERY ============
    
    let effect_master = horizon_client::game::Advancement {
        id: "effect_master".to_string(),
        title: "Effect Master",
        description: "Experience all magical effects from stellascrack foods",
        reward: Some(horizon_client::game::Reward {
            items: vec![Item {
                id: "stellascrack:strawberry_boost".to_string(),
                count: 1,
            }],
        }),
    };

    // ============ FARMING MASTERY ============
    
    let farming_master = horizon_client::game::Advancement {
        id: "farming_master".to_string(),
        title: "Farming Master",
        description: "Grow all stellascrack plants in their biomes",
        reward: Some(horizon_client::game::Reward {
            items: vec![Item {
                id: "stellascrack:sage_hedge".to_string(),
                count: 1,
            }],
        }),
    };

    // ============ FINAL BOSS ACHIEVEMENT ============
    
    let grand_chef = horizon_client::game::Advancement {
        id: "grand_chef".to_string(),
        title: "Grand Chef",
        description: "Become a true master of stellascrack cooking - craft everything and enjoy all effects!",
        reward: Some(horizon_client::game::Reward {
            items: vec![Item {
                id: "stellascrack:crystal_pot_deluxe".to_string(),
                count: 1,
            }],
        }),
    };

    eprintln!("[stellascrack_content] Registered {} advancements", advancements.len());
}

/// Условия отображения достижения
#[derive(Debug, Clone)]
struct DisplayConditions {
    show_toast: bool,
    announce_to_chat: bool,
    award_achievements: bool,
}
