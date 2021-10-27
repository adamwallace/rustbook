// 4.2 References and Borrowing
// If we don't want to pass ownership between functions to continue using variables, we can use references

// Here is how you would define and use `calculate_length()` (from `rulesofownership.rs`) with a reference to an object as a parameter instead of taking ownership of the value:

fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of {} is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

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