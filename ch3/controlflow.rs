// 3.5 control flow

fn main () {
    let num = 3;
    // nothing too interesting going on here
    if num < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    // you can't do the following since the compiler expects a boolean
    
    /*
    if number { 
        println("the number exists");
    }
    */

    if num != 0 {  
        println!("the number exists");
    }


    if num % 4 == 0 {
        println!("num is divisible by 4");
    } else if num % 3 == 0 {
        println!("num is divisible by 3");
    } else if num % 2 == 0 {
        println!("num is divisible by 2");
    } else {
        println!("num is not divisible by 4, 3, or 2");
    } // again, nothing too interesting

    // you can also use `if`s in a `let` statement
    let condition = true;
    let num2 = if condition { 5 } else { 6 };
    // HOWEVER, you cannot do: 
    // let num2 = if condition { 5 } else { "six" };
    // this is a type conflict, and rust needs to know at compile time what type a variable is definitively so that its type is valid everywhere we use said variable

    println!("the value of num2 is: {}", num2);

    // loops

    // this will loop forever:
    /*
    println!("print!")
    loop{
        println!("again!")
    }
    */ 

    // you can place the `break` keyword in a loop to break out. But you might need to pass the result of a loop to the rest of your code, so you can do that with `break` as well: 

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    // conditional loops with `while`
    // this is pretty self-explanatory:

    let mut number = 3;

    while number != 0 { // this is literally syntactic sugar for something that could be done with `loop`, `if`, `else` and `break` but it's so common that rust has a built in 
                        // construct for it
        println!("{}!", number);
        number -= 1;
    }

    println!("LIFTOFF!!!");

    // you can loop through collections pretty easily: 
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    } // this approach is error prone though, we could easily cause the program to panic if we step out of bounds of the `a` array

    // for loops

    for element in a.iter() { // this is safer because it allows rust to iterate, keeping you from going out of bounds
        println!("the value is: {}", element);
    }

    // we can do something similar using a range, and using a construct we haven't covered yet, `rev`: 

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}