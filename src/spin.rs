use hsl::HSL;
use crate::{Led, Pattern, Strip};

pub struct Spin {
    strip: Strip,
}

impl Spin {
    pub fn new() -> Self {
        Self {
            strip: Strip::new(),
        }
    }
}

const ROWS: [[u32; 3]; 9] = [
    [0, 25, 13],
    [26, 41, 11],
    [42, 55, 9],
    [56, 73, 12],
    [74, 83, 8],
    [84, 91, 6],
    [92, 95, 3],
    [96, 97, 0],
    [98, 99, 1],
];


impl Pattern for Spin {
    fn init(&mut self) {
        println!("Starting spin");
        self.update(0.0, 0.0);
    }

    fn update(&mut self, time: f64, dt: f64) -> bool {
        for r in 0..ROWS.len() {
            let row = ROWS[r];
            let from = row[0];
            let to = row[1] + 1;
            let range = to - from;

            for lid in from..to {
                let i = lid - from;
                let ni = (range + i - row[2]) % range;
                let val = ni as f64 / range as f64;

                let current = val - 0.5;
                let target = ((time / 1.0) % 1.0) - 0.5;

                let mut diff = target - current;
                if diff < -0.5 {
                    diff += 1.0;
                }
                if diff > 0.5 {
                    diff -= 1.0;
                }

                diff += 0.5;

                let d = ( (1.0 - diff) - 0.75).max(0.0) / 0.25;

                self.strip.set(lid, Led::from(HSL {
                    h: (d * 100.0 + time * 10.0) % 360.0,
                    s: 1.0,
                    l: 0.5,
                }).mul(1.0 - diff));
            }
        }

        time < 8.0
    }

    fn strip(&mut self) -> &Strip {
        &self.strip
    }
}