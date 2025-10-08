use std::{
    fs::File,
    io::{self, Read},
};

use crate::memory::Memory;
mod memory;

fn read_file_bytes(path: &str) -> io::Result<Vec<u8>> {
    let mut f = File::open(path)?;
    let mut buffer = Vec::<u8>::with_capacity(f.metadata()?.len() as usize);
    f.read_to_end(&mut buffer)?;

    buffer.foo();
    Ok(buffer)
}

fn main() {
    let mut buffer = include_bytes!("../roms/tetris.gb");
    let mut memory = Memory::with_rom(buffer).unwrap();

    memory.check_checksum().unwrap();
    println!("{:?}", buffer);
}

trait YourMother {
    fn foo(&self);
}

impl YourMother for Vec<u8> {
    fn foo(&self) {
        println!("HAHA i = {:?}", self.len());
    }
}
