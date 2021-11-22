use std::thread;
use std::time::Duration;
use hsl::HSL;
use crate::{Pattern, Strip};

pub struct Rainbow {
    strip: Strip,
    counter: u32,
}

impl Rainbow {
    pub fn new() -> Self {
        Self {
            strip: Strip::new(),
            counter: 0,
        }
    }
}

impl Pattern for Rainbow {
    fn init(&mut self) {
        println!("Starting rainbow");
        self.counter = 0;
    }

    fn update(&mut self, time: f32, dt: f32) -> bool {
        println!("Update {} {}", time, dt);

        for lid in 0..self.strip.count() {
            self.strip.set_hsl(lid, HSL {
                h: (((self.counter + lid) as f64 * 2.0) % 360.0) as f64,
                s: 1.0,
                l: 0.5,
            });
        }

        self.counter += 1;

        thread::sleep(Duration::from_millis(20));

        true
    }

    fn strip(&mut self) -> &Strip {
        &self.strip
    }
}