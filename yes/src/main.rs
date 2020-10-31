
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "yes", about = "continously output 'yes' on standard output")]
struct Opt {
    #[structopt(default_value = "y", short, long)]
    command : String
}

fn main() {

    let opt = Opt::from_args();

    loop {
        println!("{}", opt.command);
    }
}
