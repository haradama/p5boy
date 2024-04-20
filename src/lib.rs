#![no_std]

use gba::{mem_fns::__aeabi_memcpy, prelude::*};

const SCREEN_WIDTH: usize = 240;
const SCREEN_HEIGHT: usize = 160;

static mut FILL_COLOR: Option<Color> = Some(Color::from_rgb(255, 255, 255));
static mut STROKE_COLOR: Option<Color> = Some(Color::from_rgb(0, 0, 0));
static mut STROKE_WEIGHT: u16 = 1;

pub trait App {
    fn setup(&self);
    fn draw(&self);

    fn run(&self) {
        DISPCNT.write(
            DisplayControl::new()
                .with_video_mode(VideoMode::_3)
                .with_show_bg2(true),
        );

        self.setup();
        loop {
            self.draw();
        }
    }
}

pub fn width() -> usize {
    SCREEN_WIDTH
}

pub fn height() -> usize {
    SCREEN_HEIGHT
}

pub fn background(color: Color) {
    for i in 0..width() {
        for j in 0..height() {
            VIDEO3_VRAM.index(i as usize, j as usize).write(color);
        }
    }
}

pub fn fill(color: Color) {
    unsafe {
        FILL_COLOR = Some(color);
    }
}

pub fn no_fill() {
    unsafe {
        FILL_COLOR = None;
    }
}

pub fn stroke(color: Color) {
    unsafe {
        STROKE_COLOR = Some(color);
    }
}

pub fn no_stroke() {
    unsafe {
        STROKE_COLOR = None;
    }
}

pub fn stroke_weight(weight: u16) {
    unsafe {
        STROKE_WEIGHT = weight;
    }
}

pub fn line(x1: u16, y1: u16, x2: u16, y2: u16) {
    let dx = x2 as i32 - x1 as i32;
    let dy = y2 as i32 - y1 as i32;
    let mut x = x1;
    let mut y = y1;
    let mut d = 2 * dy - dx;
    let incr_e = 2 * dy;
    let incr_ne = 2 * (dy - dx);

    while x <= x2 {
        if d <= 0 {
            d += incr_e;
        } else {
            d += incr_ne;
            y += 1;
        }
        x += 1;
        unsafe {
            if let Some(color) = STROKE_COLOR {
                VIDEO3_VRAM.index(x as usize, y as usize).write(color);
            }
        }
    }
}

pub fn rect(x: u16, y: u16, width: u16, height: u16) {
    unsafe {
        if let Some(fill_color) = FILL_COLOR {
            for i in 0..width {
                for j in 0..height {
                    VIDEO3_VRAM
                        .index((x + i) as usize, (y + j) as usize)
                        .write(fill_color);
                }
            }
        }

        if let Some(stroke_color) = STROKE_COLOR {
            for i in 0..STROKE_WEIGHT {
                for j in 0..width {
                    VIDEO3_VRAM
                        .index((x + j) as usize, (y + i) as usize)
                        .write(stroke_color);
                    VIDEO3_VRAM
                        .index((x + j) as usize, (y + height - 1 - i) as usize)
                        .write(stroke_color);
                }
                for j in 0..height {
                    VIDEO3_VRAM
                        .index((x + i) as usize, (y + j) as usize)
                        .write(stroke_color);
                    VIDEO3_VRAM
                        .index((x + width - 1 - i) as usize, (y + j) as usize)
                        .write(stroke_color);
                }
            }
        }
    }
}
