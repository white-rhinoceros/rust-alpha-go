use crate::display::console::print_board;
use crate::dlgo::gotypes::{Move, Color, Stone, Point};
use crate::dlgo::board::game::Game;

mod dlgo;
mod display;

fn main() {
    // Сценарий запуска
    let board_size: usize = 9;

    let mut game = Game::new(board_size);

    // В тестовых целях делаем один ход.
    {
        let stone: Stone = (Color::Black, Point::new(3, 3));
        game.apply_move(Move::Play(stone)).unwrap();
    }

    print_board(&game);

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
