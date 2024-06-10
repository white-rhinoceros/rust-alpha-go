

use console_engine::screen::Screen as Console;
use console_engine::pixel;
use console_engine::Color as ConsoleColor;

use crate::dlgo::board::game::{Game};
use crate::dlgo::gotypes::{DisplayPoint};


pub fn print_board(game: &Game) {
    let mut scr = Console::new(
        game.get_size().try_into().unwrap(),
        game.get_size().try_into().unwrap(),
    );

    for row in 1..=game.get_size() {
        for col in 1..=game.get_size() {
            match game.get_display_point(row, col) {
                DisplayPoint::Empty => {
                    scr.set_pxl(
                        col as i32 - 1,
                        row as i32 - 1,
                        pixel::pxl_fg('.', ConsoleColor::Cyan)
                    );
                }

                // DisplayPoint::Stone(color) => {
                //     let char = if color == Color::Black {
                //         'x'
                //     } else {
                //         'o'
                //     };
                //
                //     scr.set_pxl(
                //         col as i32 - 1,
                //         row as i32 - 1,
                //         pixel::pxl_fg(char, ConsoleColor::White)
                //     );
                // }

                DisplayPoint::BlackStone => {}
                DisplayPoint::WhiteStone => {}
            }
        }
    }

    // print the screen to the terminal
    scr.draw();
}