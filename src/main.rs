extern crate sdl2;

mod cartridge;
mod keyboard;
mod processor;
mod video;

use keyboard::Keyboard;
use keyboard::KeyboardInput;
use processor::Processor;
use std::env;
use std::thread::sleep;
use std::time::Duration;
use video::Video;

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("A path to a valid *.ch8 file must be provided.");
    }

    let filename = &args[1];

    let cartridge = cartridge::load_from_file(filename).expect("Unable to load catridge.");
    let mut cpu = Processor::new();
    cpu.load_cartridge(&cartridge);

    let sdl_context = sdl2::init().unwrap();
    let mut video_out = Video::new(&sdl_context);
    let mut keyboard = Keyboard::new(&sdl_context);
    // 1 Mhz clock
    'running: loop {
        let key_res = keyboard.handle_input();
        match key_res {
            KeyboardInput::Quit => break 'running,
            KeyboardInput::Other => (),
        }
        // Emu Display Logic
        cpu.cycle();
        video_out.update(&cpu.vram);
        sleep(Duration::from_nanos(1000));
    }

    Ok(())
}
