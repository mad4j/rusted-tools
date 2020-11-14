
// zeroc - counts 0 bits


use std::fs::File;
use std::io::{BufReader, Read, Result};
use structopt::StructOpt;


#[derive(Debug, StructOpt)]
#[structopt(name = "zeroc", about = "counts zero bits in a file")]
struct Opt {

    /// File to be analized
    #[structopt(short, long)]
    path: String,

    /// Detailed information output
    #[structopt(short, long)]
    verbose: bool,
}


fn main() -> Result<()> {

    //parse command-line parameters
    let opt = Opt::from_args();

    //open file
    let input = File::open(opt.path)?;

    //pass a reference so the ownership remains here
    let reader = BufReader::with_capacity(1024, &input);

    //count total number of bits in file
    let bits: u64 = 8 * input.metadata()?.len();

    //count zero bits in file
    let zeros: u64 = reader.bytes()
                        .fold(0, |zeros, byte| zeros + byte
                                                        .unwrap_or_default()
                                                        .count_zeros() as u64);

    //compute zero bits ratio
    let ratio = zeros as f32 / bits as f32;

    //report results
    if opt.verbose {
        println!("{} / {} ({:.2})", zeros, bits, ratio);
    } else {
        println!("{}", zeros);
    }

    //return success 
    Ok(())
}
