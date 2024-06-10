//! Доска для игры в Go.

#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::{HashMap};
use std::rc::Rc;
use crate::dlgo::board::gostring::GoString;
use crate::dlgo::error::FatalError;
use crate::dlgo::gotypes::{Point, Stone};

/// Структура, представляющая доску для игры в Go. Реализует типаж Clone (в связи
/// с необходимостью хранить несколько конфигураций доски). Доска содержит цепочки
/// камней.
#[derive(Clone)]
pub struct Board {
    num_rows: usize,
    num_cols: usize,
    // Словарь, хранящий цепочки камней. Клюём в словаре являются точки на
    // доске занятые камнями, а значениями цепочки камней. Цепочки камней
    // обернуты в Rc (в тип с подсчетом ссылок), т.к. каждый камень (через
    // отображение) должен ссылаться на свою-же цепочку.
    grid: HashMap<Point, Rc<GoString>>
}

impl Board {
    /// Конструктор.
    ///
    /// # Arguments
    ///
    /// * `num_rows`: Число строк игрового поля.
    /// * `num_cols`: Число колонок игрового поля.
    ///
    /// returns: Board
    pub fn new(num_rows: usize, num_cols: usize) -> Self {
        Board {
            num_rows,
            num_cols,
            grid: HashMap::new(),
        }
    }

    ///
    ///
    /// # Arguments
    ///
    /// * `point`:
    ///
    /// returns: Option<&Rc<GoString>>
    pub fn get_go_string(&self, point: &Point) -> Option<&Rc<GoString>> {
        self.grid.get(point)
    }

    /// Размещение камня на доске и проверка количества степеней свободы соседних точек.
    ///
    /// # Arguments
    ///
    /// * `stone`: Размещаемый камень.
    ///
    /// Returns: ()
    pub fn place_stone(&mut self, stone: Stone) -> Result<(), FatalError> {
        let color = stone.0;
        let point= stone.1;

        if !self.is_on_grid(&point) {
            let err = FatalError::new(format!(
                "Точка размещения камня ({}, {}) находится за границами сетки доски ({}, {})",
                point.row,
                point.col,
                self.num_rows,
                self.num_cols,
            ));

            return Err(err);
        }

        if let Some(_) = self.grid.get(&point) {
            let err = FatalError::new(format!(
                "В точке ({}, {}) уже находится камень принадлежащей цепочке камней",
                point.row,
                point.col,
            ));

            return Err(err)
        }

        // Заведем переменные (adjacent - примыкающий, соседний).
        // Степени свободы данной точки.
        let mut liberties: Vec<Point> = Vec::new();

        // Цепочки камней, которые прилегают к размещаемому камню,
        // с одинаковым с ним цветом.
        let mut adjacent_same_color: Vec<Rc<GoString>> = Vec::new();

        // Цепочки камней, прилегающие к переданному камню,
        // с противоположным цветом.
        let mut adjacent_opposite_color: Vec<Rc<GoString>> = Vec::new();

        // Замыкание для поиска уже добавленной цепочки в вектор. Цепочки сравниваются
        // на равенство поскольку реализуют типаж PartialEq.
        let string_search = |haystack: &Vec<Rc<GoString>>, needle: &GoString| -> bool {
            let mut found = false;
            for sample in haystack {
                if (*sample).as_ref() == needle {
                    found = true;
                }
            }

            found
        };

        // И так, нам передали точку, сначала исследуем ее окружение.
        for neighbor in point.neighbors() {
            if !self.is_on_grid(&neighbor) {
                continue;
            }

            match self.grid.get(&neighbor) {
                // Если соседняя точка не занята цепочкой, увеличиваем свободы.
                None => {
                    liberties.push(neighbor);
                }
                Some(string) => {
                    // Если соседняя точка принадлежит цепочке,
                    // нужно разобрать эти цепочки по цвету.
                    if string.get_color() == color {
                        if !string_search(&adjacent_same_color, string) {
                            adjacent_same_color.push(string.clone());
                        }
                    } else {
                        if !string_search(&adjacent_opposite_color, string) {
                            adjacent_opposite_color.push(string.clone());
                        }
                    }
                }
            }
        }

        // Из переданной точки создадим цепочку с одним камнем.
        let mut new_string = GoString::new(color, vec![point], liberties);

        // Объединим все смежные цепочки камней одного вида, включая только что созданную.
        // Т.к. переменная adjacent_same_color нам больше не нужна, потребляем ее в цикле.
        for same_color_string in adjacent_same_color {
            new_string.merged_with(same_color_string.as_ref());
        }

        let new_string_rc = Rc::new(new_string);

        // Ко всем точкам образующих данную объединенную цепочку, привяжем ее саму.
        for p in new_string_rc.get_stones2() {
            self.grid.insert(p.clone(), new_string_rc.clone());
        }

        // Уменьшим количества степеней свободы соседних цепочек камней противоположного цвета.
        // Поскольку цепочки на доске не изменяемы... Клонируем цепочку и удаляем переданную
        // точку из свобод цепочки. Затем клонированную цепочку снова размещаем на доске.
        for opposite_color_string in &adjacent_opposite_color {
            let mut string: GoString = opposite_color_string.as_ref().clone();
            string.remove_liberty(&point);

            // Если размещения камня приводит к тому, что у цепочки не остается степеней свободы, то
            // удаляем с доски цепочки камней противоположного цвета с нулевой степенью свободы.
            if string.num_liberties() == 0 {
                self.remove_string(&string)
            } else {
                self.insert_string(string);
            }
        }

        Ok(())
    }

    /// Метод проверяет, попадает ли переданная точка в границы сетки доски.
    ///
    /// # Arguments
    ///
    /// * `point`: Точка на доске.
    ///
    /// returns: bool
    fn is_on_grid(&self, point: &Point) -> bool {
        (1 <= point.row) && (point.row <= self.num_rows)
            && (1 <= point.col) && (point.col <= self.num_cols)
    }

    fn insert_string(&mut self, string: GoString) {
        let as_rc = Rc::new(string);

        for p in as_rc.get_stones() {
            // Метод обновляет значения.
            self.grid.insert(p, as_rc.clone());
        }
    }

    fn remove_string(&mut self, string: &GoString) {
        /*

    def _remove_string(self, string: GoString) -> None:
        """
        Удаление цепочки камней с доски (учитываются также случаи, когда удаление цепочки может привести к
        увеличению степеней свободы других цепочек).
        :param string: GoString Удаляемая цепочка камней.
        """
        for point in string.stones:
            for neighbor in point.neighbors():
                neighbor_string = self._grid.get(neighbor)

                if neighbor_string is None:
                    continue

                if neighbor_string is not string:
                    neighbor_string.add_liberty(point)

            self._grid[point] = None
        */
    }

}
