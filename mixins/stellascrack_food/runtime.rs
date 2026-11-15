use horizon_mods::content::Block;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref BOARD_ITEMS: Mutex<HashMap<[i32; 3], String>> = Mutex::new(HashMap::new());
}

#[horizon::mixin_hook]
pub fn native_blocks(blocks: &mut Vec<Block>) {
    blocks.push(Block {
        id: "stellascrack:cutting_board".to_string(),
        template: "minecraft:stonecutter".to_string(),
        item: true,
        ..Default::default()
    });
}

#[horizon::mixin_hook]
pub fn native_right_click_block(
    pos: [i32; 3],
    _face: [i32; 3],
    canceled: &mut bool,
) {
    eprintln!("[StelleScrack] Клик по блоку на координатах: {:?}", pos);
}