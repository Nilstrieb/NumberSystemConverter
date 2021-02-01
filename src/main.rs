mod converter;

#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::io;
use regex::Regex;
use std::result::Result::Err;

fn main() {
    println!("Enter a conversion | help");


    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let trimmed = input.trim();

        if trimmed == "help" {
            println!("Enter a number and end it with a suffix [bodh] and then a space and the system you want it converted to.\n\
                Example: '255d h'\n\
                Will lead to: 'FF'");
        } else if trimmed == "exit" || trimmed == "c" || trimmed == "x"{
            break;
        } else {
            let splits = split(trimmed);
            match splits {
                Ok(a) => {
                    match converter::convert(&a.1, &a.2, &a.0) {
                        Ok(a) => println!("{}", a),
                        Err(e) => println!("{}", e)
                    }
                }
                Err(_) => println!("Invalid input")
            }
        }
    }
}

fn split(s: &str) -> Result<(String, String, String), ()> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\w+) ?(\w) (\w)$").unwrap();
    }

    for capture in RE.captures_iter(s) {
        return Ok((capture[1].to_string(), capture[2].to_string(), capture[3].to_string()));
    }
    Err(())
}
