use sha1::{Digest, Sha1};
use std::{
    env,
    error::Error,
    fs,
    io::{self, BufRead, Write},
};

fn decrypt(data: &mut [u8], hash1: &[u8], hash2: &[u8]) {
    for (i, byte) in data.iter_mut().enumerate() {
        *byte = *byte ^ hash2[i % hash2.len()] ^ hash1[i % hash1.len()];
    }
}

fn hash_str(bytes: &[u8]) -> String {
    bytes
        .iter()
        .fold(String::new(), |acc, x| acc + &format!("{:02X}", x))
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().skip(1).collect();

    let files: &[String];
    let password: String;
    match args.len() {
        0 => {
            println!("usage: gmfmod_decrypt <file> | gmfmod_decrypt <pass> <file1> [file2 .. n]");
            return Ok(());
        }
        1 => {
            print!("Password: ");
            password = {
                io::stdout().flush()?;
                let stdin = io::stdin();
                let mut line = String::new();
                stdin.lock().read_line(&mut line)?;
                line.trim().into()
            };
            files = &args[..];
        }
        _ => {
            password = args[0].clone();
            files = &args[1..];
        }
    }

    let mut hasher1 = Sha1::new();
    hasher1.input(password.as_bytes());
    let mut hasher2 = Sha1::new();
    let rev_password: String = password.chars().rev().collect();
    hasher2.input(rev_password.as_bytes());

    let hash1 = hash_str(&hasher1.result());
    let hash2 = hash_str(&hasher2.result());
    println!("Hash1: {}", hash1);
    println!("Hash2: {}", hash2);

    println!(); // This is like freshman code :l
    for filename in files {
        let mut file = fs::read(&filename)?;
        println!("Decrypting '{}'...", filename);
        decrypt(&mut file, hash1.as_bytes(), hash2.as_bytes());
        let out_path = match filename.rfind('.') {
            Some(idx) => format!("{}-decrypted{}", &filename[..idx], &filename[idx..]),
            None => format!("{}-decrypted", filename),
        };
        fs::write(&out_path, &file)?;
    }

    Ok(())
}
