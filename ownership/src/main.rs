fn main() {
    
    // Construct a ref to a string litteral
    {
        let a = "Lalala";
        println!("{}", a);
    }

    // Construct a heap-allocated string from a string litteral
    {
        let mut s = String::from("Trololo");
        s.push_str("lololololo");
        println!("{}", s);
    }
    // Basically at the end of scope, s is destroyed and 
    // heap memory is automatically freed through the `Drop` method
    // (aka destructor)

    // Deep copy of stack values
    {
        let x = 5;
        let y = x;

        // x and y are stack values so they are copied.
        // This is actually because they implement the `Copy` trait
        // But the type cannot have a destructor.
        println!("{}, {}",x, y);
    }
    
    // Ownership transfer
    {
        let s1 = String::from("five");
        let s2 = s1;

        // s2 = s1 performs a transfer of ownership. Now s1 is invalid
        // so any attempt to use it will fail
        // println!("{}, {}",s1, s2);
        println!("{}" , s2);
    }
    
    // Deep copy of heap types
    {
        let s1 = String::from("five");
        // Rust never does a deep copy with =, but there is a clone() method
        let s2 = s1.clone();
        println!("{}, {}",s1, s2);
    }

    // Ownership transfer in functions
    {
        let s = String::from("five");
        take(s); // passing s as argument transfers the ownership

        // Thus any subsequent attempt to use s will fail to compile
        // println!("{}",s);

        let i = -5;
        takeint(i);
        // It works for stack variables
        println!("{}",i);
    }
        


}

fn take( a_string: String) { // string comes into scope
    println!("take function {}", a_string);
} // string goes out of scope and memory is freed

fn takeint( a_int: i32) { // int comes into scope
    println!("take function {}", a_int);
}// int is a local-stack variable 

