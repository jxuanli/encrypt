use std::fs::{self, metadata, File};
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
        let curr_str = &curr.as_path().display().to_string()[..];
        if should_include(curr_str) {
            let temp = metadata(curr_str).unwrap();
            if temp.is_dir() {
                all_files(files, &curr);
            } else {
                files.push(curr);
            }
        }
    }
}

fn should_include(file: &str) -> bool {
    let to_exclude = vec![
        ".\\.git",
        ".\\src",
        ".\\target",
        ".\\.gitignore",
        ".\\Cargo.lock",
        ".\\Cargo.toml",
    ];
    !to_exclude.contains(&file)
}

fn write(data: &[u8; 10]) -> Result<(), std::io::Error> {
    let mut pos = 0;
    let mut buffer = File::create("foo.txt")?;
    Ok(while pos < data.len() {
        let bytes_written = buffer.write(&data[pos..])?;
        pos += bytes_written;
    })
}
