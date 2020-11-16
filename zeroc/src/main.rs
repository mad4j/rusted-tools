// zeroc - counts 0 bits

use std::fs::File;
use std::io::{BufReader, Read, Result};
use structopt::StructOpt;

use colored::Colorize;

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
    // parse command-line parameters
    let opt = Opt::from_args();

    // open file or return error
    let input = File::open(opt.path)?;

    // pass a reference so the ownership remains here
    let reader = BufReader::with_capacity(1024, &input);

    // count zero bits in file
    let zeros: u64 = reader
        .bytes()
        .map(|x| x.unwrap_or_default())
        .map(|x| x.count_zeros() as u64)
        .sum();

    // count total number of bits in file
    let bits: u64 = 8 * input.metadata()?.len();

    // compute zero bits ratio
    let ratio: f32 = zeros as f32 / bits as f32;

    // report results
    let report = if opt.verbose {
        format!(
            "{} / {} ({})",
            format!("{}", zeros,),
            format!("{}", bits),
            format!("{:6.2} %", 100_f32 * ratio).bold()
        )
        .yellow()
    } else {
        format!("{}", zeros).yellow().bold()
    };
    println!("{}", report);

    Ok(())
}
