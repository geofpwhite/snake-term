use std::{
    collections::{HashSet, VecDeque},
    ops::DerefMut,
};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Coords {
    pub x: usize,
    pub y: usize,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    U,
    D,
    L,
    R,
}

impl Direction {
    fn next(self, coords: Coords, mx: usize, my: usize) -> Coords {
        let mut c = coords.clone();
        match self {
            Direction::U => {
                if c.y > 0 {
                    c.y -= 1;
                } else {
                    c.y = my - 1;
                }
            }
            Direction::D => {
                c.y += 1;
            }
            Direction::L => {
                if c.x > 0 {
                    c.x -= 1;
                } else {
                    c.x = mx - 1
                }
            }
            Direction::R => {
                c.x += 1;
            }
        };
        c.x %= mx;
        c.y %= my;
        c
    }
}

pub(crate) struct Snake {
    pub dir: Direction,
    pub max_x: usize,
    pub max_y: usize,
    pub snake: VecDeque<Coords>,
    pub snake_set: HashSet<Coords>,
    pub food: Coords,
}

impl Snake {
    pub fn new(max_x: usize, max_y: usize) -> Self {
        let food_coords = Coords {
            x: rand::random::<u64>() as usize % max_x,
            y: rand::random::<u64>() as usize % max_y,
        };
        Self {
            max_x: max_x,
            max_y: max_y,
            snake: VecDeque::from(vec![Coords {
                x: max_x / 2,
                y: max_y / 2,
            }]),
            dir: Direction::R,
            food: food_coords,
            snake_set: HashSet::new(),
        }
    }
    pub fn change_direction(&mut self, dir: Direction) {
        match dir {
            Direction::L | Direction::R => {
                if self.dir == Direction::U || self.dir == Direction::D {
                    self.dir = dir;
                }
            }
            Direction::U | Direction::D => {
                if self.dir == Direction::R || self.dir == Direction::L {
                    self.dir = dir;
                }
            }
        }
    }
    // returns
    pub fn next(&mut self) -> Option<GameOver> {
        let next = self.dir.next(self.snake[0], self.max_x, self.max_y);
        self.snake.push_front(next);
        self.snake_set.insert(self.snake[0]);
        let hold = self.snake.pop_back().unwrap();
        self.snake_set.remove(&hold);
        if self.snake.len() != self.snake_set.len() {
            return Some(GameOver::Loss);
        }
        if self.food == self.snake[0] {
            self.snake_set.insert(hold);
            self.snake.push_back(hold);
            if self.snake.len() != self.snake_set.len() {
                panic!("You Lost");
            }
            self.food = Coords {
                x: rand::random::<u64>() as usize % self.max_x,
                y: rand::random::<u64>() as usize % self.max_y,
            };
        }
        None
    }
}

pub enum GameOver {
    Win,
    Loss,
}
