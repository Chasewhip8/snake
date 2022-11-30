extern crate core;

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use pancurses::{Attribute, Attributes, initscr, Window};

use crate::control::get_control;
use crate::state::{Board, Point, State};

mod state;
mod control;

const GAME_SPEED_MS: u64 = 100;

fn main() {
    let frame_ms_time = Duration::from_millis(GAME_SPEED_MS);
    let mut frame_end_time;

    let board = &mut Board::new(78, 24);
    let window = &mut create_window();

    loop {
        // Get the start time + the frame delay in ms
        frame_end_time = SystemTime::now().duration_since(UNIX_EPOCH)
            .unwrap()
            .checked_add(frame_ms_time)
            .unwrap();

        // Capture inputs as they come in and wait until the next frame is needed
        while SystemTime::now().duration_since(UNIX_EPOCH).unwrap() < frame_end_time {
            if let Some(control) = get_control(window) {
                Board::change_control(board, control);
            }
            std::thread::sleep(Duration::from_millis(5));
        }

        // Use the previously captured latest control to execute game logic on
        update_logic(board);

        // Display the game board.
        display_game(board, window);

        // Quit the game if collision checks fail
        if *Board::state(board) == State::GameOver {
            return;
        }
    }
}

/// Create a window object with correct input settings configured
fn create_window() -> Window {
    let window = initscr();
    window.timeout(10);
    //window.nodelay(false); Required for windows??
    window.attrset(Attributes::default() | Attribute::Bold);
    return window;
}

/// Display the game onto the terminal
fn display_game(board: &mut Board, window: &Window) {
    window.clear();
    for snake_body in Board::snake(board) {
        window.mvaddch(Point::y(snake_body) as i32, Point::x(snake_body) as i32,'X');
    }
    for fruit in Board::fruit(board) {
        window.mvaddch(Point::y(fruit) as i32, Point::x(fruit) as i32,'O');
    }
    window.mv(0, 0);
    window.refresh();
}

fn update_logic(board: &mut Board){
    // Check Collision and set State Flags
    Board::check_collision_death(board);
    Board::check_collision_food(board);

    // Execute the current move inside board.move_direction
    Board::move_snake(board);
    Board::chop_tail(board);

    // Spawn Fruit if none are pressent
    Board::spawn_fruit(board);
}