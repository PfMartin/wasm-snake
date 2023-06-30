use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy, PartialEq)]
pub struct SnakeCell(pub usize);

pub struct Snake {
    pub body: Vec<SnakeCell>,
    pub direction: Direction,
}

impl Snake {
    pub fn from(spawn_index: usize, direction: Direction, size: usize) -> Snake {
        let mut body = vec![];

        for i in 0..size {
            body.push(SnakeCell(spawn_index - i))
        }

        Snake { body, direction }
    }
}
