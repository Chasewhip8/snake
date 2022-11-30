use pancurses::Input::Character;
use pancurses::Window;

use crate::control::Control::{DOWN, LEFT, RIGHT, UP};

#[derive(PartialEq, Clone, Copy)]
pub enum Control {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

impl Control {
    pub fn from(c: char) -> Option<Control> {
        match c {
            'w' | 'i' => Some(UP),
            'a' | 'j' => Some(LEFT),
            's' | 'k' => Some(DOWN),
            'd' | 'l' => Some(RIGHT),
            _ => None
        }
    }

    pub fn opposite(control: Control) -> Control {
        match control {
            UP => DOWN,
            DOWN => UP,
            RIGHT => LEFT,
            LEFT => RIGHT
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