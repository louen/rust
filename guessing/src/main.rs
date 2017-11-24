// Use an external dependency. Implicitely call `use rand`.
extern crate rand;

use std::io; // import the io module

use std::cmp::Ordering; // import the Ordering enum

use rand::Rng; // Import the Rng trait.

fn main() {

    println!("Guessing game");
    println!("Input your guess");


    // `thread_rng` is a thread-local random number generator
    // `gen_range` generate a random number between 1 and 101 (excluded)
    // type is infered to be i32 (default integer type)
    let secret_number = rand::thread_rng().gen_range(1,101);


    // Let statements declare a variable.
    // Variable are const by default so `let mut` declares a *mutable* variable
    // `new` function is a convention for a static constructor
    let mut guess = String::new();


    // io::stin is a handle for the standard input
    // we pass a reference to guess as argument to read line.
    // but references are const by default, hence the need for &mut (instead of just &)
    io::stdin().read_line(&mut guess).expect("Failed to read !");

    // read_line retunrs a `io::Result` (an instange of a generic Result type which
    // is an enum (Ok or Err). Results have `expect()`  method which is an assert.
    // Not calling expect() will generate a warning.


    // {} are placeholders in a string. They are replaced by the variables
    // values in order of their appearance in println().
    println!("Your guess is {}", guess);


    // Declare a variable with type (u32) by *shadowing* the previous declaration
    // of `guess`.
    // `trim()` removes the whitespace (e.g. the newline read from stdin)
    // `parse()` converts it to a number (coercing in u32 because we declared
    // the variable with a specific type). Note that this coerces secret_number
    // to u32 as well.
    let guess: u32 = guess.trim().parse().expect("Failed to parse input");

    // cmp method called to compare secret_number with guess.
    // match statement is similar to switch() (note the comma between statements)
    match guess.cmp(&secret_number) {
        Ordering::Less    => println!("too small"),
        Ordering::Greater => println!("too big"),
        Ordering::Equal   => println!("a winner is you"),
    }


}
