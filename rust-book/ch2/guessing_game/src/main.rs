use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // 1..=100 includes numbers 1 thru 100 1..100 is 1-99
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        // create a mutable string for the user's guess.
        let mut guess = String::new();

        // read the input from the user, passing a mutable reference to the guess.
        io::stdin()
            .read_line(&mut guess)
        //  expect handles any potential errors, compiler warning without it.
            .expect("Failed to read line");

        // redeclair guess as a u32 integer by parsing the String guess.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
