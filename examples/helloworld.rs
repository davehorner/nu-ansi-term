extern crate ansi_term;
extern crate term_size;
extern crate crossterm;

use ansi_term::{Style, Colour};
use crossterm::event::{self, Event, KeyCode};
use std::thread::sleep;
use std::time::{Duration, Instant};
use std::io::Result;

const DELAY_MS: u64 = 100;

fn main() -> Result<()> {
    let start_time = Instant::now();

    // Frequency modifiers
    let mut freq_r = 2.0;
    let mut freq_g = 2.5;
    let mut freq_b = 3.0;

    // Offset to vary the color pattern dynamically
    let mut offset = 0.0;

    loop {
        // Check for terminal dimensions
        if let Some((width, height)) = term_size::dimensions() {
            let width = width as i32;
            let height = height as i32;


            // Calculate the position for "Hello, World!" in the center
            let text = "Hello, World!";
            let text_col = width / 2 - (text.len() as i32 / 2);
            let text_row = height / 2; 

            // Read key events and adjust frequencies or offset accordingly
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
                            offset -= 0.1;
                        }
                        KeyCode::Right => {
                            offset += 0.1;
                        }
                        KeyCode::Esc => return Ok(()), // Exit on Esc key
                        _ => {}
                    }
                }
            }

            // Generate the color gradient with updated frequencies and offset
            for row in 0..height {
                for col in 0..width {
                    let elapsed = start_time.elapsed().as_secs_f32();

                    let r = (((row + col) as f32 * freq_r + elapsed + offset).sin() * 127.5 + 127.5) as u8;
                    let g = (((row + col) as f32 * freq_g + elapsed + offset).cos() * 127.5 + 127.5) as u8;
                    let b = (((row + col) as f32 * freq_b + elapsed + offset).sin() * 127.5 + 127.5) as u8;

                    // Check if the current position matches the text position
                    if row == text_row && col >= text_col && col < text_col + text.len() as i32 {
                        // Print "Hello, World!" with a contrasting color
                        let text_color = Colour::Fixed(15); // White color for the text
                        print!("{}", Style::new().fg(text_color).on(Colour::RGB(r, g, b)).paint("Hello, World!".chars().nth((col - text_col) as usize).unwrap().to_string()));
                    } else {
                        // Print the background gradient
                        print!("{}", Style::default().on(Colour::RGB(r, g, b)).paint(" "));
                    } 
                }
                print!("\n");
            }

            // Delay for the next frame
            sleep(Duration::from_millis(DELAY_MS));
            // print!("\x1B[2J\x1B[H"); // Clear the terminal for the next frame
        } else {
            eprintln!("Could not get terminal dimensions.");
            break;
        }
    }

    Ok(())
}


/*


extern crate ansi_term;
extern crate term_size;

use ansi_term::{Style, Colour};
use std::thread::sleep;
use std::time::{Duration, Instant};

const DELAY_MS: u64 = 100;

fn main() {
    let start_time = Instant::now();

    loop {
        // Retrieve the terminal width and height
        if let Some((width, height)) = term_size::dimensions() {
            for row in 0..height {
                for col in 0..width {
                    let elapsed = start_time.elapsed().as_secs_f32();
                    // Calculate colors with time-based offset for animation
                    let r = ((row as f32 + elapsed * 5.0).sin() * 127.5 + 127.5) as u8;
                    let g = ((col as f32 + elapsed * 3.0).cos() * 127.5 + 127.5) as u8;
                    let b = ((elapsed * 2.0).sin() * 127.5 + 127.5) as u8;

                    print!("{}", Style::default().on(Colour::RGB(r, g, b)).paint(" "));
                }
                print!("\n");
            }
        } else {
            eprintln!("Could not get terminal dimensions.");
            break;
        }

        sleep(Duration::from_millis(DELAY_MS));
        print!("\x1B[2J\x1B[H"); // Clear the terminal before the next frame
    }
}

*/
