use core::sync::atomic::{AtomicBool, Ordering};

use crate::obstacle::Obstacles;
use crate::player::Player;
use embassy_time::{Duration, Timer};
use embedded_graphics::prelude::*;
use embedded_graphics::{image::ImageRaw, pixelcolor::BinaryColor};
use esp_hal::{i2c::master::I2c, rng::Rng};
use ssd1306::{
    mode::BufferedGraphicsModeAsync, prelude::I2CInterface, size::DisplaySize128x64, Ssd1306Async,
};
pub type ImgRawType = ImageRaw<'static, BinaryColor>;

pub type DisplayType<'a> = Ssd1306Async<
    I2CInterface<I2c<'a, esp_hal::Async>>,
    DisplaySize128x64,
    BufferedGraphicsModeAsync<DisplaySize128x64>,
>;

pub static IS_JUMPING: AtomicBool = AtomicBool::new(false);

const GRAVITY: i32 = 2;
const FLAP_STRENGTH: i32 = -6;

#[derive(PartialEq)]
pub enum GameState {
    Menu,
    Playing,
    GameOver,
}

pub struct Game<'a> {
    state: GameState,
    score: u32,
    player: Player,
    pub obstacles: Obstacles,
    // balls: Vec<Ball, MAX_BALLS>,
    display: DisplayType<'a>,
    rng: Rng,
}

impl<'a> Game<'a> {
    pub fn new(display: DisplayType<'a>, rng: Rng) -> Self {
        Self {
            state: GameState::Menu,
            score: 0,
            player: Game::init_player(),
            obstacles: Obstacles::new(rng),
            // balls,
            display,
            rng,
        }
    }

    pub fn init_player() -> Player {
        Player::new(10, 10)
    }

    pub async fn start(&mut self) {
        loop {
            self.display.clear(BinaryColor::Off).unwrap();
            self.obstacles.update();

            self.player.draw(&mut self.display);

            // let obstacle = Obstacle::new(&obstacle::RAW_PIPE_TOP, 30, -40);
            // obstacle.draw(&mut self.display);

            // let obstacle = Obstacle::new(&obstacle::RAW_PIPE_BOTTOM, 30, 40);

            let mut player_velocity = GRAVITY;
            if IS_JUMPING.swap(false, Ordering::Relaxed) {
                player_velocity = FLAP_STRENGTH;
            }
            self.player.update(Point::new(0, player_velocity));

            // For debugging, reset the falling
            if self.player.img.bounding_box().top_left.y > 60 {
                self.player.img = self.player.img.translate(Point::new(0, -70));
            }

            // obstacle.draw(&mut self.display);
            self.obstacles.draw_obstacles(&mut self.display);

            self.display.flush().await.unwrap();
            Timer::after(Duration::from_millis(33)).await;
        }
    }
}
