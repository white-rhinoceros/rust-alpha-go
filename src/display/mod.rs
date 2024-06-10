use std::sync::mpsc::Receiver;
use crate::display::tetra::Window;
use crate::dlgo::gotypes::DisplayState;

pub mod console;
pub mod tetra;

/// Типы экранов на которых можно отобразить игру.
pub enum ScreenType {
    Console,
    Tetra,
}

pub fn launch(
    screen_type: ScreenType,
    size: usize,
    receiver: Receiver<DisplayState>,
    resources_path: &str,
    title: &str
) -> Result<(), String> {
    match screen_type {
        ScreenType::Tetra => {
            match Window::new(size, receiver, resources_path, title) {
                Ok(_) => { Ok(()) }
                Err(err) => { Err(err.to_string()) }
            }
        }
        // ScreenType::Console => {
        //     Ok(Screen{
        //         driver:
        //     })
        // }
        _ => {
            Err("Неизвестный драйвер".to_string())
        }
    }
}