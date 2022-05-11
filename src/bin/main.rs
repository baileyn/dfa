extern crate dfa;

use std::io;
use std::io::prelude::*;
use std::str::FromStr;
use std::fs::File;

use dfa::*;

/// Request input from the user on the terminal.
/// 
/// # Usage
/// Prints `msg` to the terminal and blocks waiting for user input.
/// 
/// ```rust,no_run
/// # use io::request_input;
/// 
/// let name: String = request_input("Enter your name: ");
/// ```
pub fn request_input<T>(msg: &str) -> Option<T> 
    where T: FromStr
{
    let mut input = String::new();

    print!("{}", msg);
    io::stdout().flush().expect("Unable to flush output stream.");
    
    if io::stdin().read_line(&mut input).is_ok() {
        if let Ok(data) = T::from_str(input.trim()) {
            Some(data)
        } else {
            None
        }
    } else {
        None
    }
}

/// Request a file from the user until they enter a file that exists.
/// 
/// Returns the `File` that was specified by the user.
fn request_file() -> File {
    loop {
        let file_name: String = request_input("Enter the DFA file name: ").unwrap();
        let file = File::open(file_name);

        if let Ok(file) = file {
            // Since the user has entered a file that exists, return it.
            return file;
        } else {
            // Let the user know that file didn't exist and keep asking.
            eprintln!("The specified file didn't exist!");
        }
    }
}

fn main() {
    let file = request_file();
    
    if let Ok(dfa_builder) = DFABuilder::from(io::BufReader::new(file)) {
        if let Some(dfa) = dfa_builder.build() {
            loop {
                let line: String = request_input("Enter string ['quit' to exit]: ").unwrap();
                
                if line == "quit" {
                    break;
                }

                if dfa.is_valid_string(&line) {
                    println!("That line is valid with this DFA!");
                } else {
                    println!("That line isn't valid with this DFA.");
                }
                println!();
            }
        } else {
            eprintln!("The specified file was successfully parsed, but doesn't represent a valid DFA.");
        }
    } else {
        eprintln!("There was an error in the DFA.");
    }
}