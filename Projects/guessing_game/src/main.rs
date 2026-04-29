use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("Secret number is: \n{}", secret_number);

    loop {
        println!("Input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // Comparing
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low \n"),
            Ordering::Greater => println!("Too high \n"),
            Ordering::Equal => {
                println!("Correct \n");
                break;
            }
        }
    }
}
