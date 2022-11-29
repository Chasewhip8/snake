use rand::prelude::*;

use crate::control::Control;
use crate::control::Control::RIGHT;
use crate::state::State::{FruitCollected, GameOver, Running};

#[derive(PartialEq, Clone)]
pub struct Point {
    x: u8,
    y: u8
}

impl Point {
    pub fn from(x: u8, y: u8) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> u8 {
        self.x
    }

    pub fn y(&self) -> u8 {
        self.y
    }
}

#[derive(PartialEq)]
pub enum State {
    Running,
    FruitCollected,
    GameOver
}

pub struct Board {
    width: u8,
    height: u8,

    state: State,
    move_direction: Control,

    snake: Vec<Point>,
    fruit: Option<Point>,
    frames_since_fruit: u16
}

impl Board {
    /// Create a new instance of the board with default values
    pub fn new(width: u8, height: u8) -> Self {
        let tail_point = Point::from(
            width / 2 as u8,
            height / 2 as u8
        );

        Board {
            width,
            height,

            state: Running,
            move_direction: RIGHT,

            snake: Vec::from([
                Point::from(tail_point.x - 3, tail_point.y),
                Point::from(tail_point.x - 2, tail_point.y),
                Point::from(tail_point.x - 1, tail_point.y),
                tail_point
            ]),
            fruit: None,
            frames_since_fruit: 0
        }
    }

    /// Get the next point where the snake + the user input is facing
    fn get_next_point(board: &Self) -> Point {
        let head = board.snake.last().expect("Invalid Game State, Snake is empty");
        return match board.move_direction {
            Control::UP => Point::from(head.x, head.y - 1),
            Control::DOWN => Point::from(head.x, head.y + 1),
            Control::LEFT => Point::from(head.x - 1, head.y),
            _ => Point::from(head.x + 1, head.y)
        }
    }

    /// Move the snake to position determined by get_next_point
    pub fn move_snake(board: &mut Self){
        if board.state == GameOver {
            return;
        }

        let next_move = Board::get_next_point(board);
        board.snake.push(next_move);
    }

    /// Check for collisions against the walls and itself, sets the game state to GameOver if fails.
    pub fn check_collision_death(board: &mut Self){
        for snake_body in &board.snake {
            if snake_body.x <= 0 || snake_body.x >= board.width
                || snake_body.y <= 0 || snake_body.y >= board.height {
                board.state = GameOver;
                return;
            }
        }

        let next_move = Board::get_next_point(board);
        for snake_body in &board.snake {
            if next_move == *snake_body {
                board.state = GameOver;
                return;
            }
        }
    }

    /// Check the collision against food on the game board.
    pub fn check_collision_food(board: &mut Self){
        if let Some(fruit) = &board.fruit {
            let last_body = board.snake.last().cloned().expect("Invalid State, Snake is empty");
            if *fruit == last_body {
                board.state = FruitCollected;
                board.fruit = None;
            }
        }
    }

    /// Remove the tail of the snake
    pub fn chop_tail(board: &mut Self){
        if board.state == FruitCollected {
            board.state = Running;
            return;
        }

        if board.state != Running {
            return;
        }

        let last_body = board.snake.first().cloned().expect("Invalid State, Snake is empty");
        board.snake.retain(|body| body != &last_body);
    }

    /// Switch the control in the state
    pub fn change_control(board: &mut Self, control: Control) {
        board.move_direction = control;
    }

    /// Spawn a fruit randomly on the map
    pub fn spawn_fruit(board: &mut Self){
        if let Some(_fruit) = &board.fruit {
            return;
        }

        let frames_since_fruit = board.frames_since_fruit;
        if frames_since_fruit < 5 {
            board.frames_since_fruit += 1;
            return;
        }

        let mut rng = thread_rng();
        let pos_x = rng.gen_range(4..(board.width - 4));
        let pos_y = rng.gen_range(4..(board.height - 4));
        board.fruit = Some(Point::from(pos_x, pos_y));
        board.frames_since_fruit = 0;
    }

    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn snake(&self) -> &Vec<Point> {
        &self.snake
    }

    pub fn fruit(&self) -> &Option<Point> {
        &self.fruit
    }
}





