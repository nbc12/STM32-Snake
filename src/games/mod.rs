mod snake;


use oorandom::Rand32;
use snake::Snake;
use crate::NUM_PIXELS;

// pub enum Games {
//     Title,
//     Snake,
// }
pub struct Context<'a> {
    pub global_frame: &'a u64,
    pub rng: &'a mut Rand32
}

pub struct InputState {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub center: bool,
}

pub enum UpdateResult {
    Continue,
    Win(usize),
    Loss(usize),
    // ResetRequested,
    // NewGame(Games),
}

pub trait Game {
    fn new(context: &mut Context) -> Self;

    /// returns an `UpdateResult`, telling the wrapper what to do next
    fn update(&mut self, inputs: &InputState, context: &mut Context) -> UpdateResult;

    fn display(&self) -> [u8; NUM_PIXELS];
}

#[derive(Clone)]
pub enum GameState {
    Snake(Snake),
    // Title,
}

impl GameState {
    pub fn new(context: &mut Context) -> Self {
        let new_snake = Self::Snake(Snake::new(context));
        new_snake
    }

    pub fn update(&mut self, inputs: &InputState, context: &mut Context) {
        let update_result = match self {
            GameState::Snake(snake) => snake.update(inputs, context),
            // GameState::Title => todo!(),
        };

        match update_result {
            UpdateResult::Continue => {},
            UpdateResult::Win(_score) => {
                *self = Self::Snake(Snake::new(context));
            },
            UpdateResult::Loss(_) => {
                *self = Self::Snake(Snake::new(context));
            },
            // UpdateResult::ResetRequested => {
            //     Self::Snake(Snake::new(context))
            // },
            // UpdateResult::NewGame(_) => Self::Snake(game)
        }
    }

    pub fn display(&self) -> [u8; NUM_PIXELS] {
        match self {
            GameState::Snake(snake) => snake.display(),
            // GameState::Title => todo!(),
        }
    }
}
