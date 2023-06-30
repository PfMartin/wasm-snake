use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

enum Direction {
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
            snake: Snake::from(snake_idx, Direction::Left),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn snake_head_idx(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn update(&mut self) {
        let snake_idx = self.snake_head_idx();
        let row = snake_idx / self.width;

        self.snake.body[0].0 = match self.snake.direction {
            Direction::Up => (snake_idx + 1) % self.size,
            Direction::Right => (row * self.width) + (snake_idx + 1) % self.width,
            Direction::Down => (snake_idx + 1) % self.size,
            Direction::Left => (row * self.width) + (snake_idx - 1) % self.width,
        }
    }
}

struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}

impl Snake {
    fn from(spawn_index: usize, direction: Direction) -> Snake {
        Snake {
            body: vec![SnakeCell(spawn_index)],
            direction,
        }
    }
}
