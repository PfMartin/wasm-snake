use wasm_bindgen::prelude::*;

use crate::snake::{Direction, Snake, SnakeCell};

#[wasm_bindgen]
pub struct World {
    width: usize,
    size: usize,
    snake: Snake,
}

#[wasm_bindgen]
impl World {
    pub fn from(width: usize, snake_idx: usize) -> World {
        World {
            width,
            size: width * width,
            snake: Snake::from(snake_idx, Direction::Left, 3),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn snake_head_idx(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn change_snake_dir(&mut self, direction: Direction) {
        self.snake.direction = direction
    }

    pub fn snake_cells(&self) -> *const SnakeCell {
        // Return a pointer to the first SnakeCell
        self.snake.body.as_ptr()
    }

    pub fn snake_length(&self) -> usize {
        self.snake.body.len()
    }

    pub fn step(&mut self) {
        let next_cell = self.generate_next_snake_cell();
        self.snake.body[0] = next_cell;
    }

    fn generate_next_snake_cell(&self) -> SnakeCell {
        let snake_idx = self.snake_head_idx();
        let row = snake_idx / self.width;

        match self.snake.direction {
            Direction::Up => {
                let threshold = snake_idx - (row * self.width);
                if snake_idx == threshold {
                    return SnakeCell((self.size - self.width) + threshold);
                }

                SnakeCell(snake_idx - self.width)
            }
            Direction::Right => {
                let threshold = (row + 1) * self.width;
                if snake_idx + 1 == threshold {
                    return SnakeCell(threshold - self.width);
                }

                SnakeCell(snake_idx + 1)
            }
            Direction::Down => {
                let threshold = snake_idx - ((self.width - row) * self.width);
                if snake_idx + self.width == threshold {
                    return SnakeCell(threshold - ((row + 1) * self.width));
                }

                SnakeCell(snake_idx + self.width)
            }
            Direction::Left => {
                let threshold = row * self.width;
                if snake_idx == threshold {
                    return SnakeCell(threshold + (self.width - 1));
                }

                SnakeCell(snake_idx - 1)
            }
        }
    }
}
