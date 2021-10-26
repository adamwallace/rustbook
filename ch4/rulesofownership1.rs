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
}