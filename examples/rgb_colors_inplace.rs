extern crate ansi_term;
extern crate term_size;
extern crate crossterm;
extern crate rand;
extern crate nu_ansi_term;

use ansi_term::{Style as AnsiStyle, Colour};
use crossterm::event::{self, Event, KeyCode};
use nu_ansi_term::{Color, Style};
use rand::Rng;
use std::io::{Result, Write};
use std::thread::sleep;
use std::time::{Duration, Instant};

const DELAY_MS: u64 = 100;

fn main() -> Result<()> {
    let start_time = Instant::now();

    // Frequency modifiers for the random color updates
    let mut freq_r = 2.0;
    let mut freq_g = 2.5;
    let mut freq_b = 3.0;

    // Offset and time factor for the gradient update
    let offset = 0.0;
    let mut time_factor = 1.0;

    // Initialize random number generator
    let mut rng = rand::thread_rng();

    loop {
        // Check for terminal dimensions
        if let Some((width, height)) = term_size::dimensions() {
            let width = width as i32;
            let height = height as i32;

            // Handle key events for modifying frequency and animation speed
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
                            if time_factor < 0.1 { time_factor = 0.1; }
                        }
                        KeyCode::Right => {
                            time_factor += 0.1;
                        }
                        KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => return Ok(()), // Exit on Esc or 'q'
                        _ => {}
                    }
                }
            }

            // Update random positions with a color gradient
            for _ in 0..50 {
                let row = rng.gen_range(0..height);
                let col = rng.gen_range(0..width);

                // Calculate elapsed time for animation speed
                let elapsed = start_time.elapsed().as_secs_f32() * time_factor;

                // Use gradient pattern for RGB calculations
                let r = (row * 255 / height) as u8;
                let g = (col * 255 / width) as u8;
                let b = (((row + col) as f32 * freq_b + elapsed + offset).sin() * 127.5 + 127.5) as u8;

                // Print gradient color block at the random position
                print!("\x1B[{};{}H{}", row + 1, col + 1, Style::default().on(Color::Rgb(r, g, b)).paint(" "));
            }

            // Hide cursor and flush to render changes
            print!("\x1B[?25l");
            std::io::stdout().flush().unwrap();

            // Delay before the next update
            sleep(Duration::from_millis(DELAY_MS));
        } else {
            eprintln!("Could not get terminal dimensions.");
            break Ok(());
        }
    }
}

