use std::io; // import the io module

fn main() {

    println!("Guessing game");
    println!("Input your guess");
    

    
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
    
    
}
