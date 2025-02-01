use embedded_graphics::{
    image::{Image, ImageRaw},
    prelude::*,
};

use crate::game::{DisplayType, ImgRawType};

// 'bird', WxH Pixel = 12 x 8 px
const SPRITE_BIRD: [u8; 16] = [
    0x1f, 0x80, 0x22, 0x40, 0x44, 0xa0, 0x74, 0x20, 0x8b, 0xf0, 0x4a, 0x10, 0x31, 0xe0, 0x1f,
    0x80,
    // 0xe0, 0x70, 0xdd, 0xb0, 0xbb, 0x50, 0x8b, 0xd0, 0x74, 0x00, 0xb5, 0xe0, 0xce, 0x10, 0xe0, 0x70,
];
const RAW_BIRD: ImgRawType = ImageRaw::new(&SPRITE_BIRD, 12);

#[derive(Debug, PartialEq)]
pub enum PlayerState {
    Flying,
    Jumping,
    Falling,
}

#[derive(Debug)]
pub struct Player {
    pub img: Image<'static, ImgRawType>,
    pub state: PlayerState,
    pub velocity: u32,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        let position = Point::new(x, y);
        let img = Image::new(&RAW_BIRD, position);
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
