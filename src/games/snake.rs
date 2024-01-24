use oorandom::Rand32;

use super::{Context, Game, InputState, UpdateResult};
use crate::{
    util::{self, pos_to_idx},
    DISPLAY_DIMS, NUM_PIXELS,
};

#[derive(Clone, Copy, Debug)]
pub enum SnakeTile {
    Empty,
    Snake(usize),
    Apple,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
pub struct Snake {
    gameboard: [SnakeTile; NUM_PIXELS],
    snake_length: usize,
    head_idx: usize,
    snake_dir: Option<Direction>,
}

impl Snake {
    /// Will block infinitely if there are no free spaces on the board
    pub fn create_new_apple(&mut self, bad_idx: Option<usize>, context: &mut Context) {
        let good_cell = loop {
            // pick a random cell
            let rng: &mut Rand32 = context.rng;
            let rand_cell = rng.rand_range(0..NUM_PIXELS as u32) as usize;

            let next_head_idx = bad_idx.unwrap_or(usize::MAX);

            if rand_cell != next_head_idx {
                // is this cell where the head needs to be placed?
                if let SnakeTile::Empty = self.gameboard[rand_cell] {
                    // if the cell is empty, we found a spot!
                    break rand_cell;
                }
            }
        };

        self.gameboard[good_cell] = SnakeTile::Apple;
    }
}

impl Game for Snake {
    fn new(context: &mut Context) -> Self {
        let mut gameboard = [SnakeTile::Empty; NUM_PIXELS];
        let snake_length = 1;
        let head_idx = pos_to_idx((4, 4));
        gameboard[head_idx] = SnakeTile::Snake(snake_length);

        let mut s = Snake {
            gameboard,
            snake_length,
            head_idx,
            snake_dir: None,
        };

        s.create_new_apple(None, context);

        s
    }

    fn update(&mut self, inputs: &InputState, context: &mut Context) -> UpdateResult {
        self.snake_dir = match &self.snake_dir {
            None => {
                if inputs.up {
                    Some(Direction::Up)
                } else if inputs.down {
                    Some(Direction::Down)
                } else if inputs.right {
                    Some(Direction::Right)
                } else if inputs.left {
                    Some(Direction::Left)
                } else {
                    None
                }
            }
            Some(dir) => {
                if inputs.up && *dir != Direction::Down {
                    Some(Direction::Up)
                } else if inputs.down && *dir != Direction::Up {
                    Some(Direction::Down)
                } else if inputs.right && *dir != Direction::Left {
                    Some(Direction::Right)
                } else if inputs.left && *dir != Direction::Right {
                    Some(Direction::Left)
                } else {
                    Some(dir.clone())
                }
            }
        };

        let cur_head_pos = util::idx_to_pos(self.head_idx);

        let dir = if let Some(dir) = &self.snake_dir {
            dir
        } else {
            return UpdateResult::Continue;
        };

        let next_head_pos = match dir {
            Direction::Up => (
                cur_head_pos.0,
                if cur_head_pos.1 == 0 {
                    DISPLAY_DIMS.1 - 1
                } else {
                    cur_head_pos.1 - 1
                },
            ),
            Direction::Down => (
                cur_head_pos.0,
                if cur_head_pos.1 == DISPLAY_DIMS.1 - 1 {
                    0
                } else {
                    cur_head_pos.1 + 1
                },
            ),
            Direction::Left => (
                if cur_head_pos.0 == 0 {
                    DISPLAY_DIMS.0 - 1
                } else {
                    cur_head_pos.0 - 1
                },
                cur_head_pos.1,
            ),
            Direction::Right => (
                if cur_head_pos.0 == DISPLAY_DIMS.0 - 1 {
                    0
                } else {
                    cur_head_pos.0 + 1
                },
                cur_head_pos.1,
            ),
        };

        let next_head_idx = util::pos_to_idx(next_head_pos);

        match self.gameboard[next_head_idx] {
            // if the snake will eat an apple, dont decrement the snake's tiles. just increment the length.
            SnakeTile::Apple => {
                self.snake_length += 1;
                if self.snake_length == NUM_PIXELS {
                    return UpdateResult::Win(self.snake_length);
                }
                self.create_new_apple(Some(next_head_idx), context);
            }
            // if the snake will eat itself, end the game.
            SnakeTile::Snake(_) => {
                if self.snake_dir.is_some() {
                    return UpdateResult::Loss(self.snake_length);
                }
            }
            // if the snake won't eat anything, decrement snake segments lifetime
            SnakeTile::Empty => {
                self.gameboard.iter_mut().for_each(|tile| match tile {
                    SnakeTile::Snake(length) => {
                        *tile = if *length == 1 {
                            SnakeTile::Empty
                        } else {
                            SnakeTile::Snake(*length - 1)
                        };
                    }
                    _ => {}
                });
            }
        }
        self.head_idx = next_head_idx;
        self.gameboard[self.head_idx] = SnakeTile::Snake(self.snake_length);

        UpdateResult::Continue
    }

    fn display(&self) -> [u8; NUM_PIXELS] {
        let mut buf = [0; NUM_PIXELS];
        for (i, tile) in self.gameboard.iter().enumerate() {
            buf[i] = match tile {
                SnakeTile::Empty => 0,
                SnakeTile::Snake(_) => 2,
                SnakeTile::Apple => 1,
            };
        }
        buf
    }
}
