use hsl::HSL;
use crate::{Pattern, Strip};

pub struct Rainbow {
    strip: Strip,
}

impl Rainbow {
    pub fn new() -> Self {
        Self {
            strip: Strip::new(),
        }
    }
}

impl Pattern for Rainbow {
    fn init(&mut self) {
        println!("Starting rainbow");
        self.update(0.0, 0.0);
    }

    fn update(&mut self, time: f64, dt: f64) -> bool {
        for lid in 0..self.strip.count() {
            self.strip.set_hsl(lid, HSL {
                h: (time * 100.0 + (lid as f64 * 2.0)) % 360.0,
                s: 1.0,
                l: 0.5,
            });
        }

        time < 16.0
    }

    fn strip(&mut self) -> &Strip {
        &self.strip
    }
}