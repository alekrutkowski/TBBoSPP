// Converted from Python version with 'ChatGPT 4o'

use rand::seq::SliceRandom;
use std::io;
use std::process;

const HEARTS: &str = "♥";
const DIAMONDS: &str = "♦";
const SPADES: &str = "♠";
const CLUBS: &str = "♣";
const BACKSIDE: &str = "backside";

fn main() {
    println!("Blackjack, by Al Sweigart");

    println!("
        Rules:
        Try to get as close to 21 without going over.
        Kings, Queens, and Jacks are worth 10 points.
        Aces are worth 1 or 11 points.
        Cards 2 through 10 are worth their face value.
        (H)it to take another card.
        (S)tand to stop taking cards.
        On your first play, you can (D)ouble down to increase your bet
        but must hit exactly one more time before standing.
        In case of a tie, the bet is returned to the player.
        The dealer stops hitting at 17.");

    let mut money = 5000;
    while money > 0 {
        println!("Money: {}", money);
        let bet = get_bet(money);

        let mut deck = get_deck();
        let mut dealer_hand = vec![deck.pop().unwrap(), deck.pop().unwrap()];
        let mut player_hand = vec![deck.pop().unwrap(), deck.pop().unwrap()];

        println!("Bet: {}", bet);

        // Player's turn
        loop {
            display_hands(&player_hand, &dealer_hand, false);

            if get_hand_value(&player_hand) > 21 {
                println!("You busted!");
                money -= bet;
                break;
            }

            let move_choice = get_move(&player_hand, money - bet);

            if move_choice == "D" {
                let additional_bet = get_bet(std::cmp::min(bet, money - bet));
                money -= additional_bet;
                println!("Bet increased to {}.", bet + additional_bet);
            }

            if move_choice == "H" || move_choice == "D" {
                let new_card = deck.pop().unwrap();
                println!("You drew a {} of {}.", new_card.0, new_card.1);
                player_hand.push(new_card);

                if get_hand_value(&player_hand) > 21 {
                    println!("You busted!");
                    money -= bet;
                    break;
                }
            }

            if move_choice == "S" || move_choice == "D" {
                break;
            }
        }

        // Dealer's turn
        if get_hand_value(&player_hand) <= 21 {
            while get_hand_value(&dealer_hand) < 17 {
                println!("Dealer hits...");
                dealer_hand.push(deck.pop().unwrap());
                display_hands(&player_hand, &dealer_hand, false);

                if get_hand_value(&dealer_hand) > 21 {
                    println!("Dealer busted! You win ${}!", bet);
                    money += bet;
                    break;
                }
            }
        }

        display_hands(&player_hand, &dealer_hand, true);

        let player_value = get_hand_value(&player_hand);
        let dealer_value = get_hand_value(&dealer_hand);

        if dealer_value > 21 || (player_value <= 21 && player_value > dealer_value) {
            println!("You won ${}!", bet);
            money += bet;
        } else if player_value < dealer_value || player_value > 21 {
            println!("You lost!");
            money -= bet;
        } else {
            println!("It's a tie, the bet is returned to you.");
        }
    }
    println!("You're out of money. Thanks for playing!");
}

fn get_bet(max_bet: i32) -> i32 {
    loop {
        println!("How much do you bet? (1-{}, or QUIT)", max_bet);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        if input.trim().eq_ignore_ascii_case("QUIT") {
            println!("Thanks for playing!");
            process::exit(0);
        }

        if let Ok(bet) = input.trim().parse::<i32>() {
            if bet >= 1 && bet <= max_bet {
                return bet;
            }
        }
    }
}

fn get_deck() -> Vec<(String, &'static str)> {
    let mut deck = vec![];

    for &suit in &[HEARTS, DIAMONDS, SPADES, CLUBS] {
        for rank in 2..=10 {
            deck.push((rank.to_string(), suit));
        }
        for &rank in &["J", "Q", "K", "A"] {
            deck.push((rank.to_string(), suit));
        }
    }
    let mut rng = rand::thread_rng();
    deck.shuffle(&mut rng);
    deck
}

fn display_hands(player_hand: &[(String, &str)], dealer_hand: &[(String, &str)], show_dealer_hand: bool) {
    if show_dealer_hand {
        println!("DEALER: {}", get_hand_value(dealer_hand));
        display_cards(dealer_hand);
    } else {
        println!("DEALER: ???");
        display_cards(&[("backside".to_string(), BACKSIDE)].iter().chain(&dealer_hand[1..]).cloned().collect::<Vec<_>>());
    }

    println!("PLAYER: {}", get_hand_value(player_hand));
    display_cards(player_hand);
}

fn get_hand_value(cards: &[(String, &str)]) -> i32 {
    let mut value = 0;
    let mut num_aces = 0;

    for card in cards {
        match card.0.as_str() {
            "A" => num_aces += 1,
            "K" | "Q" | "J" => value += 10,
            rank => value += rank.parse::<i32>().unwrap_or(0),
        }
    }

    value += num_aces;
    for _ in 0..num_aces {
        if value + 10 <= 21 {
            value += 10;
        }
    }

    value
}

fn display_cards(cards: &[(String, &str)]) {
    let mut rows = vec![String::new(), String::new(), String::new(), String::new(), String::new()];

    for card in cards {
        rows[0].push_str(" ___  "); // Top of the card
        if card.0 == "backside" {
            // Card backside representation
            rows[1].push_str("|## | ");
            rows[2].push_str("|###| ");
            rows[3].push_str("|_##| ");
        } else {
            // Card front representation
            let (rank, suit) = card;
            rows[1].push_str(&format!("|{} | ", rank.pad_to_width(2))); // Rank on left
            rows[2].push_str(&format!("| {} | ", suit)); // Suit in the center
            rows[3].push_str(&format!("|_{}| ", rank.pad_to_width_with_char(2, '_'))); // Rank on the right, with underscores
        }
    }

    for row in rows {
        println!("{}", row);
    }
}

// Helper functions for padding, which isn't as straightforward in Rust
trait PadToWidth {
    fn pad_to_width(&self, width: usize) -> String;
    fn pad_to_width_with_char(&self, width: usize, pad_char: char) -> String;
}

impl PadToWidth for String {
    fn pad_to_width(&self, width: usize) -> String {
        let mut padded = self.clone();
        while padded.len() < width {
            padded.push(' ');
        }
        padded
    }

    fn pad_to_width_with_char(&self, width: usize, pad_char: char) -> String {
        let mut padded = self.clone();
        while padded.len() < width {
            padded.insert(0, pad_char);
        }
        padded
    }
}

fn get_move(player_hand: &[(String, &str)], money: i32) -> String {
    loop {
        let options = if player_hand.len() == 2 && money > 0 {
            vec!["(H)it", "(S)tand", "(D)ouble down"]
        } else {
            vec!["(H)it", "(S)tand"]
        };

        println!("{}", options.join(", "));
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().to_uppercase().as_str() {
            "H" | "S" | "D" => return input.trim().to_uppercase(),
            _ => continue,
        }
    }
}
