//! Модуль реализует структуры для хранения игрового состояния.

#![allow(dead_code)]
#![allow(unused_variables)]

use std::rc::Rc;
use crate::dlgo::error::FatalError;
use crate::dlgo::board::goboard::Board;
use crate::dlgo::gotypes::{Color, Move, Point};

/// Точка на игровом поле может иметь следующие типы.
pub enum PointType {
    Empty,
    Stone(Color),
    //Liberty(Color),
    //DeadStone(Color),
}

/// Игра в Go!
pub struct Game {
    // Состояние в игре неизменяемо (!, тип Rc). Это значит, что после
    // создания состояния, новое состояние можно создать лиш клонированием
    // старого, совершить ход и заморозить (обернуть в Rc).
    state: Rc<GameState>,
    board_size: usize,
    is_over: bool,
}

impl Game {
    /// Конструктор. Создает новую игру.
    ///
    /// # Arguments
    ///
    /// * `board_size`: Размер доски
    ///
    /// returns: GameState
    pub fn new(board_size: usize) -> Self {
        Game {
            state: Rc::new(GameState {
                // Пустая доска размера board_size x board_size.
                board: Board::new(board_size, board_size),
                // Первым в игру вступает игрок черными камнями.
                player_color: Color::Black,
                previous_state: None,
                last_move: None,
            }),
            board_size,
            is_over: false,
        }
    }

    /// Реализует ход игрока в игре.
    ///
    /// # Arguments
    ///
    /// * `player_move`: Ход игрока. Ход содержит цвет камней игрока.
    ///
    /// returns: Result<(), RecoverableError>
    pub fn apply_move(&mut self, player_move: Move) -> Result<(), FatalError> {
        if self.is_over {
            let err = FatalError::new(format!(
                "Игра завершена! Нельзя делать ходы в завершенной игре."
            ));

            return Err(err);
        }

        self.state = match player_move {
            // Размещение камня на доске, доска изменилась.
            Move::Play(stone) => {
                // Проверим правильность хода: цвет камня, размещаемого на доске, должен
                // совпадать с цветом камней, которыми должен играть "следующий игрок"
                // (хранится в поле GameState::next_player_color).
                let color = stone.0;
                if color != self.state.player_color {
                    let err = FatalError::new(
                        format!(
                            "Размещаемый камень должен иметь {} цвет",
                            self.state.player_color
                        )
                    );

                    return Err(err);
                }

                // Следующее состояние доски: клонируем доску и размещаем камень.
                let mut next_board = self.state.board.clone();
                // Транслируем ошибку вызывающей функции.
                next_board.place_stone(stone)?;

                // Создаем новое состояние в игре.
                Rc::new(GameState {
                    board: next_board,
                    player_color: color.other(),
                    previous_state: Some(self.state.clone()),
                    last_move: Some(player_move),
                })
            }

            // Пропуск хода или выход из игры, расположение камней не меняется.
            Move::Pass(color) | Move::Resign(color) => {
                // Создаем новое состояние в игре.
                Rc::new(GameState {
                    board: self.state.board.clone(),
                    player_color: color.other(),
                    previous_state: Some(self.state.clone()),
                    last_move: Some(player_move),
                })
            }
        };

        // После совершенного хода - проверяем, не завершилась ли игра.
        self.is_over = self.state.is_over();

        Ok(())
    }

    /// Метод определяет, завершена ли игра.
    pub fn is_over(&self) -> bool {
        self.is_over
    }

    /// Отдает размер доски.
    pub fn get_size(&self) -> usize {
        self.board_size
    }

    pub fn get_point_type(&self, row: usize, col: usize) -> PointType {
        let point = Point::new(row, col);

        match self.state.board.get_go_string(&point) {
            None => { PointType::Empty }
            Some(string) => { PointType::Stone(string.get_color()) }
        }
    }
}

/// Вспомогательная структура хранящая состояние игры.
struct GameState {
    board: Board,                      // Текущее состояние доски (к этому состоянию ожидается
                                       // ход цветом, который хранится в поле player_color).
    player_color: Color,               // Цвет игрока (камня), который должен сделать ход.
    previous_state: Option<Rc<Self>>,  // Предыдущее состояние доски (до "последнего" хода).
    last_move: Option<Move>,           // Последний ход (ход который перевел доску в текущее
                                       // состояние).
}

impl GameState {
    /// Вспомогательный метод, отдающий "ситуацию" в игре, т.е. кортеж из игрока (цвета),
    /// который должен сделать ход и состояния доски.
    fn situation(&self) -> (&Color, &Board) {
        (&self.player_color, &self.board)
    }

    /// Определение момента окончания игры по последнему ходу, т.е. по "ходу" который перевел
    /// игровую доску в текущее состояние.
    /// Игра оканчивается в случае: игрок выходит из игры (текущий ход Move::Resign), игроки
    /// подряд пропускают ход (Move::Pass).
    fn is_over(&self) -> bool {
        if let Some(last_move) = &self.last_move {
            // Что это за ход?
            match last_move {
                // Это пропуск хода?
                Move::Pass(_) => {
                    match &self.previous_state {
                        Some(previous_last_state) => {
                            // Это второй "пас", игра завершается.
                            if let Some(Move::Pass(_)) = previous_last_state.last_move {
                                return true;
                            }
                        }
                        // Если предыдущего состояния не было, это первый пас.
                        _ => { return false; }
                    }
                }
                // Это выход игрока из игры?
                Move::Resign(_) => { return true; }
                // Любой другой ход.
                _ => { return false; }
            }
        }

        // Если self.last_move равен None, значит игра только началась,
        // и ни один игрок не сделал хода. Игра не окончена.
        false
    }

    ///  Определяет, приведет ли ход к самозахвату.
    ///
    /// # Arguments
    ///
    /// * `player`: Игрок делающий ход.
    /// * `player_move`: Ход игрока.
    ///
    /// returns: bool
    fn is_move_self_capture(&self, player: Color, player_move: Move) -> bool {


        /*
        if not move.is_play:
            return False
        next_board = copy.deepcopy(self.board)
        next_board.place_stone(player, move.point)
        new_string = next_board.get_go_string(move.point)

        return new_string.num_liberties == 0

        */
        todo!()
    }

    /// Метод определяет, нарушит ли ход игрока правило "ко" (ситуационное суперко).
    ///
    /// # Arguments
    ///
    /// * `player`: Игрок делающий ход.
    /// * `player_move`: Ход игрока.
    ///
    /// returns: bool
    fn does_move_violate_ko(&self, player: Color, player_move: Move) -> bool {


        /*
        if not move.is_play:
            return False

        next_board = copy.deepcopy(self.board)
        next_board.place_stone(player, move.point)
        next_situation = (player.other, next_board)

        past_state = self.previous_state
        while past_state is not None:
            if past_state.situation == next_situation:
                return True
            past_state = past_state.previous_state

        return False
        */
        todo!()
    }

    ///  Показывает, является ли ход допустимым для данного игрового состояния.
    ///
    /// # Arguments
    ///
    /// * `player_move`: Ход игрока.
    ///
    /// returns: bool
    fn is_valid_move(&self, player_move: Move) -> bool {


        todo!()
        /*
        if self.is_over():
            return False
        if move.is_pass or move.is_resign:
            return True
        return (
            self.board.get(move.point) is None
            and not self.is_move_self_capture(self.next_player, move)
            and self.does_move_violate_ko(self.next_player, move)
        )
        */
    }
}