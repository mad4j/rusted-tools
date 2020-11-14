
// zeroc - counts 0 bits


use std::fs::File;
use std::io::{BufReader, Read, Result};
use structopt::StructOpt;


#[derive(Debug, StructOpt)]
#[structopt(name = "zeroc", about = "counts zero bits")]
struct Opt {

    /// File to be analized
    #[structopt(short, long)]
    path: String,

    /// Detailed information output
    #[structopt(short, long)]
    verbose: bool
}


fn main() -> Result<()> {

    let opt = Opt::from_args();

    let input = File::open(opt.path)?;
    let mut reader = BufReader::new(input);

    let mut zero_counter: u64 = 0;
    let mut bit_counter: u64 = 0;

    let mut buffer = [0; 1024];

    loop {
        let i = reader.read(&mut buffer)?;
        if i == 0 {
            break;
        }

        bit_counter += 8*(i as u64);

        for b in 0..i {
            zero_counter += buffer[b].count_zeros() as u64;
        }
    }

    let ratio = zero_counter as f32 / bit_counter as f32;

    if opt.verbose {
        println!("{} / {} ({:.2})", zero_counter, bit_counter, ratio);
    } else {
        println!("{}", zero_counter);
    }

    Ok(())
}
