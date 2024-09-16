use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs::OpenOptions;
use std::io::Write;
use read_input::prelude::*;


fn main() {
    let cap = vec!["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"];
    let low = vec!["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"];
    let num = vec!["0" ,"1", "2", "3", "4", "5", "6", "7", "8","9"];
    let spe = vec!["!", "@", "#", "$", "%", "^", "&", "*", "(", ")"];
    let mut chars = Vec::new();
    chars.extend(cap);
    chars.extend(low);
    chars.extend(num);
    chars.extend(spe);
    let mut rng = thread_rng();
    let password_length = 12;
    let mut pass = String::new();
    print!("What is the Site or App: ");
    let site_app = input::<String>().get();
    print!("Username or E-mail address: ");
    let account_name = input::<String>().get();
    for _ in 0..password_length {
        let random_char = chars.choose(&mut rng).unwrap();
        pass.push_str(*random_char);
    }
    let csv_line = format!("{},{},{}", site_app, account_name, pass);
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("passwords.csv");

    writeln!(file.expect("unable to save"), "{}", csv_line);
    println!("{:#?}", pass);
}
