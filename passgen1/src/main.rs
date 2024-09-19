use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs::OpenOptions;
use std::io::Write;
use read_input::prelude::*;
use std::env;
use std::io;

// No need to import metadata since we're using it via std::fs::metadata

fn main() {
    let args: Vec<String> = env::args().collect();
    let cap = vec![
        "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R",
        "S", "T", "U", "V", "W", "X", "Y", "Z",
    ];
    let low = vec![
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r",
        "s", "t", "u", "v", "w", "x", "y", "z",
    ];
    let num = vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let spe = vec!["!", "@", "#", "$", "%", "^", "&", "*", "(", ")"];
    let mut chars = Vec::new();
    chars.extend(cap);
    chars.extend(low);
    chars.extend(num);
    chars.extend(spe);
    let mut next = String::from("y"); // Changed to String type

    while next != "n".to_string() {
        if args.len() > 1 {
            next = "n".to_string()
        }
        // Password generation
        let mut rng = thread_rng();
        let password_length = 12;
        let mut pass = String::new();
        for _ in 0..password_length {
            let random_char = chars.choose(&mut rng).unwrap();
            pass.push_str(*random_char);
        }

        // User inputs
        if args.len() > 1 {} else {
            print!("What is the Site or App: ");
        }
        let site_app = if let Some(site_arg) = env::args().nth(1) {
            site_arg
        } else {
            io::stdout().flush().expect("Failed to flush stdout");
            let input_string = input::<String>().get().trim().to_string();
            input_string
        };
        if args.len() > 2 {} else {
            print!("Username or E-mail address: ");
        }
        let account_name = if let Some(account_arg) = env::args().nth(2) {
            account_arg
        } else {
            io::stdout().flush().expect("Failed to flush stdout");
            let input_string = input::<String>().get().trim().to_string();
            input_string
        };

        // Check if file exists
        let file_exists = std::fs::metadata("passwords.csv");

        if file_exists.is_ok() {
            // File exists, open it in append mode
            let file = OpenOptions::new().append(true).open("passwords.csv");

            let mut file = file.expect("Unable to open file");
            let csv_line = format!("{},{},{}", site_app, account_name, pass);
            writeln!(file, "{}", csv_line).expect("Unable to write data");
        } else {
            // File doesn't exist, create it and write headers
            let file = OpenOptions::new()
                .write(true)
                .create(true)
                .open("passwords.csv");

            let mut file = file.expect("Unable to create file");

            // Write headers first
            writeln!(file, "{},{},{}", "Application", "Username", "Password")
                .expect("Unable to write headers");

            // Then write the data line
            let csv_line = format!("{},{},{}", site_app, account_name, pass);
            writeln!(file, "{}", csv_line).expect("Unable to write data");

            // Prompt to continue
            if args.len() > 1 {

            } else {
                print!("Continue? (y/n): ");
            }
            next = if args.len() > 1 {
                "n".to_string()
            } else {
                io::stdout().flush().expect("Failed to flush stdout");
                let input_string = input::<String>().get().trim().to_string();
                input_string
            }
        }
    }
}
