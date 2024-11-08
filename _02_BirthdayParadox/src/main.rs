// Converted from Python version with 'ChatGPT 4o with canvas'

use chrono::{Datelike, Duration, NaiveDate};
use rand::Rng;
use std::collections::HashSet;
use std::io;

fn get_birthdays(number_of_birthdays: usize) -> Vec<NaiveDate> {
    let mut birthdays = Vec::new();
    let start_of_year = NaiveDate::from_ymd_opt(2001, 1, 1).expect("Invalid date");

    let mut rng = rand::thread_rng();
    for _ in 0..number_of_birthdays {
        let random_number_of_days = rng.gen_range(0..365);
        let birthday = start_of_year + Duration::days(random_number_of_days);
        birthdays.push(birthday);
    }
    birthdays
}

fn get_match(birthdays: &[NaiveDate]) -> Option<NaiveDate> {
    let mut seen = HashSet::new();
    for &birthday in birthdays {
        if !seen.insert(birthday) {
            return Some(birthday);
        }
    }
    None
}

fn main() {
    println!(
        "Birthday Paradox Simulation, by Al Sweigart al@inventwithpython.com\n\nThe birthday paradox shows us that in a group of N people, the odds that two of them have matching birthdays is surprisingly large.\nThis program does a Monte Carlo simulation (that is, repeated random simulations) to explore this concept.\n\n(It's not actually a paradox, it's just a surprising result.)\n"
    );

    let months = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];

    let num_birthdays = loop {
        println!("How many birthdays shall I generate? (Max 100)");
        let mut response = String::new();
        io::stdin().read_line(&mut response).expect("Failed to read line");
        if let Ok(num) = response.trim().parse::<usize>() {
            if num > 0 && num <= 100 {
                break num;
            }
        }
        println!("Please enter a valid number between 1 and 100.");
    };

    println!("\nHere are {} birthdays:", num_birthdays);
    let birthdays = get_birthdays(num_birthdays);
    for (i, birthday) in birthdays.iter().enumerate() {
        if i != 0 {
            print!(", ");
        }
        let month_name = months[(birthday.month() - 1) as usize];
        print!("{} {}", month_name, birthday.day());
    }
    println!("\n");

    // Determine if there are two birthdays that match.
    let match_birthday = get_match(&birthdays);
    println!("In this simulation, ");
    if let Some(birthday) = match_birthday {
        let month_name = months[(birthday.month() - 1) as usize];
        println!("multiple people have a birthday on {} {}", month_name, birthday.day());
    } else {
        println!("there are no matching birthdays.");
    }
    println!();

    // Run through 100,000 simulations:
    println!("Generating {} random birthdays 100,000 times...", num_birthdays);
    println!("Press Enter to begin...");
    let mut _enter = String::new();
    io::stdin().read_line(&mut _enter).expect("Failed to read line");

    println!("Let's run another 100,000 simulations.");
    let mut sim_match = 0;
    for i in 0..100_000 {
        if i % 10_000 == 0 {
            println!("{} simulations run...", i);
        }
        let birthdays = get_birthdays(num_birthdays);
        if get_match(&birthdays).is_some() {
            sim_match += 1;
        }
    }
    println!("100,000 simulations run.");

    // Display simulation results:
    let probability = (sim_match as f64 / 100_000.0) * 100.0;
    println!(
        "Out of 100,000 simulations of {} people, there was a matching birthday in that group {} times.",
        num_birthdays, sim_match
    );
    println!(
        "This means that {} people have a {:.2}% chance of having a matching birthday in their group.",
        num_birthdays, probability
    );
    println!("That's probably more than you would think!");
}
