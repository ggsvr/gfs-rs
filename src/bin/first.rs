use std::time;

use minifb::{Key, Window, WindowOptions};
use engine::{color::*, buffer::*};

const WIDTH: usize = 600;
const HEIGHT: usize = 400;

const MAX_FPS: u32 = 60;

const TITLE: &str = "minifb test";

const OPTIONS: WindowOptions = WindowOptions {
    borderless: false,
    title: true,
    resize: true,
    scale: minifb::Scale::X1,
    scale_mode: minifb::ScaleMode::AspectRatioStretch,
    topmost: false,
    transparency: false,
    none: false,
};

fn main() {

    let (mut buffer, mut window) = init();


    while window.is_open() && !window.is_key_down(Key::Escape) {

        buffer.blit(50, -50, rgb(255, 255, 255));

        update(&mut window, &mut buffer).expect("deu ruim");
    }
}

fn update(window: &mut Window, buffer: &mut Buffer) -> Result<(), minifb::Error> {

    window.update_with_buffer(buffer.buffer(), buffer.width(), buffer.height())
}

fn init() -> (Buffer, Window) {
    let buffer = Buffer::new(WIDTH, HEIGHT);
    let mut window = Window::new(TITLE, WIDTH, HEIGHT, OPTIONS).expect("Could not create window."); 
    window.limit_update_rate(Some(time::Duration::from_secs_f64(1.0 / MAX_FPS as f64)));
    (buffer, window)
}