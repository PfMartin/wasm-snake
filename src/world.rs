use wasm_bindgen::prelude::*;

use crate::snake::{Direction, Snake, SnakeCell};

#[wasm_bindgen(module = "/www/utils/rnd.js")]
extern "C" {
    fn random(max: usize) -> usize;
}

#[wasm_bindgen]
pub struct World {
    width: usize,
    size: usize,
    snake: Snake,
    next_cell: Option<SnakeCell>,
    reward_cell: usize,
}

#[wasm_bindgen]
impl World {
    pub fn from(width: usize, snake_idx: usize) -> World {
        let snake = Snake::from(snake_idx, Direction::Right, 3);

        let size = width * width;
        let reward_cell = World::generate_reward_cell(size, &snake.body);

        World {
            width,
            size,
            snake,
            next_cell: None,
            reward_cell,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn snake_head_idx(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn snake_cells(&self) -> *const SnakeCell {
        // Return a pointer to the first SnakeCell
        self.snake.body.as_ptr()
    }

    pub fn reward_cell(&self) -> usize {
        self.reward_cell
    }

    pub fn snake_length(&self) -> usize {
        self.snake.body.len()
    }

    pub fn change_snake_dir(&mut self, direction: Direction) {
        let next_cell = self.generate_next_snake_cell(&direction);

        if self.snake.body[1].0 == next_cell.0 {
            return;
        }

        self.next_cell = Some(next_cell);
        self.snake.direction = direction
    }

    pub fn step(&mut self) {
        let temp = self.snake.body.clone();

        match self.next_cell {
            Some(cell) => {
                self.snake.body[0] = cell;
                self.next_cell = None;
            }
            None => {
                self.snake.body[0] = self.generate_next_snake_cell(&self.snake.direction);
            }
        }

        for i in 1..self.snake.body.len() {
            self.snake.body[i] = SnakeCell(temp[i - 1].0)
        }

        if self.reward_cell == self.snake_head_idx() {
            if self.snake_length() < self.size {
                self.reward_cell = World::generate_reward_cell(self.size, &self.snake.body)
            } else {
                self.reward_cell = 1000;
            }

            self.snake.body.push(SnakeCell(self.snake.body[1].0));
        }
    }

    fn generate_next_snake_cell(&self, direction: &Direction) -> SnakeCell {
        let snake_idx = self.snake_head_idx();
        let row = snake_idx / self.width;

        match direction {
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

    fn generate_reward_cell(max: usize, snake_body: &Vec<SnakeCell>) -> usize {
        let mut reward_cell;

        'reward_cell_not_in_snake_validation: loop {
            reward_cell = random(max);
            if !snake_body.contains(&SnakeCell(reward_cell)) {
                break 'reward_cell_not_in_snake_validation;
            }
        }

        reward_cell
    }
}
