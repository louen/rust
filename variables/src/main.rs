fn main() {

    // -- const and mutables

    // mutable variable declaration
    let mut x = 5;
    println!("x = {}",x);

    // this creates a compiler error when x is not mutable.
    x = 6;

    println!("x = {}",x);

    // using `const` instead of `let` declares a constant.
    // const variables must be explicitely typed
    // they can only be assigned constant expressions (à la `constexpr` in c++)
    // Note : underscore in numerical litterals are here for readability.
    // They have no effect and are optionnal.
    const MAX_POINTS: u32 = 100_000;

    println!("max = {}",MAX_POINTS);


    // -- shadowing
    // here each line creates a new instance of x
    // as a const variable which is re-assigned.
    // This is different than a mutable variable but achieves the same effect.
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    println!("x = {}",x);

    // Shadowing a variable with a variable of different type.
    let spaces = "   ";
    let spaces = spaces.len();
    println!("{}", spaces);

    // Here this is better than using a mutable `spaces` variables
    // because mutable variables cannot be reassigned to a different type
    // e.g.
    // ```
    // let mut aa = "aaa";
    // aa = aa.len(); // error : mismatched types.
    // ```

    // -- Data Types

    // Here we must explicit the type because the compiler cannot infer it.
    let guess:u32 = "42".parse().expect("Not a number");
    println!("{}", guess);

    // Integer types :
    // i8 , u8      8 bit  signed and unsigned 
    // i16, u16     16 bit signed and unsigned
    // i32, u32     32 bit signed and unsigned
    // i64, u64     64 bit signed and unsigned
    // isize, usize native integer size (32 or 64 bits depending on machine).
    // typically the type returned by `len()`

    // litterals
    let decimal  = 68_655;  // With underscores for readability
    let hex = 0x424242;     // Hexadecimal
    let octal = 0o755;      // Octal
    let binary = 0b11_01_00;// Binary
    let ascii = b'a';       // Byte (ascii value)

    println!("ascii  = {}", ascii);
    // Floating point types : f32 (single) and f64 (double)

    // Arithmetic operators
    let x = decimal + hex - octal * binary / guess;
    println!("x = {}", x);
    let x = 30/7; // integer division
    println!("x = {}", x);
    let y = 30%7; // modulus
    println!("y = {}", y);

    // Rust also has booleans

    let v = true;
    let f:bool = false;

    if v && !f {
        println!("Yeeah !");
    }

    // Rust also has chars

    let c = 'z'; // char litterals just like in C with single quotes
    let scry = '𝒴'; // chars support utf-8 characters natively.
    let eternity: char = '永';
    // Chars are 'unicode scalar values' (which may or may not be actual chars).

    println!("{}{}{}", c,scry,eternity);

    // --- Compound types

    // Rust has tuples
    let tuple:(i32,f64, u8) = (42,3.14,0xff);

    let (a,b,c) = tuple; // extract individual members from tuple
    println!("{},{},{}",a,b,c);

    // Values can be accessed with .0, .1 etc.
    println!("{},{},{}",tuple.0, tuple.1,tuple.2);


    // Arrays of elements of the same type
    let arr = [2,3,5,7,11,13];

    // arrays are accessed c-style
    println!("{} is prime", arr[3]);

    // Bounds-checking is active by default and produces a `panic`, i.e. a crash
    // ```println!("{} is prime", arr[10]);```







}
