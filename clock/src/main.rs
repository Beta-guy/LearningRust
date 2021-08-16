use std::time::Duration;
use std::thread::sleep;
use std::io;
use std::io::Write;
use chrono::prelude::*;

fn main() {
    loop {
        std::process::Command::new("clear").status().unwrap();
        let utc: DateTime<Local> = Local::now();
        println!("{}", utc.format("%T"));
        io::stdout().flush().unwrap();
        sleep(Duration::new(1, 0));
    }

}
