use crossterm::{ExecutableCommand, cursor, style::{self, Color, PrintStyledContent}, terminal, QueueableCommand};
use rand::Rng;
use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;

// Constants
const NUMBER_OF_LOGOS: usize = 5;
const PAUSE_AMOUNT: u64 = 200; // milliseconds

// Directions
#[derive(Copy, Clone, PartialEq)]
enum Direction {
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

struct Logo {
    color: Color,
    x: u16,
    y: u16,
    direction: Direction,
}

fn main() {
    let mut stdout = stdout();
    let (width, height) = terminal::size().expect("Failed to get terminal size");
    let width = width - 1; // Prevent auto newline on Windows

    let colors = [
        Color::Red,
        Color::Green,
        Color::Yellow,
        Color::Blue,
        Color::Magenta,
        Color::Cyan,
        Color::White,
    ];

    let mut rng = rand::thread_rng();
    let mut logos = vec![];

    // Initialize logos
    for _ in 0..NUMBER_OF_LOGOS {
        let mut x = rng.gen_range(1..width - 4);
        if x % 2 == 1 {
            x -= 1;
        }
        let y = rng.gen_range(1..height - 4);
        let direction = match rng.gen_range(0..4) {
            0 => Direction::UpRight,
            1 => Direction::UpLeft,
            2 => Direction::DownRight,
            _ => Direction::DownLeft,
        };
        let color = colors[rng.gen_range(0..colors.len())];
        logos.push(Logo { color, x, y, direction });
    }

    let mut corner_bounces = 0;

    loop {
        stdout.execute(cursor::Hide).unwrap();
        stdout.execute(terminal::Clear(terminal::ClearType::All)).unwrap();

        // Handle each logo's movement
        for logo in &mut logos {
            stdout.queue(cursor::MoveTo(logo.x, logo.y)).unwrap();
            stdout.queue(PrintStyledContent(style::style("   "))).unwrap();

            let original_direction = logo.direction;

            // Detect collisions with corners
            match (logo.x, logo.y) {
                (0, 0) => {
                    logo.direction = Direction::DownRight;
                    corner_bounces += 1;
                }
                (0, y) if y == height - 1 => {
                    logo.direction = Direction::UpRight;
                    corner_bounces += 1;
                }
                (x, 0) if x == width - 3 => {
                    logo.direction = Direction::DownLeft;
                    corner_bounces += 1;
                }
                (x, y) if x == width - 3 && y == height - 1 => {
                    logo.direction = Direction::UpLeft;
                    corner_bounces += 1;
                }
                _ => {}
            }

            // Detect edge bounces and adjust direction
            match (logo.x, logo.direction) {
                (0, Direction::UpLeft) => logo.direction = Direction::UpRight,
                (0, Direction::DownLeft) => logo.direction = Direction::DownRight,
                (x, Direction::UpRight) if x == width - 3 => logo.direction = Direction::UpLeft,
                (x, Direction::DownRight) if x == width - 3 => logo.direction = Direction::DownLeft,
                _ => {}
            }
            match (logo.y, logo.direction) {
                (0, Direction::UpLeft) => logo.direction = Direction::DownLeft,
                (0, Direction::UpRight) => logo.direction = Direction::DownRight,
                (y, Direction::DownLeft) if y == height - 1 => logo.direction = Direction::UpLeft,
                (y, Direction::DownRight) if y == height - 1 => logo.direction = Direction::UpRight,
                _ => {}
            }

            // Change color if direction changed
            if logo.direction != original_direction {
                logo.color = colors[rng.gen_range(0..colors.len())];
            }

            // Move the logo
            match logo.direction {
                Direction::UpRight => {
                    logo.x += 2;
                    logo.y -= 1;
                }
                Direction::UpLeft => {
                    logo.x -= 2;
                    logo.y -= 1;
                }
                Direction::DownRight => {
                    logo.x += 2;
                    logo.y += 1;
                }
                Direction::DownLeft => {
                    logo.x -= 2;
                    logo.y += 1;
                }
            }

            // Draw the logo at the new position
            stdout.queue(cursor::MoveTo(logo.x, logo.y)).unwrap();
            stdout.queue(style::SetForegroundColor(logo.color)).unwrap();
            stdout.queue(PrintStyledContent(style::style("DVD"))).unwrap();
        }

        // Display corner bounces
        stdout.queue(cursor::MoveTo(5, 0)).unwrap();
        stdout.queue(style::SetForegroundColor(Color::White)).unwrap();
        stdout.queue(PrintStyledContent(style::style(format!("Corner bounces: {}", corner_bounces)))).unwrap();

        stdout.flush().unwrap();
        sleep(Duration::from_millis(PAUSE_AMOUNT));
    }
}
