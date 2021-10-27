// 4.2 References and Borrowing
// If we don't want to pass ownership between functions to continue using variables, we can use references

// Here is how you would define and use `calculate_length()` (from `rulesofownership.rs`) with a reference to an object as a parameter instead of taking ownership of the value:

fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of {} is {}.", s1, len);
    
    
    mutRefs(); // ignore for now
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
    let mut s = String::from("Hello");
    change(&mut s);
    println!("The string has changed: {}", s)
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!")
}