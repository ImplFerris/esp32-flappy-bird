use embedded_graphics::{image::Image, prelude::*};

use crate::game::DisplayType;

#[derive(Debug, PartialEq)]
pub enum PlayerState {
    Flying,
    Jumping,
    Falling,
}

#[derive(Debug)]
pub struct Player {
    pub img: Image<'static, super::sprites::ImgRawType>,
    pub state: PlayerState,
    pub velocity: u32,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        let position = Point::new(x, y);
        let img = Image::new(&super::sprites::RAW_BIRD, position);
        Self {
            state: PlayerState::Flying,
            velocity: 0,
            img,
        }
    }

    pub fn draw(&self, display: &mut DisplayType) {
        self.img.draw(display).unwrap();
    }

    pub fn update(&mut self, mut shift_by: Point) {
        let y = self.img.bounding_box().top_left.y;
        if shift_by.y + y <= 0 {
            // make it zero so it won't go outside of the screen
            shift_by.y = -y;
        }
        self.img = self.img.translate(shift_by);
    }
}
