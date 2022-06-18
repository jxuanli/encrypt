use std::io::prelude::*;
use std::fs::{self, File};

fn main() -> std::io::Result<()> {
    let data = b"some bytes";

    let mut pos = 0;
    let mut buffer = File::create("foo.txt")?;

    while pos < data.len() {
        let bytes_written = buffer.write(&data[pos..])?;
        pos += bytes_written;
    }

    
    let paths = fs::read_dir("./").unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
    Ok(())
}