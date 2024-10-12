extern crate ansi_term;
extern crate term_size;
extern crate crossterm;

use ansi_term::{Style, Colour};
use crossterm::{cursor, execute, terminal, event::{self, Event, KeyCode}};
use std::io::{stdout, Result, Write};
use std::thread::sleep;
use std::time::{Duration, Instant};

const DELAY_MS: u64 = 100;

fn main() -> Result<()> {
    let mut stdout = stdout();
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

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
                        KeyCode::Esc |
                        KeyCode::Char('q') | 
                        KeyCode::Char('Q')
                         => {
                            execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen)?;
                            return Ok(()); // Exit on Esc key
                        }
                        _ => {}
                    }
                }
            }

            // Move the cursor to the top left corner
            execute!(stdout, cursor::MoveTo(0, 0))?;

            // Generate the color gradient with updated frequencies and offset
            for row in 0..height {
                for col in 0..width {
                    let elapsed = start_time.elapsed().as_secs_f32();

                    let r = (((row + col) as f32 * freq_r + elapsed + offset).sin() * 127.5 + 127.5) as u8;
                    let g = (((row + col) as f32 * freq_g + elapsed + offset).cos() * 127.5 + 127.5) as u8;
                    let b = (((row + col) as f32 * freq_b + elapsed + offset).sin() * 127.5 + 127.5) as u8;

                    // Check if the current position matches the text position
                    if row == text_row && col >= text_col && col < text_col + text.len() as i32 {
                        let text_color = Colour::Fixed(15); // White color for the text
                        print!("{}", Style::new().fg(text_color).on(Colour::RGB(r, g, b)).paint("Hello, World!".chars().nth((col - text_col) as usize).unwrap().to_string()));
                    } else {
                        print!("{}", Style::default().on(Colour::RGB(r, g, b)).paint(" "));
                    } 
                }
                print!("\n");
            }

            // Flush the output buffer to apply all changes
            stdout.flush()?;

            // Delay for the next frame
            sleep(Duration::from_millis(DELAY_MS));
        } else {
            eprintln!("Could not get terminal dimensions.");
            break;
        }
    }

    execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen)?;
    Ok(())
}
