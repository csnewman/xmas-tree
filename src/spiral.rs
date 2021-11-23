use std::thread;
use std::time::Duration;
use hsl::HSL;
use crate::{Led, Pattern, Strip};

pub struct Spiral {
    strip: Strip,
}

impl Spiral {
    pub fn new() -> Self {
        Self {
            strip: Strip::new(),
        }
    }
}

impl Pattern for Spiral {
    fn init(&mut self) {
        println!("Starting spiral");
        self.update(0.0, 0.0);
    }

    fn update(&mut self, time: f64, dt: f64) -> bool {
        let t = 8.0f64;
        let c = 100.0 / (t / 2.0);

        for lid in 0..self.strip.count() {
            let state = if time < (t / 2.0) {
                lid as f64 / c < time
            } else {
                // (self.strip.count()  - 1 - lid) as f64 / c < (time - (t / 2.0))
                (self.strip.count()  - 1 - lid) as f64 / c < (t - time)
            };

            if state {
                self.strip.set_hsl(lid, HSL {
                    h: (360.0 / 100.0) * (lid as f64),
                    s: 1.0,
                    l: 0.5,
                });
            } else {
                self.strip.set(lid, Led {
                    r: 0,
                    g: 0,
                    b: 0
                });
            }
        }

        time < t
    }

    fn strip(&mut self) -> &Strip {
        &self.strip
    }
}