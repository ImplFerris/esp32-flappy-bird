use embedded_graphics::{image::Image, prelude::*};
use esp_hal::rng::Rng;
use heapless::spsc::Queue;

use crate::game::DisplayType;

#[derive(Debug)]
pub struct Obstacle {
    pub top: Image<'static, super::sprites::ImgRawType>,
    pub bottom: Image<'static, super::sprites::ImgRawType>,
}

impl Obstacle {
    pub fn new(x: i32, y: i32) -> Self {
        let top = Image::new(&super::sprites::RAW_PIPE_TOP, Point::new(x, y));
        let bottom_pos = y + PIPE_HEIGHT as i32 + OBSTALCE_VERTICAL_GAP;

        let bottom = Image::new(&super::sprites::RAW_PIPE_BOTTOM, Point::new(x, bottom_pos));

        Self { top, bottom }
    }

    pub fn draw(&self, display: &mut DisplayType) {
        self.top.draw(display).unwrap();
        self.bottom.draw(display).unwrap();
    }

    /// If the velocity is negative (in our case), the obstacle moves to the left.
    pub fn move_by_velocity(&mut self, velocity: i32) {
        // self.x += velocity;
        // self.img = Image::new(&RAW_CACTUS1, Point::new(self.x, self.y));
        // self.x += velocity;
        // println!("X: {}", self.x);
        self.top = self.top.translate(Point::new(velocity, 0));
        self.bottom = self.bottom.translate(Point::new(velocity, 0));
    }
}

const MAX_OBSTACLES: usize = 4;
const OBSTALCE_VERTICAL_GAP: i32 = 30;
const PIPE_HEIGHT: usize = 60;
pub const OBSTACLE_VELOCITY: i32 = -2; // obstacles moving left side, so it is X-velocity
const OBSTALCE_HORIZONTAL_GAP: i32 = 50;

pub struct Obstacles {
    rng: Rng,
    pub buffer: Queue<Obstacle, MAX_OBSTACLES>,
    display_width: i32,
}

impl Obstacles {
    pub fn new(mut rng: Rng, display_width: i32) -> Self {
        let mut buffer = Queue::new();

        for i in 0..MAX_OBSTACLES - 1 {
            let obs_y = Obstacles::get_rand_y(&mut rng);
            let offset = OBSTALCE_HORIZONTAL_GAP * i as i32;
            buffer
                .enqueue(Obstacle::new(display_width + offset, obs_y))
                .unwrap();
        }

        Obstacles {
            rng,
            buffer,
            display_width,
        }
    }

    pub fn update(&mut self) -> bool {
        for obstacle in self.buffer.iter_mut() {
            obstacle.move_by_velocity(OBSTACLE_VELOCITY);
        }

        let mut new_obs = false;
        if let Some(first) = self.buffer.peek() {
            if first.top.bounding_box().top_left.x < 0 {
                new_obs = true;
                // Remove the first obstacle and add a new one at the end
                self.buffer.dequeue();
                let obs_y = Obstacles::get_rand_y(&mut self.rng);
                self.buffer
                    .enqueue(Obstacle::new(
                        self.display_width + OBSTALCE_HORIZONTAL_GAP,
                        obs_y,
                    ))
                    .ok();
            }
        }
        new_obs
    }

    fn get_rand_y(rng: &mut Rng) -> i32 {
        let y = -(PIPE_HEIGHT as i32 / 2 + 20);
        let rand_num = (rng.random() % 20).max(1);
        y + rand_num as i32
    }

    pub fn draw_obstacles(&mut self, display: &mut DisplayType) {
        for obs in self.buffer.iter() {
            obs.draw(display);
        }
    }
}
