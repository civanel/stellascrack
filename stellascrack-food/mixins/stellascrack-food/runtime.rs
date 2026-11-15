use crate::game::entity::Entity;
use crate::game::item::ItemStack;
use crate::game::world::World;
use horizon_mods::content::Block;

#[horizon::mixin_hook]
pub fn native_blocks(blocks: &mut Vec<Block>) {
    let board = Block {
        id: "stellascrack:cutting_board".to_string(),
        template: "minecraft:stonecutter".to_string(),
        item: true,
        ..Default::default()
    };
    blocks.push(board);
}
#[horizon::mixin_hook]
pub fn native_right_click_block(
    _world: &mut World,
    _player: &mut Entity,
    _block_pos: [i32; 3],
    _item_in_hand: &ItemStack,
) -> bool {
    eprintln!("[stellascrack] Клик по блоку!");
    false
}

#[cfg(test)]
mod tests {
    #[test]
    fn placeholder_test() {
        assert_eq!(1 + 1, 2);
    }
}