use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("guess the number");

    let chicken_dinner = rand::thread_rng().gen_range(1..101); // inclusive lower bound, exclusive upper bound, can also use 1..=100

    loop{

        let mut guess = String::new();
        println!("enter your guess");

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("problem here");
                continue;
            }
        };

        match guess.cmp(&chicken_dinner) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Large!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}