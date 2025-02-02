use core::sync::atomic::{AtomicBool, Ordering};

use crate::obstacle::Obstacles;
use crate::player::Player;
use crate::sprites;
use core::fmt::Write;
use embassy_time::{Duration, Timer};
use embedded_graphics::image::Image;
use embedded_graphics::mono_font::ascii::{FONT_5X8, FONT_6X10};
use embedded_graphics::mono_font::MonoTextStyleBuilder;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::text::{Baseline, Text};
use esp_hal::{i2c::master::I2c, rng::Rng};
use heapless::String;
use ssd1306::{
    mode::BufferedGraphicsModeAsync, prelude::I2CInterface, size::DisplaySize128x64, Ssd1306Async,
};

pub type DisplayType<'a> = Ssd1306Async<
    I2CInterface<I2c<'a, esp_hal::Async>>,
    DisplaySize128x64,
    BufferedGraphicsModeAsync<DisplaySize128x64>,
>;

pub static BUTTON_PRESSED: AtomicBool = AtomicBool::new(false);

const GRAVITY: i32 = 2;
const FLAP_STRENGTH: i32 = -6;

#[derive(PartialEq, Clone, Copy)]
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
        let (player, obstacles) = Game::init_game_state(&display, rng);

        Self {
            state: GameState::Menu,
            score: 0,
            player,
            obstacles,
            // balls,
            display,
            rng,
        }
    }

    pub fn init_player() -> Player {
        Player::new(10, 10)
    }

    fn reset_game(&mut self) {
        self.score = 0;
        self.state = GameState::Playing;

        let (player, obstacles) = Game::init_game_state(&self.display, self.rng);
        self.player = player;
        self.obstacles = obstacles;
    }

    fn init_game_state(display: &DisplayType<'a>, rng: Rng) -> (Player, Obstacles) {
        let (screen_width, _) = display.dimensions();
        (
            Game::init_player(),
            Obstacles::new(rng, screen_width as i32),
        )
    }

    pub async fn start(&mut self) {
        let mut title_buff: String<64> = String::new();
        let screen_height = self.display.dimensions().1 as i32;
        let mut prev_state;
        loop {
            title_buff.clear();

            prev_state = self.state;

            match self.state {
                GameState::Menu => {
                    if BUTTON_PRESSED.swap(false, Ordering::Relaxed) {
                        self.reset_game();
                        self.state = GameState::Playing;
                    }
                }
                GameState::Playing => {
                    let mut player_velocity = GRAVITY;
                    if BUTTON_PRESSED.swap(false, Ordering::Relaxed) {
                        player_velocity = FLAP_STRENGTH;
                    }
                    self.player.update(Point::new(0, player_velocity));

                    if self.obstacles.update() {
                        self.score += 1;
                    }

                    self.collison_handle();

                    if self.player.img.bounding_box().top_left.y > screen_height {
                        BUTTON_PRESSED.store(false, Ordering::Relaxed);
                        self.state = GameState::GameOver;
                    }
                }
                _ => {
                    if BUTTON_PRESSED.swap(false, Ordering::Relaxed) {
                        self.state = GameState::Menu;
                    }
                }
            }

            self.clear_display();

            match self.state {
                GameState::Menu => self.draw_title_text("Press to start..."),
                GameState::Playing => self.draw_game(),
                GameState::GameOver => {
                    self.draw_game_over();
                }
            }

            self.display.flush().await.unwrap();
            if prev_state == GameState::Playing && self.state == GameState::GameOver {
                BUTTON_PRESSED.store(false, Ordering::Relaxed);
                // Wait and show the game over screen
                Timer::after(Duration::from_millis(1000)).await;
                BUTTON_PRESSED.store(false, Ordering::Relaxed);
            }

            Timer::after(Duration::from_millis(33)).await;
        }
    }

    pub fn clear_display(&mut self) {
        self.display.clear_buffer();
        self.display.clear(BinaryColor::Off).unwrap();
    }

    pub fn draw_game(&mut self) {
        self.player.draw(&mut self.display);
        self.obstacles.draw_obstacles(&mut self.display);
        self.print_score();
    }

    fn print_score(&mut self) {
        let mut score_text: String<16> = String::new();
        write!(score_text, "Score: {}", self.score).unwrap();

        let text_style = MonoTextStyleBuilder::new()
            .font(&FONT_5X8)
            .text_color(BinaryColor::On)
            .build();

        Text::with_baseline(&score_text, Point::new(0, 0), text_style, Baseline::Top)
            .draw(&mut self.display)
            .unwrap();
    }

    fn draw_title_text(&mut self, title: &str) {
        let text_style = MonoTextStyleBuilder::new()
            .font(&FONT_6X10)
            .text_color(BinaryColor::On)
            .build();

        let text_width = title.len() as i32 * FONT_6X10.character_size.width as i32;
        let text_height = FONT_6X10.character_size.height as i32;

        // Get display dimensions
        let (width, height) = self.display.dimensions();

        // Calculate top-left position to center the text
        let x = (width as i32 - text_width) / 2;
        let y = (height as i32 - text_height) / 2;

        Text::with_baseline(title, Point::new(x, y), text_style, Baseline::Top)
            .draw(&mut self.display)
            .unwrap();
    }

    fn collison_handle(&mut self) {
        for obs in self.obstacles.buffer.iter() {
            if detect_collison(&mut self.player.img, &obs.top)
                || detect_collison(&mut self.player.img, &obs.bottom)
            {
                BUTTON_PRESSED.store(false, Ordering::Relaxed);
                self.state = GameState::GameOver;
                break;
            }
        }
    }

    pub fn draw_game_over(&mut self) {
        let mut score_text: String<32> = String::new();

        Image::new(&super::sprites::RAW_GAME_OVER, Point::new(16, 32))
            .draw(&mut self.display)
            .unwrap();

        write!(score_text, "Score: {}", self.score).unwrap();
        let text_style = MonoTextStyleBuilder::new()
            .font(&FONT_6X10)
            .text_color(BinaryColor::On)
            .build();

        let text_width = score_text.len() as i32 * FONT_6X10.character_size.width as i32;
        // let text_height = FONT_6X10.character_size.height as i32;

        // // Get display dimensions
        let (width, _) = self.display.dimensions();

        // // Calculate top-left position to center the text
        let x = (width as i32 - text_width) / 2;
        // let y = (height as i32 - text_height) / 2;

        Text::with_baseline(&score_text, Point::new(x, 42), text_style, Baseline::Top)
            .draw(&mut self.display)
            .unwrap();
    }
}

pub fn detect_collison(
    a: &mut Image<'static, sprites::ImgRawType>,
    b: &Image<'static, sprites::ImgRawType>,
) -> bool {
    let intersection = a.bounding_box().intersection(&b.bounding_box());

    if intersection.size.width == 0 || intersection.size.height == 0 {
        return false;
    }

    true
}
