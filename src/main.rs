mod rainbow;

use rs_ws281x::{ChannelBuilder, Controller, ControllerBuilder, StripType};
use std::time::{Instant};
use hsl::HSL;
use crate::rainbow::Rainbow;

pub trait Pattern {
    fn init(&mut self);

    fn update(&mut self, time: f32, dt: f32) -> bool;

    fn strip(&mut self) -> &Strip;
}

#[derive(Copy, Clone)]
pub struct Led {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

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
            b: vals.2
        })
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


        loop {
            for pattern in &mut patterns {
                pattern.init();
                draw(&mut controller, pattern.strip());
                // TODO: Fade in
                // controller.set_brightness(0, 0);

                let start_time = Instant::now();
                let mut last_run = Instant::now();

                let mut con = true;
                while con {
                    let now = Instant::now();
                    let t = now.duration_since(start_time).as_secs_f32();
                    let dt = now.duration_since(last_run).as_secs_f32();
                    last_run = now;

                    con = pattern.update(t, dt);
                    draw(&mut controller, pattern.strip());
                }

                // TODO: Fade out
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

fn main() {
    println!("Xmas tree");

    let mut driver = Driver::new();
    driver.add_pattern(Rainbow::new());
    driver.run();
}
