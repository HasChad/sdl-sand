use macroquad::color::Color;
use rand::{thread_rng, Rng};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CellState {
    Dead,
    Sand,
    Water,
    Stone,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Direction {
    Right,
    Left,
    None,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CellType {
    Solid,
    Liquid,
    //Gas,
    None,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Cell {
    pub state: CellState,
    pub cell_type: CellType,
    pub move_direction: Direction,
    pub is_moved: bool,
    pub color: Color,
}

impl Cell {
    pub fn spawn_empty() -> Cell {
        Cell {
            state: CellState::Dead,
            move_direction: Direction::None,
            cell_type: CellType::None,
            is_moved: false,
            color: Color::from_rgba(10, 10, 10, 255),
        }
    }
    pub fn spawn_sand() -> Cell {
        Cell {
            state: CellState::Sand,
            move_direction: if thread_rng().gen_bool(0.5) {
                Direction::Left
            } else {
                Direction::Right
            },
            cell_type: CellType::Solid,
            is_moved: true,
            color: Color::from_rgba(255, 204, 92, 255),
        }
    }

    pub fn spawn_water() -> Cell {
        Cell {
            state: CellState::Water,
            move_direction: if thread_rng().gen_bool(0.5) {
                Direction::Left
            } else {
                Direction::Right
            },
            cell_type: CellType::Liquid,
            is_moved: true,
            color: Color::from_rgba(71, 140, 207, 255),
        }
    }

    pub fn spawn_stone() -> Cell {
        Cell {
            state: CellState::Stone,
            move_direction: Direction::None,
            cell_type: CellType::Solid,
            is_moved: true,
            color: Color::from_rgba(151, 125, 139, 255),
        }
    }
}
