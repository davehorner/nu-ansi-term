extern crate ansi_term;
extern crate term_size;
extern crate crossterm;
extern crate rand;

use ansi_term::{Style, Colour};
use crossterm::event::{self, Event, KeyCode};
use rand::Rng;
use std::io::{Result, Write};
use std::thread::sleep;
use std::time::{Duration, Instant};

const DELAY_MS: u64 = 100;

fn main() -> Result<()> {
    let start_time = Instant::now();

    // Frequency modifiers
    let mut freq_r = 2.0;
    let mut freq_g = 2.5;
    let mut freq_b = 3.0;

    // Offset to vary the color pattern dynamically
    let offset = 0.0;

    // Time factor for animation speed
    let mut time_factor = 1.0;

    // Initialize random number generator
    let mut rng = rand::thread_rng();

    loop {
        // Check for terminal dimensions
        if let Some((width, height)) = term_size::dimensions() {
            let width = width as i32;
            let height = height as i32;

            // Read key events and adjust frequencies, offset, or time factor accordingly
            while event::poll(Duration::from_millis(1))? {
                if let Event::Key(key_event) = event::read()? {
                    match key_event.code {
                        KeyCode::Up => {
                            freq_r += 0.1;
                            freq_g += 0.1;
                            freq_b += 0.1;
                        }
                        KeyCode::Down => {
                            freq_r -= 0.1;
                            freq_g -= 0.1;
                            freq_b -= 0.1;
                        }
                        KeyCode::Left => {
                            time_factor -= 0.1;
                            if time_factor < 0.1 { time_factor = 0.1; } // Prevent going negative or zero
                        }
                        KeyCode::Right => {
                            time_factor += 0.1;
                        }
                        KeyCode::Esc => return Ok(()), // Exit on Esc key
                        _ => {}
                    }
                }
            }

            // Randomly select a few positions to update
            for _ in 0..50 { // Update 50 random positions per frame
                let row = rng.gen_range(0..height);
                let col = rng.gen_range(0..width);

                // Adjust the elapsed time by the time factor
                let elapsed = start_time.elapsed().as_secs_f32() * time_factor;
                let r = (((row + col) as f32 * freq_r + elapsed + offset).sin() * 127.5 + 127.5) as u8;
                let g = (((row + col) as f32 * freq_g + elapsed + offset).cos() * 127.5 + 127.5) as u8;
                let b = (((row + col) as f32 * freq_b + elapsed + offset).sin() * 127.5 + 127.5) as u8;

                // Move the cursor to the random position and print the color block
                print!("\x1B[{};{}H{}", row + 1, col + 1, Style::default().on(Colour::RGB(r, g, b)).paint(" "));
            }

            // Flush output to make changes visible immediately
            print!("\x1B[?25l"); // Optionally hide the cursor for aesthetics
            std::io::stdout().flush().unwrap();

            // Delay for the next update
            sleep(Duration::from_millis(DELAY_MS));
        } else {
            eprintln!("Could not get terminal dimensions.");
            break Ok(());
        }
    }
}

