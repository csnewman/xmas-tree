use hsl::HSL;
use crate::{Led, Pattern, Strip};

pub struct Rows {
    strip: Strip,
}

impl Rows {
    pub fn new() -> Self {
        Self {
            strip: Strip::new(),
        }
    }
}

const ROWS: [[u32; 2]; 9] = [
    [0, 25],
    [26, 41],
    [42, 55],
    [56, 73],
    [74, 83],
    [84, 91],
    [92, 95],
    [96, 97],
    [98, 99],
];

impl Pattern for Rows {
    fn init(&mut self) {
        println!("Starting rows");
        self.update(0.0, 0.0);
    }

    fn update(&mut self, time: f64, dt: f64) -> bool {

        // time.floor() as usize

        let row = ROWS[ ((time * 10.0).floor() as usize) % ROWS.len()];

        let from = row[0];
        let to = row[1] + 1;

        for lid in 0..self.strip.count() {
            let state = lid >= from && lid < to;

            if state {
                self.strip.set_hsl(lid, HSL {
                    h: (time * 100.0) % 360.0,
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

        time < 4.0
        // true
    }

    fn strip(&mut self) -> &Strip {
        &self.strip
    }
}