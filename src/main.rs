extern crate sdl2;

mod cartridge;
mod processor;

use processor::Processor;
use sdl2::pixels::Color;
use std::env;
use std::io::{Error, ErrorKind};
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(Error::new(
            ErrorKind::Other,
            "A path to a ch8 file must be provided.",
        ));
    }

    let filename = &args[1];

    let cartridge = cartridge::load_from_file(filename)?;
    let mut cpu = Processor::new();
    cpu.load_cartridge(&cartridge);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Chip-8 Emulator", 640, 320)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();
    // 1 Mhz clock
    loop {
        cpu.cycle();
        sleep(Duration::from_nanos(1000));
    }
}
