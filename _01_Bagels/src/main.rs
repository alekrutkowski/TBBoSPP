// Converted from Python version with 'ChatGPT 4o with canvas'

use rand::seq::SliceRandom;
use std::io;

const NUM_DIGITS: usize = 3;
const MAX_GUESSES: u32 = 10;

fn main() {
    println!("Bagels, a deductive logic game.\nBy Al Sweigart al@inventwithpython.com\n");
    println!("I am thinking of a {}-digit number with no repeated digits.\nTry to guess what it is. Here are some clues:", NUM_DIGITS);
    println!("When I say:    That means:");
    println!("  Pico         One digit is correct but in the wrong position.");
    println!("  Fermi        One digit is correct and in the right position.");
    println!("  Bagels       No digit is correct.\n");
    println!("For example, if the secret number was 248 and your guess was 843, the clues would be Fermi Pico.\n");

    loop {
        let secret_num = get_secret_num();
        println!("I have thought up a number.");
        println!("You have {} guesses to get it.", MAX_GUESSES);

        let mut num_guesses = 1;
        while num_guesses <= MAX_GUESSES {
            let mut guess = String::new();

            // Keep looping until they enter a valid guess:
            loop {
                println!("Guess #{}: ", num_guesses);
                io::stdin().read_line(&mut guess).expect("Failed to read line");
                guess = guess.trim().to_string();

                if guess.len() == NUM_DIGITS && guess.chars().all(|c| c.is_digit(10)) {
                    break;
                } else {
                    println!("Please enter a valid {}-digit number.", NUM_DIGITS);
                }
            }

            let clues = get_clues(&guess, &secret_num);
            println!("{}", clues);
            num_guesses += 1;

            if guess == secret_num {
                break;
            }

            if num_guesses > MAX_GUESSES {
                println!("You ran out of guesses.");
                println!("The answer was {}.", secret_num);
            }
        }

        // Ask player if they want to play again.
        println!("Do you want to play again? (yes or no)");
        let mut play_again = String::new();
        io::stdin().read_line(&mut play_again).expect("Failed to read line");
        if !play_again.trim().to_lowercase().starts_with('y') {
            break;
        }
    }
    println!("Thanks for playing!");
}

fn get_secret_num() -> String {
    let mut numbers: Vec<char> = "0123456789".chars().collect();
    let mut rng = rand::thread_rng();
    numbers.shuffle(&mut rng);

    numbers.iter().take(NUM_DIGITS).collect()
}

fn get_clues(guess: &str, secret_num: &str) -> String {
    if guess == secret_num {
        return "You got it!".to_string();
    }

    let mut clues = Vec::new();

    for (i, guess_char) in guess.chars().enumerate() {
        if secret_num.chars().nth(i) == Some(guess_char) {
            clues.push("Fermi".to_string());
        } else if secret_num.contains(guess_char) {
            clues.push("Pico".to_string());
        }
    }

    if clues.is_empty() {
        "Bagels".to_string()
    } else {
        clues.sort();
        clues.join(" ")
    }
}
