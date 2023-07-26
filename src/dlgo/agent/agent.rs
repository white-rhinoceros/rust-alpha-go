//! Общие типы для реализации бота (агента) играющего в Go.

trait Agent {
    fn new() -> Self;
    //fn select_move(&self, game_state: &GameState);
}