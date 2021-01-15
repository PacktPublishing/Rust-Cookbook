//-- #########################
//-- Task: Defining your own error type
//-- Author: Vigneshwer.D
//-- Version: 1.0.0
//-- Date: 26 March 17
//-- #########################

use std::fmt;
use std::num::ParseIntError;
type Result<T> = std::result::Result<T, CustomError>;

#[derive(Debug)]
enum CustomError {
    EmptyVec,
    Parse(ParseIntError),
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CustomError::EmptyVec => write!(f, "please use a vector with at least one element"),
            // This is a wrapper, so defer to the underlying types' implementation of `fmt`.
            CustomError::Parse(ref e) => e.fmt(f),
        }
    }
}

fn double_val(vec: Vec<&str>) -> Result<i32> {
    vec.first()
        // Change the error to our new type.
        .ok_or(CustomError::EmptyVec)
        .and_then(|s| {
            s.parse::<i32>()
                // Update to the new error type here also.
                .map_err(CustomError::Parse)
                .map(|i| 2 * i)
        })
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let numbers = vec!["93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];
    print(double_val(numbers));
    print(double_val(empty));
    print(double_val(strings));
}
