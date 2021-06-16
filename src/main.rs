mod cartridge;
mod processor;

use processor::Processor;
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
    let mut cpu = Processor::build();
    cpu.load_cartridge(&cartridge);

    // 1 Mhz clock
    loop {
        cpu.cycle();
        sleep(Duration::from_nanos(1000));
    }
}
