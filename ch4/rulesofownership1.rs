// Ch 4 - ownership - rust's most unique feature, allows rust to make memory safety guarantees without needing a garbage collector

// 4.1 what is ownership? 
// a system through which memory is managed through ownership with a set of rules that the compiler checks at compile time. These features do not slow down your program

// The Stack and Heap 
// in a systems programming language like rust, knowing what values are on the stack vs the heap dictate how  the language behaves and why you have to make certain decisions

// both the stack and the heap are parts of memory that are available to your code to use at runtime

// The stack stores values in the order it gets them and removes values in the opposite order (last in, first out). Adding stuff is "pushing" to the stack, removing is "popping"
// All data stored in the stack must have a known, fixed size. Otherwise, it must be stored on the heap

// When you put stuff on the heap, you request a certain amount of space. The memory allocator finds an empty spot on the heap that's big enough, marks it as being in use, then
// returns a "pointer" which is a location address. This is called "allocating" or "allocating on the heap"
// Since this pointer is a known, fixed size you can store it on the stack but when you want the actual data, you must follow the pointer

// Pushing to the stack is faster because the allocator never has to search for a place to store new data; that location is at the top of the stack. 

// When your code calls a function, the values passed into the function (including, potentially, pointers to data on the heap) and the function’s local variables get pushed onto 
// the stack. When the function is over, those values get popped off the stack

// Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap so you don’t 
// run out of space are all problems that ownership addresses

/* 
Keep these rules in mind while learning about ownership: 

    * Each value in Rust has a variable that’s called its owner.
    * There can only be one owner at a time.
    * When the owner goes out of scope, the value will be dropped.
*/

// let's begin by taking a look at scope
// In the following example, the variable `s` refers to a string literal where the value of the string is hardcoded into the text of our program 
// Note that the variable is valid from the point at which it's declared until the end of the current scope  

