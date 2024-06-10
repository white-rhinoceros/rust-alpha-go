/// Rust Alpha Go!

use std::sync::mpsc::channel;
use std::thread::spawn;
use crate::display::ScreenType::{Console, Tetra};
use crate::dlgo::gotypes::{Move, Color, Stone, Point, DisplayState};
use crate::dlgo::board::game::Game;

mod dlgo;
mod display;

fn main() {
    // Сценарий запуска
    let board_size: usize = 19;

    let mut game = Game::new(board_size);

    // Канал для пересылки сообщений о состоянии игры.
    let (sender, receiver) = channel::<DisplayState>();


    // Запуск отображения игры в отдельном потоке.
    // Оператор move копирует (копируемый тип) board_size в замыкание.
    let handler = spawn(move || {
        display::launch(
            Tetra,
            board_size,
            receiver,
            "./resources",
            "Игра Go!"
        ).unwrap();
    });

    /* Далее тестовый код */
    {
        let stone: Stone = (Color::Black, Point::new(3, 3));
        game.apply_move(Move::Play(stone)).unwrap();

        // Отображаем игру.
        sender.send(game.get_display_state())
            .expect("Не удалось отправить данные в канал для отображения");
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




    // Ждем завершение потока - т.е. закрытие окна в потоке.
    handler.join().unwrap();
}
