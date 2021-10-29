// 4.2 References and Borrowing
// If we don't want to pass ownership between functions to continue using variables, we can use references

// Here is how you would define and use `calculate_length()` (from `rulesofownership.rs`) with a reference to an object as a parameter instead of taking ownership of the value:

fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of {} is {}.", s1, len);
    
    
    mutRefs(); // ignore for now
    multMut(); // ignore for now
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what it refers to, nothing happens.

// The ampersands are references, they allow you to refer to some value without taking ownership of it

/*

s                        s1
---------------         -------------------         -------------------
name   |  value             name | value                index | value
---------------         -------------------         -------------------
ptr    |   -----------> ptr      | ------------------>    0   |    h
---------------         -------------------         -------------------
                        len      | 5                      1   |    e
                        -------------------         -------------------
                        capacity | 5                      2   |    l
                        -------------------         -------------------
                                                          3   |    l
                                                    -------------------
                                                          4   |    o
                                                    -------------------

*/

// Note: The opposite of referencing by using & is dereferencing, which is accomplished with the dereference operator, *. We’ll see some uses of the dereference operator in 
// Chapter 8 and discuss details of dereferencing in Chapter 15

// From the code above, the `&s1` REFERS to the value of `s1` but does not own it. So the value it points to will not be dropped when the reference stops being used
// Similarly, the signature of the function indicates it is receiving a `String` reference
// We call the action of creating a reference "borrowing" 

// We cannot modify something we're borrowing as such: 

/*

fn main() {
    let s = String::from("Hello");
    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world!");
}

*/

// This doesn't work. Just as how variables are immutable by default, so too are references 

// Mutable references

// We can fix the errors above with some small tweaks: 

fn mutRefs() {
    let mut s = String::from("Hello");          // `s` needs to be mutable
    change(&mut s);                             // we have to create a mutable reference
    println!("The string has changed: {}", s);
}

fn change(some_string: &mut String) {           // this needs to accept a mutable reference, making it clear that the change function will mutate the value it borrows
    some_string.push_str(", world!");
}

// NOTE: you can only have one mutable reference to a particular piece of data at a time. This code will fail: 

/* 

let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s; 

println!("{}, {}", r1, r2)

*/

// This fails because we cannot borrow `s` as mutable more than once at a time

/* 
The benefit of having this restriction is that Rust can prevent data races at compile time. A data race is similar to a race condition and happens when these three behaviors occur:

    * Two or more pointers access the same data at the same time
    * At least one of the pointers is being used to write to the data
    * There’s no mechanism being used to synchronize access to the data
*/

// AS ALWAYS, we can use curly braces to create a new scopem allowing for multiple mutable references, just not simultaneous ones:

fn multMut() {
    let mut s = String::from("Hello");
    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problem
    let r2 = &mut s;


// NOTE: we also cannot have a mutable reference while we have an immutable one. This code will not compile: 

/*

let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
let r3 = &mut s; // BIG PROBLEM

println!("{}, {}, and {}", r1, r2, r3);

*/

// Users of an immutable reference don’t expect the values to suddenly change out from under them
// Multiple mutable references are fine because no one who is just reading the data has the ability to affect anyone else's reading of the data

// Note that a reference's scope starts from where it is introduced and continues through the last time that reference is used. The following code will compile because the last 
// usage of the immutable references, the `println!`, occurs before the mutable reference is introduced: 

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("r1: {} and r2: {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("r3: {}", r3);

    dangle_example(); // ignore for now 
}
// the scopes of immutable references `r1` and `r2` end after the first `println!` where they are last used which is before the mutable reference to `r3` is created. The scopes don't
// overlap, so this is allowed. The ability of the compiler to tell that a reference is no longer being used at a point before the end of the scope is called Non-Lexical Lifetimes (NLL for short)

// Dangling references

// In languages with pointers, it's easy to erroneously create a "dangling pointer", which references a location in memory that may have been given to someone else, by freeing some memory while 
// preserving a pointer to that memory. The Rust compiler guarantees that references will NEVER be dangling, if you have a referenece to some data, the compiler will ensure that the data will 
// never go out of scope before the reference to the data does

// If we try to create a dangling reference, Rust will prevent with a compile-time error: 

/*

fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello"); // this is a new string

    &s                             // we return a reference to `s`
}                                  // here, `s` goes out of scope and is dropped, thus memory goes away
                                   // danger!

*/

// The error will look something like: 

/*
5 | fn dangle() -> &String {
  |                ^ expected named lifetime parameter
  = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
*/

// The error will refer to a topic not yet covered: lifetimes (ch 10). Ignoring that, the other message contains the key to the problem with this code
// Since `s` is created inside `dangle()`, when the function is finished, `s` will be deallocated but we tried to return a reference to it. Rust won't let us do this

// The solution is to return the `String` directly: 

fn dangle_example() {
    let something = no_dangle();

    println!("something: {}", something);
}

fn no_dangle() -> String {
    let s = String::from("no dangling references here");

    s
}


/* Recap of the rules of references: 

    * At any given time, you can have EITHER one mutable reference OR any number of immutable references
    * References must always be valid

*/ 