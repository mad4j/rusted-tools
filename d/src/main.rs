use chrono::{Datelike, Utc};

fn main() {

    let now = Utc::now();

    print!(
        "{:04}{:02}{:02}",
        now.year(),
        now.month(),
        now.day()
    );
}
