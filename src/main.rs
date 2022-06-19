use std::fs::{self, File, metadata};
use std::io::prelude::*;
use std::path::PathBuf;
// use sha3::{Digest, Sha3_256};

fn main() -> std::io::Result<()> {
    let data = b"some bytes";
    write(data)?;
    let mut files: Vec<PathBuf> = Vec::new();
    all_files(&mut files, &PathBuf::from(".\\"));

    for path in files {
        println!("{}", path.display())
    }
    Ok(())
}

fn all_files(files: &mut Vec<PathBuf>, file: &PathBuf) {
    let paths = fs::read_dir(file).unwrap();  
    for path in paths {
        let curr = path.unwrap().path();
        let temp = metadata(&curr.as_path().display().to_string()[..]).unwrap();
        if temp.is_dir() {
            all_files(files, &curr);
        } else {
            files.push(curr);
        }
    }
}

fn write(data: &[u8; 10]) -> Result<(), std::io::Error> {
    let mut pos = 0;
    let mut buffer = File::create("foo.txt")?;
    Ok(while pos < data.len() {
        let bytes_written = buffer.write(&data[pos..])?;
        pos += bytes_written;
    })
}
