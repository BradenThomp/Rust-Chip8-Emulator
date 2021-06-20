use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub enum KeyboardInput {
    Quit,
    Other, // Temporary place holder until keyboard input can be handled.
}

pub struct Keyboard {
    event_pump: sdl2::EventPump,
}

impl Keyboard {
    pub fn new(sdl_context: &sdl2::Sdl) -> Keyboard {
        let event_pump = sdl_context.event_pump().unwrap();
        Keyboard {
            event_pump: event_pump,
        }
    }
    pub fn handle_input(&mut self) -> KeyboardInput {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return KeyboardInput::Quit,
                _ => (),
            }
        }
        KeyboardInput::Other
    }
}
