use crate::dlgo::gotypes::{Move, Color};
use crate::dlgo::board::game::Game;

mod dlgo;

fn main() {
    // Сценарий запуска
    let board_size: usize = 9;

    let mut game = Game::new(board_size);

    // В тестовых целях делаем один ход.
    {
        game.apply_move(Move::Pass(Color::Black)).unwrap();
    }

    // let bots = (
    //     agent::naive::RandomBot::new(),
    //     agent::naive::RandomBot::new(),
    // );
    //
    // while !game.is_over() {
    //     // Очистка экрана и задержка.
    //
    //     print_board(game.board());
    //
    //     //let bot_move = boots.game.next_player();
    //     let bot_move = match game.next_player() {
    //         Color::Black => {
    //             bots.0.select_move(game)
    //         }
    //         Color::White => {
    //             bots.1.select_move(game)
    //         }
    //     };
    //
    //     print_move(game.next_player(), bot_move);
    //
    //     game.apply_move(bot_move);
    // }
}
