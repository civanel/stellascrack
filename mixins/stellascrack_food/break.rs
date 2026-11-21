use horizon_client::game::Game;

pub fn before_break(_game: &mut Game, pos: [i32; 3]) {
    eprintln!("[stellascrack] Breaking block at {:?}", pos);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_before_break() {
        let mut game = Game::default();
        before_break(&mut game, [10, 64, 20]);
    }
}
