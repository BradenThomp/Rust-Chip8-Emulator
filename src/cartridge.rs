use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Error;

pub fn load_from_file(filename: &str) -> Result<[u8; 3584], Error> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);
    let mut buffer = [0x0; 3584];
    reader.read(&mut buffer)?;
    Ok(buffer)
}
