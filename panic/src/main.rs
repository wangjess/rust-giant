use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
    // Calling the panic! macro
    //panic!("Crash + Burn!");

    // Out of bounds access attempt. Rust will stop execution and refuse to continue.
    //let v = vec![1, 2, 3];
    //v[99];

    // Running RUST_BACKTRACE=1 lists all the functions that have been called to get to that point
    // RUST_BACKTRACE=full 

    // Recovering from Errors with Result
    let greeting_file_result = File::open("hello.txt");

    // Uses the match expression
    // As shown, useful but cluttering and difficult to read
    // Cleaner and more concise expression is the "unwrap_or_else" method.
    // unwrap_or_else is covered in Chapter 13.
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            }
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        }, 
    };

    // Using .unwrap() calls panic!, but without a custom message
    //let greeting_file = File::open("test.txt").unwrap();

    // Use .expect() to pass a custom message to panic! macro
    //let greeting_file = File::open("test.txt").expect("test.txt should be included");

    // Shortcut for Propagating Errors: The ? Operator
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut username = String::new();

        File::open("hello.txt")?.read_to_string(&mut username)?;

        Ok(username)
    }

    // Note: ? operator converts Result types to the scoping function's Result type
    // Will not work if the scoping function's Result type is incompatible
    // ex: called inside main(), which does not return a Result type (it is Void type)
    // Can manually over-ride this by specifying main(), which is a special
    // entry and exit function, to return a Result<(), Box<dyn Error>>
    // ? cannot be mix and matched with Result and Option. 
    // That is, ? cannot auto-convert a Result to an Option or vice versa. 
}
