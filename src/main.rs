use std::io::prelude::*;
use std::fs::{self, File};

fn main() -> std::io::Result<()> {
    let data = b"some bytes";
    write(data)?;
    let paths = all_files();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
    Ok(())
}

fn all_files() -> fs::ReadDir {
    let temp = fs::read_dir("./").unwrap();
    temp
}

fn write(data: &[u8; 10]) -> Result<(), std::io::Error> {
    let mut pos = 0;
    let mut buffer = File::create("foo.txt")?;
    Ok(while pos < data.len() {
        let bytes_written = buffer.write(&data[pos..])?;
        pos += bytes_written;
    })
}