use rand::seq::SliceRandom;
use rand::thread_rng;
use std::env;
use std::fs::OpenOptions;
use std::io::{self, Write};
use read_input::prelude::*; // If you prefer, you can replace this with standard I/O methods

fn main() {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // Character vectors for password generation
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

    // Get the 'special' argument if provided
    let binding = String::new();
    let special = args.get(4).unwrap_or(&binding);

    // Initialize 'chars' vector
    let mut chars = Vec::new();
    chars.extend(&cap);
    chars.extend(&low);
    chars.extend(&num);

    // Conditionally include special characters
    if special.trim().eq_ignore_ascii_case("yes") {
        chars.extend(&spe);
    }

    // Determine if command-line arguments are provided
    if args.len() > 3 {
        // Arguments provided, run once and exit
        let site_app = args.get(1).unwrap().clone();
        let account_name = args.get(2).unwrap_or(&String::new()).clone();

        // Get password length from arguments
        let max: usize = match args.get(3) {
            Some(arg) => match arg.parse() {
                Ok(len) => len,
                Err(_) => {
                    eprintln!("Error: '{}' is not a valid number for password length.", arg);
                    std::process::exit(1);
                }
            },
            None => 12, // Default password length
        };

        // Generate password
        let pass = generate_password(&chars, max);

        // Save data
        save_data(&site_app, &account_name, &pass);

        // Program ends here
    } else {
        // No arguments, enter loop
        let mut next = String::from("y");
        while next != "n" {
            // Generate password
            let pass = generate_password(&chars, 12);

            // Prompt user for inputs
            print!("What is the Site or App: ");
            io::stdout().flush().expect("Failed to flush stdout");
            let site_app = input::<String>().get().trim().to_string();

            print!("Username or E-mail address: ");
            io::stdout().flush().expect("Failed to flush stdout");
            let account_name = input::<String>().get().trim().to_string();

            // Save data
            save_data(&site_app, &account_name, &pass);

            // Prompt to continue
            print!("Continue? (y/n): ");
            io::stdout().flush().expect("Failed to flush stdout");
            next = input::<String>().get().trim().to_string();
        }
    }
}

// Function to generate a random password
fn generate_password(chars: &Vec<&str>, length: usize) -> String {
    let mut rng = thread_rng();
    let mut pass = String::new();
    for _ in 0..length {
        let random_char = chars.choose(&mut rng).unwrap();
        pass.push_str(random_char);
    }
    pass
}

// Function to save data to the CSV file
fn save_data(site_app: &str, account_name: &str, pass: &str) {
    use std::fs::OpenOptions;
    use std::io::Write;

    // Check if file exists
    let file_exists = std::fs::metadata("passwords.csv").is_ok();

    // Open the file in append mode, create if it doesn't exist
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("passwords.csv")
        .expect("Unable to open or create file");

    // If the file didn't exist, write headers
    if !file_exists {
        writeln!(file, "{},{},{}", "Application", "Username", "Password")
            .expect("Unable to write headers");
    }

    // Write the data line
    writeln!(file, "{},{},{}", site_app, account_name, pass)
        .expect("Unable to write data");
}
