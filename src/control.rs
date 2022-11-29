use pancurses::Input::Character;
use pancurses::Window;

use crate::control::Control::{DOWN, LEFT, RIGHT, UP};

pub enum Control {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

impl Control {
    pub fn from(c: char) -> Option<Control> {
        match c {
            'w' => Some(UP),
            'a' => Some(LEFT),
            's' => Some(DOWN),
            'd' => Some(RIGHT),
            _ => None
        }
    }
}

/// Get the latest control or none from the window.
pub fn get_control(window: &Window) -> Option<Control> {
    if let Some(Character(c)) = window.getch() {
        return Control::from(c);
    }
    None
}