#![allow(dead_code)]

use std::thread::sleep;
use std::time;

use super::button::Pos;
use piston_window::*;
use piston_window::{AdvancedWindow, PistonWindow};

pub fn sin_bounce(target: f64, factor: f64, time_passed: f64) -> Option<f64> {
    if time_passed < 0.0 || time_passed > target * factor {
        None
    } else {
        Some(
            (target - time_passed / factor) * (0.25 * (time_passed / factor + 18.0)).sin() + target,
        )
    }
}

pub fn bounce(window: &mut PistonWindow, pos: Pos, left: bool, speed: i32) {
    if left {
        window.set_position(Position {
            x: (0.0 - window.size().width) as i32,
            y: pos.y as i32,
        });

        for x in 0..(pos.x / speed as f64) as i32 + 5 {
            // move window slowly to target position
            window.set_position(Position {
                x: x * speed,
                y: pos.y as i32,
            });
            sleep(time::Duration::from_nanos(1));
        }
        for x in (pos.x as i32 / speed..pos.x as i32 / speed + 1).rev() {
            // move window slowly to target position
            window.set_position(Position {
                x: x * speed,
                y: pos.y as i32,
            });
            sleep(time::Duration::from_nanos(1));
        }

        for x in pos.x as i32 / speed - 1..pos.x as i32 / speed {
            // move window slowly to target position
            window.set_position(Position {
                x: x * speed,
                y: pos.y as i32,
            });
            sleep(time::Duration::from_nanos(1));
        }
    } else {
        let width = window
            .window
            .ctx
            .window()
            .current_monitor()
            .unwrap()
            .size()
            .width;
        window.set_position(Position {
            x: width as i32,
            y: pos.y as i32,
        });

        for x in (pos.x as i32 / speed - 2..width as i32 / speed).rev() {
            // move window slowly to target position
            window.set_position(Position {
                x: x * speed,
                y: pos.y as i32,
            });
            sleep(time::Duration::from_nanos(1));
        }
        for x in pos.x as i32 / speed - 2..pos.x as i32 / speed + 1 {
            // move window slowly to target position
            window.set_position(Position {
                x: x * speed,
                y: pos.y as i32,
            });
            sleep(time::Duration::from_nanos(1));
        }
        for x in (pos.x as i32..pos.x as i32 + 15).rev() {
            // move window slowly to target position
            window.set_position(Position { x, y: pos.y as i32 });
            sleep(time::Duration::from_nanos(1));
        }
    }
}
