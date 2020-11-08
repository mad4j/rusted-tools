
// zeroc - counts 0 bits

use std::fs::File;
use std::io::{BufReader, Read, Result};

fn main() -> Result<()> {

    let path = "file.txt";

    let input = File::open(path)?;
    let mut reader = BufReader::new(input);

    let mut counter: u32 = 0;
    let mut buffer = [0; 1024];

    loop {
        let i = reader.read(&mut buffer)?;
        if i == 0 {
            break;
        }
        for k in &buffer {
            counter += k.count_zeros();
        }
    }

    println!("{}", counter);

    Ok(())
}


/*

use data_encoding::HEXUPPER;
use ring::digest::{Context, Digest, SHA256};
use std::fs::File;
use std::io::{BufReader, Read, Write, Result};

fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}

fn main() -> Result<()> {
    let path = "file.txt";

    let mut output = File::create(path)?;
    write!(output, "We will generate a digest of this text")?;

    let input = File::open(path)?;
    let reader = BufReader::new(input);
    let digest = sha256_digest(reader)?;

    println!("SHA-256 digest is {}", HEXUPPER.encode(digest.as_ref()));

    Ok(())
}
*/