fn main() 
{                       // `s` is not valid, it is not yet declared
    let s = "hello";    // `s` is valid from this point forward
                        // this is still valid
                        // validity ends with scope

    // The point is that it's important to recognize that 1) `s` is valid when it comes into scope and REMAINS valid until it goes out of scope 

    // Multiple variables can interact with the same data in different ways in Rust. For example: 

    let x = 5; 
    let y = x; // One variable is created and a copy of that variable is assigned to the second variable, `y`. Both variables are pushed onto the stack

    // Now the string version: 

    let s1 = String::from("hello");
    let s2 = s1;

    // It does not work the same way as the example above with integers.
    // Take a look at the table below to see what is happening to String under the covers. A String is made up of three parts, shown on the left: a pointer to the memory that holds 
    // the contents of the string, a length, and a capacity. This group of data is stored on the stack. On the right is the memory on the heap that holds the contents: 

    /*
                 s1
        -------------------         -------------------
            name | value                index | value
        -------------------         -------------------
        ptr      | ------------------>    0   |    h
        -------------------         -------------------
        len      | 5                      1   |    e
        -------------------         -------------------
        capacity | 5                      2   |    l
        -------------------         -------------------
                                          3   |    l
                                    -------------------
                                          4   |    o
                                    -------------------
    */
    // The length is how much memory in bytes the contents of `String` is currently using
    // The capacity is the total amount of memory in bytes that the `String` has received from the allocator
    // The difference between length and capacity matter but not right now, so we will ignore capacity

    // When we assign `s1` to `s2` the `String` data is copied (meaning we copy the pointer, length and capacity that are on the stack). We do not copy the data on the heap that 
    // the pointer refers to. See the next table for a visual representation of this: 

    /*
                 s1
        -------------------         -------------------
            name | value                index | value
        -------------------         -------------------
        ptr      | ------------------>    0   |    h
        -------------------     |   -------------------
        len      | 5            |         1   |    e
        -------------------     |   -------------------
        capacity | 5            |         2   |    l
        -------------------     |   -------------------
                                |         3   |    l
                                |   -------------------
                                |         4   |    o
                                |   -------------------
                                |
                 s2             |
        -------------------     |    
            name | value        |        
        -------------------     |   
        ptr      | -------------|
        -------------------         
        len      | 5                      
        -------------------         
        capacity | 5                
        -------------------
    
    */
    // NOTE: if Rust copied the heap data as well, the table would include another index/value table that `s2` would be pointing to. RUST DOES NOT DO THIS. If it did, the 
    // operation `s1 = s2` could very expensive in terms of runtime performance if the data on the heap were large 

    // Earlier it was mentioned that when a variable goes out of scope, Rust calls `drop` to free up memory but note that `s1` and `s2` are pointing to the same location. 
    // This could be a problem because when languages try to free the same memory twice, it leads to a "double free" error and could lead to memory corruption and therefor
    // security vulnerabilities 

    // To ensure memory safety, there is one more detail to what happens in this situation in Rust. After `let s2 = s1`, Rust considers `s1` to no longer be valid. Therefor,
    // Rust doesn't need to free anything when `s1` goes out of scope. So trying the following will result in an error: 

    /*
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1);
    */

    // You’ll get an error because Rust prevents you from using the invalidated reference

    // If you’ve heard the terms shallow copy and deep copy while working with other languages, the concept of copying the pointer, length, and capacity without copying the 
    // data probably sounds like making a shallow copy. But because Rust also invalidates the first variable, instead of being called a shallow copy, it’s known as a move 

    // In addition, there’s a design choice that’s implied by this: Rust will never automatically create “deep” copies of your data. Therefore, any automatic copying can be
    // assumed to be inexpensive in terms of runtime performance

    // Ways Variables and Data Interact: Clone

    // If we do want to deeply copy the heap data of the String, not just the stack data, we can use a common method called clone. This uses method syntax: 

    let s1 = String::from("hello");
    let s2 = s1.clone(); // heap data gets copied, but this MAY be expensive 

    println!("s1 = {}, s2 = {}", s1, s2);

    // So why does the following code work (with integers?):
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    // The reason is that types such as integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make
    // That means there’s no reason we would want to prevent `x` from being valid after we create the variable `y`. In other words, there’s no difference between deep and 
    // shallow copying here, so calling clone wouldn’t do anything different from the usual shallow copying and we can leave it out

    // Rust has a special annotation called the `Copy` trait that we can place on types like integers that are stored on the stack (more about traits in ch 10). If a type
    // implements the `Copy` trait, an older variable is still usable after assignment (we cannot do this of the type or any of it's parts has implemented the `Drop` trait)
    // If the type needs something special to happen when the value goes out of scope and we add the `Copy` annotation to that type, we'll get a runtime error 
    // (Learn more about the `Copy` annotation in Appendix C "Derivable Traits")

    /*
    What types can use the `Copy` trait? As a general rule, any group of simple scalar values can implement `Copy` and nothing that requires allocation or is some form
    of resource can implement `Copy`: 
    
        * All integer types, such as `u32`
        * The Boolean type
        * All floating point types, such as `f64`
        * The character type, char
        * Tuples, if they only contain types that can also implement `Copy`. So (i32, String) does not implement `Copy`
    */

    // The following is an example of passing ownership (this example starts in `main()` and follows into the next two functions):

    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

    // Here, x goes out of scope, then s. But because s's value was moved, nothing
    // special happens.

    returnValuesAndScopePt1(); // ignore for now
    tupleReturnExample(); // ignore for now
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

 fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.



// The final sections will be examples from the 4.1 subsection "Return Values and Scope". Note that we called `returnValuesAndScopePt1()` at the bottom of `main()`

// Returning values can also transfer ownership
fn returnValuesAndScopePt1() {
    let s1 = gives_ownership();                       // gives_ownership moves its return value into s1
    let s2 = String::from("hello");                   // s2 comes into scope
    let s3 = takes_and_gives_back(s2);                // s2 is moved into takes_and_gives_back, which also moves its return value into s3
}                                                     // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {                      // gives_ownership will move its return value into the function that calls it
    let some_string = String::from("yours");          // some_string comes into scope
    some_string                                       // some_string is returned and moves out to the calling function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
a_string                                              // a_string is returned and moves out to the calling function
}

// The ownership of a variable follows the same pattern every time: assigning a value to another variable moves it. When a variable that includes data on the heap goes
// out of scope, the value will be cleaned up by drop unless the data has been moved to be owned by another variable

// Taking ownership and then returning it with every function sucks. "References" take care of this, but we'll talk about that in the next section. For now, know that 
// it's possible to return multiple values using a tuple: 

fn tupleReturnExample() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}