

use console_engine::screen::Screen as Console;
use console_engine::pixel;
use console_engine::Color as ConsoleColor;

use crate::dlgo::board::game::{Game, PointType};
use crate::dlgo::gotypes::Color;


pub fn print_board(game: &Game) {
    let mut scr = Console::new(
        game.get_size().try_into().unwrap(),
        game.get_size().try_into().unwrap(),
    );

    for row in 1..=game.get_size() {
        for col in 1..=game.get_size() {
            match game.get_point_type(row, col) {
                PointType::Empty => {
                    scr.set_pxl(
                        col as i32 - 1,
                        row as i32 - 1,
                        pixel::pxl_fg('.', ConsoleColor::Cyan)
                    );
                }
                PointType::Stone(color) => {
                    let char = if color == Color::Black {
                        'x'
                    } else {
                        'o'
                    };

                    scr.set_pxl(
                        col as i32 - 1,
                        row as i32 - 1,
                        pixel::pxl_fg(char, ConsoleColor::White)
                    );
                }
            }
        }
    }

    // print the screen to the terminal
    scr.draw();
}