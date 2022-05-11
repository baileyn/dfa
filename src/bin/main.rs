extern crate dfa;
extern crate terminal;

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
    
    match DFABuilder::from(io::BufReader::new(file)) {
        Ok(dfa_builder) => {
            if let Some(dfa) = dfa_builder.build() {
                loop {
                    // Request input from the user for the string to test.
                    let line: String = request_input("Enter string ['quit' to exit]: ").unwrap();
                    terminal::clear();
                    
                    // If the user specified to quit, exit the program.
                    if line == "quit" {
                        break;
                    }

                    // Determine whether or not the supplied line is valid for the DFA.
                    if dfa.is_valid_string(&line) {
                        println!("That line is valid with this DFA!");
                    } else {
                        println!("That line isn't valid with this DFA.");
                    }
                }
            } else {
                eprintln!("Unable to build DFA.");
            }
        },
        Err(e) => {
            // If an error occured, determine why and print out a useful message.
            match e {
                DFABuilderError::MalformedLine(e) => eprintln!("{}", e),
                DFABuilderError::TransitionAlreadyExists => eprintln!("Multiple state transitions for the same letter was detected."),
                DFABuilderError::NonIntegralFinalState => eprintln!("A final state was detected that didn't have an integral value."),
                DFABuilderError::EmptyStream => eprintln!("No data was found in the specified file."),
                DFABuilderError::ExpectedChar => eprintln!("Expected a single character, but received more."),
                DFABuilderError::ExptectedInt => eprintln!("Expected an integer, but got different input.")
            }
        }
    } 
}