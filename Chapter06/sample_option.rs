//-- #########################
//-- Task: Implementing Options
//-- Author: Vigneshwer.D
//-- Version: 1.0.0
//-- Date: 26 March 17
//-- #########################

// All arguments are handled explicitly using `match`.
fn compare_stmt_match(input: Option<&str>) {
    // Specify a course of action for each case.
    match input {
        Some("Rust CookBook") => println!("Rust CookBook was selected"),
        Some(_inner) => println!("Rust CookBook not selected"),
        None => println!("No input provided"),
    }
}

// All arguments are handled implicitly using `unwrap`.
fn compare_stmt_unwrap(input: Option<&str>) {
    // `unwrap` returns a `panic` when it receives a `None` value
    let inside_val = input.unwrap();
    if inside_val == "Another Book" {
        panic!("Rust CookBook is not selected");
    }

    println!("I love {}s!!!!!", inside_val);
}

// main execution starts here
fn main() {
    let desired_book = Some("Rust CookBook");
    let another_book = Some("Another Book");
    let empty_value = None;

    compare_stmt_match(desired_book);
    compare_stmt_match(another_book);
    compare_stmt_match(empty_value);

    println!("*********************");

    let rand_book = Some("Random Book");
    let no_val = None;

    compare_stmt_unwrap(rand_book);
    compare_stmt_unwrap(no_val);
}
