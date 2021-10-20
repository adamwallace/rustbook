// 3.2 functions

fn main() {
    println!("main func");
    another_func(6, 4);
    five_caller(); // ignore me for now
}

// NOTE: Function bodies are made up of a series of statements optionally ending in an expression. 
// Statements are instructions that perform some action and do not return a value. Expressions evaluate to a resulting value.

fn another_func(x: i32, y: i32) { // type declaration in param. you MUST declare parameter types in the function signature
    println!("another func");
    println!("{} {}", x, y);

    // let x = (let y = 6); // you cannot do this because statements do not return values. therefor, you cannot assing a let statement to another variable 
    // the `let y = 6` statement does not return a value, so there isn't anything for `x` to bind to. you also cannot say something like `x = y = 6`

    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    // this works because `y` is a value whose assigned expression evaluates to 4 and goes on to get bound to `y` 

    /* 
        **** BIG WARNING ****: IF YOU ADD A SEMICOLON TO THE END OF AN EXPRESSION, YOU TURN IT INTO A STATEMENT, WHICH DOES NOT RETURN A VALUE. EXPRESSIONS
                              DO NOT INCLUDE ENDING SEMICOLONS
    */
}


// functions with return values - for functiosn that return values, we do not name return values, but we do declare their type after an arrow: `->` 

fn five() -> i32 {
    5
}

fn five_caller() {
    let x = five();
    println!("the value returned from five() is: {}", x);
    
    // note that we can define functions wherever, so we shall do that in the next example 
    let y = plus_one(x);
    println!("the value of y is: {}", y);
}

fn plus_one(y: i32) -> i32 {
    y + 1
}

