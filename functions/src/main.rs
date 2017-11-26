fn main() {
    println!("Hello, world!");
    other_function(12, 42);
    test();

    let i = five();
    println!("{}", i);

    let j = add_one(i);
    println!("{}",j);
}

// functions parameters must be typed
fn other_function(x: i32, y:i32) {
    println!("Other function : {}, {}", x, y);
}

// Statements and expressions
// Statements do something and have no valu
// Expression have a value
fn test() {
    // unlike C, assignments are not expressions, so this is invalid :
    // ```let x = (let y = 4);```

    let x = 5;

    let y = { // A sequence between curly braces can be an expression
        let x =3; // This is a statement
        x+1       // Note the absence of semicolon which makes this an expression
                  // (and not a statement). Kinda like an inplace function with 
                  // an implicit return.
    };

    println!("{}, {}", x, y);
}


fn five() -> i32 {
    5 // Another example of the implicit return without semicolon.
}

fn add_one(x:i32) ->i32 {
    x + 1   // Again, no semicolon
            // If there was one the compiler would complain at the 
            // type mismatch between the return type (i32) and the 
            // fact that statements don't evaluate and have the type
            // `()` (empty type).
}
