#![no_std]
#![no_main]

use gba::prelude::*;
use visualboy::*;

struct MyApp;

impl App for MyApp {
    fn setup(&self) {
        background(Color::from_rgb(200, 200, 200));
    }

    fn draw(&self) {
        fill(Color::from_rgb(255, 0, 0));
        stroke(Color::from_rgb(0, 0, 255));
        stroke_weight(2);
        rect(50, 50, 100, 60);

        no_fill();
        stroke(Color::from_rgb(0, 255, 0));
        stroke_weight(4);
        rect(100, 80, 80, 40);

        fill(Color::from_rgb(255, 255, 0));
        no_stroke();
        rect(150, 30, 60, 80);
    }
}

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub fn main() -> ! {
    let app = MyApp;
    app.run();
    loop {}
}
