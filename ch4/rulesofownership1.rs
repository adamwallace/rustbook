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
// Since this pointer is a known, fixed size you can store it on the stack but when youj want the actual data, you must follow the pointer

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
                        // still valid
}                       // validity ends with scope

// The point is that it's important to recognize that 1) `s` is valid when it comes into scope and REMAINS valid until it goes out of scope 




