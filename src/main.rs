extern crate core;

mod state;
mod control;

use std::time::{Duration, SystemTime, UNIX_EPOCH};
use pancurses::{A_BLINK, A_COLOR, A_NORMAL, Attribute, Attributes, COLOR_BLACK, color_content, COLOR_GREEN, COLOR_PAIR, COLOR_PAIRS, COLOR_RED, COLOR_YELLOW, ColorPair, COLORS, init_pair, initscr, PDC_COLOR_SHIFT, Window};
use pancurses::Attribute::Blink;
use crate::control::get_control;
use crate::state::{Board, Point};

const GAME_SPEED_MS: u64 = 100;

fn main() {
    let frame_ms_time = Duration::from_millis(GAME_SPEED_MS);
    let mut frame_end_time;

    let board = &mut Board::new(78, 24);
    let window = &mut create_window(board);

    loop {
        frame_end_time = SystemTime::now().duration_since(UNIX_EPOCH)
            .unwrap()
            .checked_add(frame_ms_time)
            .unwrap();

        // Capture inputs as they come in and wait until the next frame is needed
        while SystemTime::now().duration_since(UNIX_EPOCH).unwrap() < frame_end_time {
            if let Some(control) = get_control(window) {
                Board::change_control(board, control);
            }
            std::thread::sleep(Duration::from_millis(100));
        }

        // Use the previously captured latest control to execute game logic on
        update_logic(board);

        // Display the game board.
        display_game(board, window);
    }
}

fn create_window(board: &mut Board) -> Window {
    let mut window = initscr();
    window.timeout(10);
    window.nodelay(false);
    window.attrset(Attributes::default() | Attribute::Bold);
    return window;
}

fn display_game(board: &mut Board, window: &Window) {
    window.clear();
    for snake_body in Board::snake(board) {
        window.mvaddch(Point::y(snake_body) as i32, Point::x(snake_body) as i32,'X');
    }
    for fruit in Board::fruit(board) {
        window.mvaddch(Point::y(fruit) as i32, Point::x(fruit) as i32,'O');
    }
    window.refresh();
}

fn update_logic(board: &mut Board){
    Board::check_collision_death(board);
    Board::move_snake(board); // Execute the current move inside board.move_direction
    Board::check_collision_food(board);
    Board::chop_tail(board);
}