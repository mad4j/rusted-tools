use chrono::{DateTime, Local};

fn main() {

    let now : DateTime<Local> = Local::now();

    print!("{}", now.format("%Y%m%d"));
}
