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
    fn get_next_point(&self) -> Point {
        let head = self.snake.last().expect("Invalid Game State, Snake is empty");
        return match self.move_direction {
            Control::UP => Point::from(head.x, head.y - 1),
            Control::DOWN => Point::from(head.x, head.y + 1),
            Control::LEFT => Point::from(head.x - 1, head.y),
            _ => Point::from(head.x + 1, head.y)
        }
    }

    /// Move the snake to position determined by get_next_point
    pub fn move_snake(&mut self){
        if self.state == GameOver {
            return;
        }

        self.snake.push(self.get_next_point());
    }

    /// Check for collisions against the walls and itself, sets the game state to GameOver if fails.
    pub fn check_collision_death(&mut self){
        for snake_body in &self.snake {
            if snake_body.x <= 0 || snake_body.x >= self.width
                || snake_body.y <= 0 || snake_body.y >= self.height {
                self.state = GameOver;
                return;
            }
        }

        let next_move = self.get_next_point();
        for snake_body in &self.snake {
            if next_move == *snake_body {
                self.state = GameOver;
                return;
            }
        }
    }

    /// Check the collision against food on the game board.
    pub fn check_collision_food(&mut self){
        if let Some(fruit) = &self.fruit {
            let last_body = self.snake.last().cloned().expect("Invalid State, Snake is empty");
            if *fruit == last_body {
                self.state = FruitCollected;
                self.fruit = None;
            }
        }
    }

    /// Remove the tail of the snake
    pub fn chop_tail(&mut self){
        if self.state == FruitCollected {
            self.state = Running;
            return;
        }

        if self.state != Running {
            return;
        }

        let last_body = self.snake.first().cloned().expect("Invalid State, Snake is empty");
        self.snake.retain(|body| body != &last_body);
    }

    /// Switch the control in the state
    pub fn change_control(&mut self, control: Control) {
        self.move_direction = control;
    }

    /// Spawn a fruit randomly on the map
    pub fn spawn_fruit(&mut self){
        if let Some(_fruit) = &self.fruit {
            return;
        }

        let frames_since_fruit = self.frames_since_fruit;
        if frames_since_fruit < 5 {
            self.frames_since_fruit += 1;
            return;
        }

        let mut rng = thread_rng();
        let pos_x = rng.gen_range(4..(self.width - 4));
        let pos_y = rng.gen_range(4..(self.height - 4));
        self.fruit = Some(Point::from(pos_x, pos_y));
        self.frames_since_fruit = 0;
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





