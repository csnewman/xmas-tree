use hsl::HSL;
use rand::Rng;
use rand::rngs::ThreadRng;
use crate::{Led, Pattern, Strip};

pub struct Twinkle {
    strip: Strip,
    rng: ThreadRng,
    power: [f64; 100],
    acc: [f64; 100],
    hue: [f64; 100]
}

impl Twinkle {
    pub fn new() -> Self {
        Self {
            strip: Strip::new(),
            rng: rand::thread_rng(),
            power: [0.0; 100],
            acc: [0.0; 100],
            hue: [0.0; 100]
        }
    }
}

const DIFF: f64 = 4.0;

impl Pattern for Twinkle {
    fn init(&mut self) {
        println!("Starting Twinkle");
        for lid in 0..self.strip.count() {
            let ptr = lid as usize;
            self.acc[ptr] = self.rng.gen::<f64>();
        }

        self.update(0.0, 0.0);
    }

    fn update(&mut self, time: f64, dt: f64) -> bool {

        for lid in 0..self.strip.count() {
            let ptr = lid as usize;

            if self.power[ptr] > 0.0 {
                self.power[ptr] -= DIFF * dt;
            } else {
                self.power[ptr] = 0.0;
                self.acc[ptr] += dt;

                while self.acc[ptr] >= 1.0 {
                    if self.rng.gen::<f64>() > 0.6 {
                        self.power[ptr] = std::f64::consts::PI;
                        self.hue[ptr] = self.rng.gen::<f64>();
                        break;
                    }

                    self.acc[ptr] -= 1.0;
                }

                // self.acc[ptr] += self.rng.gen::<f64>() * dt;
                // if self.acc[ptr] > 0.998 {
                //     self.power[ptr] = std::f64::consts::PI;
                //     self.acc[ptr] = 0.0;
                // } else {
                //     self.power[ptr] = 0.0;
                // }
            }

            self.strip.set(lid, Led::from(HSL {
                h: self.hue[ptr] * 360.0,
                s: 1.0,
                l: 0.5,
            }).mul(self.power[ptr].sin()));
        }

        time < 6.0
    }

    fn strip(&mut self) -> &Strip {
        &self.strip
    }
}