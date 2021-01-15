//-- #########################
//-- Task: Implementing Boxing
//-- Author: Vigneshwer.D
//-- Version: 1.0.0
//-- Date: 26 March 17
//-- #########################

use std::error;
use std::fmt;
use std::num::ParseIntError;
type Result<T> = std::result::Result<T, Box<error::Error>>;

#[derive(Debug)]
enum CustomError {
    EmptyVec,
    Parse(ParseIntError),
}
impl From<ParseIntError> for CustomError {
    fn from(err: ParseIntError) -> CustomError {
        CustomError::Parse(err)
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CustomError::EmptyVec => write!(f, "please use a vector with at least one element"),
            CustomError::Parse(ref e) => e.fmt(f),
        }
    }
}

impl error::Error for CustomError {
    fn description(&self) -> &str {
        match *self {
            CustomError::EmptyVec => "empty vectors not allowed",
            CustomError::Parse(ref e) => e.description(),
        }
    }
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            CustomError::EmptyVec => None,
            CustomError::Parse(ref e) => Some(e),
        }
    }
}

fn double_val(vec: Vec<&str>) -> Result<i32> {
    let first = try!(vec.first().ok_or(CustomError::EmptyVec));
    let parsed = try!(first.parse::<i32>());
    Ok(2 * parsed)
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
