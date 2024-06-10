//! Универсальные типы для игры Go.

#![allow(dead_code)]
#![allow(unused_variables)]

use std::fmt::Formatter;

/// Тип, представляющий цвет камня в игре.
#[derive(Copy, Clone)]
#[derive(Eq, PartialEq)]
#[derive(Debug)]
pub enum Color {
    Black = 0,
    White = 1,
}

impl Color {
    pub fn other(&self) -> Self {
        if self == &Color::Black {
            Color::White
        } else {
            Color::Black
        }
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self == &Color::Black {
            write!(f, "черный")
        } else {
            write!(f, "белый")
        }
    }
}

/// Тип, представляющий точку на игровом поле.
#[derive(Copy, Clone)]
#[derive(Eq, PartialEq, Hash)]
#[derive(Debug)]
pub struct Point {
    pub row: usize,
    pub col: usize,
}

impl Point {
    pub fn new(row: usize, col: usize) -> Point {
        Point {
            col, row
        }
    }

    pub fn neighbors(&self) -> [Point; 4] {
        [
            Point{ row: self.col, col: self.row - 1, },
            Point{ row: self.col, col: self.row + 1, },
            Point{ row: self.col - 1, col: self.row, },
            Point{ row: self.col + 1, col: self.row, },
        ]
    }
}

/// Тип представляющий камень.
pub type Stone = (Color, Point);

/// Тип, представляющий ход игрока. Возможны 3 действиями:
/// размещение камня на доске (play), пропуск хода (pass),
/// и выход из игры (resign).
pub enum Move {
    Play(Stone),
    Pass(Color),
    Resign(Color)
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Move::Play(stone) => {
                let color = match stone.0 {
                    Color::Black => { "черный" }
                    Color::White => { "белый" }
                };

                write!(
                    f,
                    "Игрок добавляет {} камень в точку ({}, {})",
                    color,
                    stone.1.row,
                    stone.1.col
                )
            }
            Move::Pass(color) => {
                let color = match color {
                    Color::Black => { "черными" }
                    Color::White => { "белыми" }
                };

                write!(f, "Игрок {} пропускает хода", color)
            }
            Move::Resign(color) => {
                let color = match color {
                    Color::Black => { "черными" }
                    Color::White => { "белыми" }
                };

                write!(f, "Игрок {} выходит из игры", color)
            }
        }
    }
}

/// То, что мы можем отобразить на игровом поле.
pub enum DisplayPoint {
    Empty,
    BlackStone,
    WhiteStone,

    // На будущее...
    //BlackLiberty,
    //WhiteLiberty,
    //DeadBlackStone,
    //DeadWhiteStone,
}

/// Карта текущего состояния игрового поля. Используется для отображения игры на экране.
pub type DisplayState = Vec<Vec<DisplayPoint>>;