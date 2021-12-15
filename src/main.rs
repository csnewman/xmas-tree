mod rainbow;
mod spiral;
mod spin;
mod rows;
mod twinkle;

use std::{env, thread};
use rs_ws281x::{ChannelBuilder, Controller, ControllerBuilder, StripType};
use std::time::{Duration, Instant};
use hsl::HSL;
use crate::twinkle::Twinkle;
use crate::rainbow::Rainbow;
use crate::rows::Rows;
use crate::spin::Spin;
use crate::spiral::Spiral;

pub trait Pattern {
    fn init(&mut self);

    fn update(&mut self, time: f64, dt: f64) -> bool;

    fn strip(&mut self) -> &Strip;
}

#[derive(Copy, Clone)]
pub struct Led {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Led {
    pub fn mul(&self, by: f64) -> Led {
        Led {
            r: (self.r as f64 * by) as u8,
            g: (self.g as f64 * by) as u8,
            b: (self.b as f64 * by) as u8
        }
    }
}

impl From<HSL> for Led {
    fn from(hsl: HSL) -> Self {
        let vals = hsl.to_rgb();
        Led {
            r: vals.0,
            g: vals.1,
            b: vals.2,
        }
    }
}

#[derive(Clone)]
pub struct Strip {
    pub leds: [Led; 100],
}

impl Strip {
    pub fn new() -> Self {
        Self {
            leds: [Led {
                r: 0,
                g: 0,
                b: 0,
            }; 100]
        }
    }

    pub fn count(&self) -> u32 {
        self.leds.len() as u32
    }

    pub fn set(&mut self, pos: u32, led: Led) {
        self.leds[pos as usize] = led;
    }

    pub fn set_hsl(&mut self, pos: u32, hsl: HSL) {
        let vals = hsl.to_rgb();
        self.set(pos, Led {
            r: vals.0,
            g: vals.1,
            b: vals.2,
        })
    }

    pub fn get(&self, pos: u32) -> Led {
        self.leds[pos as usize]
    }

    pub fn get_hsl(&self, pos: u32) -> HSL {
        let led = self.get(pos);
        HSL::from_rgb(&[led.r, led.g, led.b])
    }
}

pub struct Driver {
    controller: Controller,
    patterns: Vec<Box<dyn Pattern>>,
}

impl Driver {
    pub fn new() -> Self {
        let controller = ControllerBuilder::new()
            .freq(800_000)
            .dma(10)
            .channel(0,
                     ChannelBuilder::new()
                         .pin(18)
                         .count(100)
                         .strip_type(StripType::Ws2811Bgr)
                         .brightness(255)
                         .build(),
            )
            .channel(1, ChannelBuilder::new().build())
            .build().unwrap();

        Self {
            controller,
            patterns: vec![],
        }
    }

    pub fn add_pattern<P>(&mut self, pat: P) where P: Pattern + 'static {
        self.patterns.push(Box::new(pat))
    }

    pub fn run(self) {
        let mut patterns = self.patterns;
        let mut controller = self.controller;


        let mut last_strip = Strip::new();

        loop {
            for pattern in &mut patterns {
                pattern.init();

                fade(&mut controller, pattern.strip(), &last_strip);

                let start_time = Instant::now();
                let mut last_run = Instant::now();

                let mut con = true;
                while con {
                    let now = Instant::now();
                    let t = now.duration_since(start_time).as_secs_f64();
                    let dt = now.duration_since(last_run).as_secs_f64();
                    last_run = now;

                    con = pattern.update(t, dt);
                    draw(&mut controller, pattern.strip());
                }

                last_strip = pattern.strip().clone();
            }
        }
    }
}

pub fn draw(controller: &mut Controller, strip: &Strip) {
    let output = controller.leds_mut(0);
    for lid in 0..strip.count() {
        let led = &strip.leds[lid as usize];
        output[lid as usize] = [led.r, led.g, led.b, 0];
    }

    controller.render().unwrap();
    controller.wait().unwrap();
}

pub fn fade(controller: &mut Controller, new_strip: &Strip, last_strip: &Strip) {
    let mut strip = Strip::new();
    let start_time = Instant::now();
    let duration = 1.0f64;

    loop {
        let now = Instant::now();
        let t = now.duration_since(start_time).as_secs_f32() as f64;

        if t >= duration {
            return;
        }

        let p = t / duration;

        for lid in 0..strip.count() {
            // let hsl1 = last_strip.get_hsl(lid);
            // let hsl2 = new_strip.get_hsl(lid);
            //
            // strip.set_hsl(lid, HSL {
            //     h: interp(hsl1.h, hsl2.h, p),
            //     s: interp(hsl1.s, hsl2.s, p),
            //     l: interp(hsl1.l, hsl2.l, p),
            //     // s: 1.0,
            //     // l: 0.5,
            // });
            let l1 = last_strip.get(lid);
            let l2 = new_strip.get(lid);
            strip.set(lid, Led {
                r: interpu8(l1.r, l2.r, p),
                g: interpu8(l1.g, l2.g, p),
                b: interpu8(l1.b, l2.b, p),
            });
        }

        draw(controller, &strip);
    }
}

fn interp(a: f64, b: f64, t: f64) -> f64 {
    a * (1.0 - t) + b * t
}

fn interpu8(a: u8, b: u8, t: f64) -> u8 {
    (a as f64 * (1.0 - t) + b as f64 * t) as u8
}

fn main() {
    println!("Xmas tree");
    println!("{:?}", env::current_dir().unwrap());

    let mut driver = Driver::new();
    driver.add_pattern(Rainbow::new());
    driver.add_pattern(Spiral::new());
    driver.add_pattern(Rows::new());
    driver.add_pattern(Spin::new());
    driver.run();
}
