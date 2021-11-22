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

const FLIPS: [bool; 9] = [
    false,
    true,
    false,
    true,
    false,
    true,
    false,
    true,
    false,
];

const CR: usize = 2;


impl Pattern for Rows {
    fn init(&mut self) {
        println!("Starting rows");
        self.update(0.0, 0.0);
    }

    fn update(&mut self, time: f32, dt: f32) -> bool {

        // time.floor() as usize

        let v = (time).floor() as usize;

        for r in 0..ROWS.len() {
            let row = ROWS[r];
            let from = row[0];
            let to = row[1] + 1;

            let range = to - from;

            // let state =  ((v >> r) & 1) == 1;


            for lid in from..to {
                let i = lid - from;

                // let mut val = ((((i + row[2]) % range) as f64) / (range as f64));
                // if FLIPS[r] {
                //     val = 1.0 - val;
                // }

                // let val = if FLIPS[r] {
                //     ((((range + row[2] - i) % range) as f64) / (range as f64))
                // } else {
                //     ((((i + row[2]) % range) as f64) / (range as f64))
                // };

                let hr = range / 2;

                let fi = if FLIPS[r] { range - 1 - i } else { i };
                let cent = if FLIPS[r] { range - 1 - row[2] } else { row[2] };

                let mut val = ((((range + fi + cent - hr) % range) as f64) / (range as f64));
                // if FLIPS[r] {
                //     val = 1.0 - val;
                // }

                //FLIPS
                // let state = val <= 0.25;
                // let state = true;

                let ni = (range + i - row[2]) % range;
                let val = ni as f64 / range as f64;
                // let state = val >= 0.25 && val <= 0.5;

                // let

                let current = val - 0.5;
                let target = (time as f64 % 1.0) - 0.5;

                let mut diff = target - current;
                if diff < -0.5 {
                    diff += 1.0;
                }
                if diff > 0.5 {
                    diff -= 1.0;
                }

                diff += 0.5;

                let d = ( (1.0 - diff) - 0.75).max(0.0) / 0.25;

                // let d1 = val - target;

                // let mut diff = val - (time as f64 % 1.0);

                // diff /= 10.0;
                // if diff < 0.0 {
                //     diff = 1.0 -diff;
                // }

                // self.strip.set(lid, Led {
                //     r: (255 as f64 * d) as u8,
                //     g: 0,
                //     b: 0
                //     // g: if r == CR { 255 } else { 0 },
                //     // b: if r == CR { 255 } else { 0 },
                // });

                let rgb = HSL {
                    h: (diff * 100.0 + (time * 10.0) as f64) % 360.0,
                    s: 1.0,
                    l: 0.5,
                }.to_rgb();

                self.strip.set(lid, Led {
                    r: (rgb.0 as f64 * (d)) as u8,
                    g: (rgb.1 as f64 * (d)) as u8,
                    b: (rgb.2 as f64 * (d)) as u8
                });


                // let state = ni <= range / 4;

                // if state /*&& r <= CR*/ {
                //     // self.strip.set_hsl(lid, HSL {
                //     //     h: (((i as f64) / (range as f64)) * 360.0 * 0.5 + (time * 100.0) as f64) % 360.0,
                //     //     s: 1.0,
                //     //     l: 0.5,
                //     // });
                //     self.strip.set(lid, Led {
                //         r: 100,
                //         // g: 0,
                //         // b: 0
                //         g: if r == CR { 255 } else { 0 },
                //         b: if r == CR { 255 } else { 0 },
                //     });
                // } else {
                //     self.strip.set(lid, Led {
                //         r: 0,
                //         g: 0,
                //         b: 0,
                //     });
                // }
            }
        }

        // let row = ROWS[ ((time * 10.0).floor() as usize) % ROWS.len()];

        time < 8.0
        // true
    }

    fn strip(&mut self) -> &Strip {
        &self.strip
    }
}