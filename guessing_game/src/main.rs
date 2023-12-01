use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn run_game() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut attempts = 0;
    loop {
        println!("Input your guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to get input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("You guessed: {guess}");
        attempts += 1;
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Guess higher"),
            Ordering::Greater => println!("Guess lower"),
            Ordering::Equal => {
                println!("You win! You guessed the number in {attempts} attempts.");
                break;}
        };
    }
}

fn play_again() {
    loop {
        println!("Do you want to play again? (y/n)");
        
        let mut retry = String::new();

        io::stdin()
            .read_line(&mut retry)
            .expect("Failed to read line");

        let retry = retry.trim();

        if retry == "y" {
            println!("Great! Let's start :)");
            run_game();
        }
        else {
            println!("See you next time :)");
            break;
        }
    }
}


fn main() {
    println!("Welcome to the number guessing game! Guess the number between 1-100");
    run_game();
    play_again();
}
