fn main() {
    
    // -- if instructions 
    let x = 42;

    // Note the absence of  ( ) around the condition
    if x < 12 {
        println!("First");
    } else {
        println!("Second");
    }

    // conditions in a if statement must be bool
    
    // `if x {` // will fail with type mismatch (bool and integer)
    if x != 0 {
        println!("x is nonzero");
    }

    // of course there is else if
    if  x % 3 == 0  {
        println!("A");
    } else if  x % 3 == 1 {
        println!("B");
    } else if  x % 3 == 1 {
        println!("C");
    }


    // if are actually expressions and can be used in assignments
    let y = if  x < 12 {
        17
    } else {
        19
        // here we cannot put something that is not an integer (e.g. a string like `"six"`)
        // because the two branches have to evaluate to the same type
    };
    println!("{}", y);


    // -- loops

    // Basic loop
    let mut x = 10;
    loop { // this is a `while(true)` loop
        x = x -1;
        println!("{}",x);
        if x ==0 {
            break;
        }
    }
    
    // while loop
    x = 10;
    while x > 0 {
        x = x -1;
        println!("{}",x);
    }

    // for(each) loop
    let array = [2,3,5,7,11,13,17];
    for n in array.iter() { // use iter() to get an iterator
        println!("{} is prime", n);
    }

    // Reverse loop with range
    // here (1..10) is a range (1 included and 10 excluded)
    // and rev() gets a reverse iterator
    for n in (1..10).rev() {
        println!("{}", n);
    }

    // note : this does not work with tuples (they have no `iter`)

}
