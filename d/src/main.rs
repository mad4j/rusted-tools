use chrono::{DateTime, Local};

fn main() {

    let now : DateTime<Local> = Local::now();

    println!("{}", now.format("%Y%m%d"));
}
