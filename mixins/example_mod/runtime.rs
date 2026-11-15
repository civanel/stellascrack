use crate::game::entity::Entity;
use crate::game::item::ItemStack;
use crate::game::world::World;
use horizon_mods::content::Block;

// Хук для сбора (регистрации) блоков
#[horizon::mixin_hook]
pub fn native_blocks(blocks: &mut Vec<Block>) {
    let board = Block {
        id: "stellascrack:cutting_board".to_string(),
        template: "minecraft:stonecutter".to_string(), // Копируем поведение камнереза
        item: true, // Добавляем предмет в инвентарь
        ..Default::default()
    };
    blocks.push(board);
}

// Хук клика правой кнопкой мыши по блоку
#[horizon::mixin_hook]
pub fn native_right_click_block(
    _world: &mut World,
    _player: &mut Entity,
    _block_pos: [i32; 3],
    _item_in_hand: &ItemStack,
) -> bool {
    // Выводим в консоль, чтобы проверить, что клик работает
    eprintln!("[stellascrack] Клик по блоку!");
    
    // Возвращаем false, чтобы не отменять стандартное поведение
    false
}

#[cfg(test)]
mod tests {
    #[test]
    fn placeholder_test() {
        assert_eq!(1 + 1, 2);
    }
}