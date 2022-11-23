// use library input/output (io)
use std::io;

// use library rand
use rand::Rng;

// use library Ordering
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    // generate secret number in range 1 to 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    //println!("The secret number is {secret_number}");
    loop {
        println!("Please input your guess.");

        // declare new variable guess, type String
        let mut guess = String::new();

        // catch guess from user
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line.");

        // convert guess variable to numeric type
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // print user guess
        println!("You guessed: {guess}");

        // function to compare the secret number with the user guess
        match guess.cmp(&secret_number) {
            // first variant
            Ordering::Less => println!("Too small!"),
            // second variant
            Ordering::Greater => println!("Too big!"),
            // third variant
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
