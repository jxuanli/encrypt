use std::fs::{self, metadata};
use std::path::PathBuf;
use std::str;
use hex;
use aes_gcm_siv::{Aes256GcmSiv, Key, Nonce}; 
use aes_gcm_siv::aead::{Aead, NewAead};
use base64;


fn main() -> std::io::Result<()> {
    let mut files: Vec<PathBuf> = Vec::new();
    all_files(&mut files, &PathBuf::from(".\\"));
    parse_key(files)?;
    Ok(())
}

fn parse_key(files: Vec<PathBuf>) -> Result<(), std::io::Error> {
    let key = String::from_utf8(fs::read(".\\log")?).unwrap();
    let parts: Vec<&str> = key.split(",").collect();
    Ok(if parts[0] == "0" {
        write_all(files, &encrypt)?;
        fs::write(".\\log", format!("{},{}", "1", "an example very very secret key."))?;
    } else {
        println!("Files have already been encrypted!");
        write_all(files, &decrypt)?;
        fs::write(".\\log", format!("{},{}", "0", ""))?;
        println!("Files have already been decrypted!");
    })
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
        ".\\log"
    ];
    !to_exclude.contains(&file)
}

fn write_all(paths: Vec<PathBuf>, target: &dyn Fn(&str)->String) -> Result<(), std::io::Error> {
    for path_buf in paths {
        let path = path_buf.to_str().unwrap();
        let data = &String::from_utf8(fs::read(path)?).unwrap();
        println!("writing: {:?}, {:?}", path, target(data));
        fs::write(path, target(data))?;
    }
    Ok(())
}

fn encrypt(content: &str) -> String {
    let key = b"an example very very secret key.";
    let cipher = Aes256GcmSiv::new(Key::from_slice(key));
    let nonce = Nonce::from_slice(b"unique nonce");
    let ciphertext = cipher.encrypt(nonce, hex::encode(content).as_ref()).expect("encryption failure!");
    base64::encode(ciphertext)
}

fn decrypt(content: &str) -> String {
    println!("{}", content);
    let key = b"an example very very secret key.";
    let cipher = Aes256GcmSiv::new(Key::from_slice(key));
    let nonce = Nonce::from_slice(b"unique nonce");
    let plaintext = cipher.decrypt(nonce, content.as_ref()).expect("decryption failure!");
    base64::encode(plaintext)
}