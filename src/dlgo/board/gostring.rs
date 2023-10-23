//! Модуль цепочки камней.

#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashSet;
use std::fmt::Formatter;
use crate::dlgo::gotypes::{Point, Color};

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

/// Цепочка камней. Определяет связанную группу камней и ее степени свободы.
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)] // Для сравнения цепочек камней.
pub struct GoString {
    color: Color,
    stones: HashSet<Point>,
    liberties: HashSet<Point>,
}

impl GoString {
    /// Конструирует новую цепочку камней.
    ///
    /// # Arguments
    ///
    /// * `color`: Цвет цепочки камней.
    /// * `stones`: Срез точек занятых камнями цепочки.
    /// * `liberties`: Срез из точек представляющих точки свободы группы камней.
    ///
    /// returns: Self
    pub fn new(color: Color, stones: Vec<Point>, liberties: Vec<Point>) -> Self {
        GoString {
            color,
            stones: HashSet::from_iter(stones),
            liberties: HashSet::from_iter(liberties),
        }
    }

    /// Возвращает цвет цепочки камней.
    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn get_stones(&self) -> Vec<Point> {
        let mut stones = Vec::new();
        for stone in &self.stones {
            stones.push(stone.clone());
        }

        stones
    }

    pub fn get_stones2(&self) -> &HashSet<Point> {
        &self.stones
    }

    /// Число степеней свободы у данной цепочке камней.
    pub fn num_liberties(&self) -> usize {
        self.liberties.len()
    }

    /// Добавляет степень свободы цепочке камней. Результат зависит от того, была ли
    /// добавляемая точка представлена в цепочке. Если да то возвращается false, в
    /// противном случае true.
    ///
    /// # Arguments
    ///
    /// * `point`: Точка на игровом поле (степень свободы).
    ///
    /// returns: bool
    pub fn add_liberty(&mut self, point: Point) {
        self.liberties.insert(point);
    }

    /// Удаляет степень свободы из цепочки камней. Возвращаемый результат зависит
    /// от того, находилась ли удаляемая точка в цепочке.
    ///
    /// # Arguments
    ///
    /// * `point`: Точка на игровом поле (степень свободы).
    ///
    /// returns: bool
    pub fn remove_liberty(&mut self, point: &Point) -> bool {
        self.liberties.remove(point)
    }

    /// Объединение двух цепочек камней. Метод принимает владение данной цепочкой.
    /// TODO: Сделать оператором.
    ///
    /// # Arguments
    ///
    /// * `other`: Ссылка на присоединяемую цепочку.
    ///
    /// returns: self
    pub fn merged_with(&mut self, other: &Self) -> &Self {
        assert_eq!(
            self.color,
            other.color,
            "Не допускается объединение цепочек камней разного цвета"
        );

        // Объединим камни обоих цепочек. Точка реализует типаж Clone.
        for stone in &other.stones {
            self.stones.insert(stone.clone());
        }

        // Что-бы получить число степеней свободы объединенной цепочки камней,
        // сложим степени свободы слагаемых цепочек и вычтем из полученной
        // суммы камни принадлежащие новой цепочки.
        for stone in &other.liberties {
            self.liberties.insert(stone.clone());
        }

        for stone in &self.stones {
            self.liberties.remove(stone);
        }

        self
    }

    /// Сравнение на равенство двух цепочек камней.
    /// TODO: Сделать оператором.
    ///
    /// # Arguments
    ///
    /// * `other`:
    ///
    /// returns: bool
    pub fn equal(&self, other: &Self) -> bool {
        if
            self.color == other.color
            && self.stones == other.stones
            && self.liberties == other.liberties
        {
            return true;
        }

        false
    }
}

// // Implement `Iterator` for `GoString`.
// impl Iterator for &GoString {
//     // Мы перебираем точки цепочки.
//     type Item = Point;
//
//     // next() is the only required method
//     fn next(&mut self) -> Option<Self::Item> {
//         match self.stones.iter().next() {
//             None => { None }
//             Some(point) => { Some(*point) }
//         }
//     }
// }

// Вводим типаж Write для строк.
use std::fmt::Write;

// Форматный вывод цепочки камней.
impl std::fmt::Display for GoString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut message = String::new();
        message += LINE_ENDING;

        message += match self.color {
            Color::Black => { "Цепочка черных камней:" }
            Color::White => { "Цепочка белых камней:" }
        };

        message += LINE_ENDING;

        message += "Содержит точки: [";
        for point in &self.stones {
            write!(message, "{:?}, ", point).unwrap();
        }
        message += "]";

        message += LINE_ENDING;

        message += "Имеет точки свободы: [";
        for point in &self.liberties {
            write!(message, "{:?}, ", point).unwrap();
        }
        message += "]";

        write!(f, "{}{}", message, LINE_ENDING)
    }
}

/// Тестирование методов цепочки камней.
#[test]
pub fn go_string_test() {
    let stones = vec![
        Point::new(1, 2),
        Point::new(2, 2),
    ];

    let mut liberties: Vec<Point> = vec![
        Point::new(1, 1),
        Point::new(2, 1),
        Point::new(3, 2),
        Point::new(2, 3),
        Point::new(1, 4),
    ];

    let string = GoString::new(
        Color::Black,
        stones,
        liberties,
    );
}


