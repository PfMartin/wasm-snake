use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

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

    pub fn update(&mut self) {
        let snake_idx = self.snake_head_idx();
        let row = snake_idx / self.width;
        let col = snake_idx % self.width;

        self.snake.body[0].0 = match self.snake.direction {
            Direction::Up => (row - 1) % self.width * self.width + col,
            Direction::Right => (row * self.width) + (col + 1) % self.width,
            Direction::Down => (row + 1) % self.width * self.width + col,
            Direction::Left => (row * self.width) + (col - 1) % self.width,
        }
    }
}

pub struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}

impl Snake {
    fn from(spawn_index: usize, direction: Direction, size: usize) -> Snake {
        let mut body = vec![];

        for i in 0..size {
            body.push(SnakeCell(spawn_index - i))
        }

        Snake { body, direction }
    }
}
