fn main() {
    // 3.1 
    // RUST IS STATICALLY TYPED - MEANING WE MUST KNOW THE TYPES OF ALL VARIABLES AT COMPILE TIME

    let x = 5;
    println!("the value of x is: {}", x);
    // x = 6; this does not work, need to define x with `mut`

    const MAX_VAL: u32 = 100_000; // when using the const keyword, type definition is REQUIRED
    // Rust’s naming convention for constants is to use all uppercase with underscores between words, and underscores can be inserted in numeric literals to improve readability


    // shadowing a variable is defining a variable with `let` that is not mutable but redefining it afterwords. This actually creates a new variable

    let y = 5; 
    let y = y + 1;
    let y = y * 2;

    println!("y is: {}", y); 

    let spaces = "   ";
    let spaces = spaces.len(); // these are two distinct variables so this is allowed, even if the type is not the same and they have the same name
    /*
    let mut spaces = "   ";
    spaces = spaces.len();
    */ 
    // ^ this however, will result in a compiler-time error because we're trying to switch the type of an existing variable

    // 3.2
    // data types - scalar and compound 

    let guess: u32 = "42".parse().expect("not a number!");
    // if there's no type annotation here, rust will display an error

    /**************** scalar types - represents a single value such as integers, floating-point numbers, Booleans and characters ****************/

    // integer types in rust 

    /*
        LENGTH      SIGNED      UNSIGNED
        --------------------------------
        8-bit       i8          u8
        16-bit      i16         u16
        32-bit      i32         u32
        64-bit      i64         u64
        128-bit     i128        u128
        arch        isize       usize
    */

    // integer literals

    /*
        NUMBER LITERALS         EXAMPLE
        --------------------------------
        Decimal                 98_222
        Hex                     0xff
        Octal                   0o77
        Binary                  0b1111_0000
        Byte (use `u8` only)    b'A'
    */

    /*

    NOTE: when it comes to integer overflow, if you have a u8 and try to assign it a value larger than 255, an overflow will occur. In debug mode, rust will check for this and 
    panic at runtime if overflow occurs. If you compile in release mode with the `--release` flag, rust will not check for overflows that cause panic. If an overflow occurs, 
    Rust perform "two's compliment wrapping", meaning that values greater than max value the type can hold will wrap around. This will not cause panic, but if you have a u8 
    value of 256, it will be reassigned to 1, which is probably not what you're expecting. To explicitly handle the possibility of overflow, you can use families of methods
    that the standard library provides on primitive numeric types:
        * Wrap all modes with the `wrapping_*` methods, such as `wrapping_add`
        * Return the `None` value if there is an overflow with the `checked_*` methods
        * Return the value and a boolean indicating whether there was an overflow with the `overflowing_*` methods
        * Saturate at the values minimum or maximum values with the `saturating_*` methods
    */

    // floating-point types

    // there are two floating-point types, f32 and f64

    let z = 2.0;     // f64
    let z: f32 = 3.0; // f32

    // floating-point numbers are represented according to the IEEE-754 standard. f32 is a single-precision float while f64 has double precision

    // numeric operations - no real surprises here

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    // the Rust book's Appendix B contains a list of all operators Rust provides

    // Booleans 

    let t = true; 
    let f: bool = false; // explicit type annotation 

    // character type
    // NOTE: `char` literals are specified with single quotes as opposed to string literals, which are specified with double quotes

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻'; // please god no 

    // chars are 4 bytes in size and represents a Unicode Scalar Value, meaning it can represent a lot more than ASCII. Lord help us 
    // chapter 8 will talk more about chars, because a "character" isn't really a concept in Unicode so your intuition of what a `char` is probably doesn't match what it is in Rust

    /**************** compound types - groups multiple values into one type. Rust has two of these - touples and arrays ****************/
    
    // tuple type

    let tup: (i32, f64, u8) = (500, 6.4, 1); // tuples have a fixed length: once declared, they cannot grow or shrink in size
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup; // destructuring 
    println!("the value of y is: {}", y);

    // in addition to destructuring, we can access a a tuple element directly with `.` 
    // i hate this convention because i will never remember it 

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("tuple values: {} {} {}", five_hundred, six_point_four, one);

    
}