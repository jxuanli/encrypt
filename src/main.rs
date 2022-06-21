use aes_gcm_siv::{
    aead::generic_array::{typenum::UInt, GenericArray},
    {
        aead::{
            consts::{B0, B1},
            generic_array::typenum::UTerm,
            Aead, NewAead,
        },
        {Aes256GcmSiv, Key, Nonce},
    },
};
use hex;
use rand;
use sha3::{Digest, Sha3_256};
use std::fs::{self, metadata};
use std::io::{self, Error};
use std::path::PathBuf;
use std::str;

fn main() -> std::io::Result<()> {
    let mut files: Vec<PathBuf> = Vec::new();
    all_files(&mut files, &PathBuf::from(".\\"));
    parse_key(files)?;
    Ok(())
}

fn parse_key(files: Vec<PathBuf>) -> Result<(), Error> {
    Ok(if get_log(0)? == "0" {
        println!("Please create a password");
        let key: [u8; 32] = rand::random();
        let nonce: [u8; 12] = rand::random();
        fs::write(
            ".\\log",
            format!(
                "{},{},{},{}",
                "1",
                hex::encode(key),
                hex::encode(nonce),
                to_sha3()?
            ),
        )?;
        write_all(files, &encrypt)?;
    } else {
        println!("Files have already been encrypted!");
        println!("Please enter your password");
        while to_sha3()? != get_log(3)? {
            println!("Password incorrect, please try again");
        }
        write_all(files, &decrypt)?;
        fs::write(".\\log", format!("{},{},{}", "0", "", ""))?;
        println!("Files have already been decrypted!");
    })
}

fn get_log(index: usize) -> Result<String, Error> {
    let key = String::from_utf8(fs::read(".\\log")?).unwrap();
    let logs: Vec<&str> = key.split(",").collect();
    Ok(logs[index].to_owned())
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
        ".\\log",
        ".\\.vscode",
    ];
    !to_exclude.contains(&file)
}

fn write_all(
    paths: Vec<PathBuf>,
    target: &dyn Fn(&str) -> Result<String, Error>,
) -> Result<(), Error> {
    for path_buf in paths {
        let path = path_buf.to_str().unwrap();
        let data = &String::from_utf8(fs::read(path)?).unwrap();
        println!("writing: {:?}, {:?}", path, target(data));
        fs::write(path, target(data)?)?;
    }
    Ok(())
}

fn encrypt(content: &str) -> Result<String, Error> {
    let ciphertext = &get_cipher()?
        .encrypt(&get_nonce()?, content.as_bytes().as_ref())
        .unwrap();
    Ok(hex::encode(ciphertext))
}

fn decrypt(content: &str) -> Result<String, Error> {
    let plaintext = &get_cipher()?
        .decrypt(&get_nonce()?, hex::decode(content).unwrap().as_ref())
        .unwrap();
    Ok(String::from_utf8(plaintext.to_vec()).unwrap())
}

fn get_nonce() -> Result<GenericArray<u8, UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>>, Error> {
    let _nonce_str = &hex::decode(get_log(2)?).unwrap();
    let nonce = *Nonce::from_slice(_nonce_str);
    Ok(nonce)
}

fn get_cipher() -> Result<Aes256GcmSiv, Error> {
    Ok(Aes256GcmSiv::new(Key::from_slice(
        &hex::decode(get_log(1)?).unwrap(),
    )))
}

fn to_sha3() -> Result<String, Error> {
    let mut pw = String::new();
    io::stdin().read_line(&mut pw)?;
    Ok(hex::encode(
        Sha3_256::new().chain_update(pw.as_bytes()).finalize(),
    ))
}
