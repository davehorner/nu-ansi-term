extern crate ansi_term;
extern crate crossterm;
extern crate rand;
extern crate nu_ansi_term;

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType},
    event::{self, Event, KeyCode},
    style::{Color, Print, SetBackgroundColor, SetForegroundColor},
};
use rand::Rng;
use std::collections::{HashMap, HashSet};
use std::io::{stdout, Write, Result};
use std::thread::sleep;
use std::time::{Duration, Instant};

const DELAY_MS: u64 = 100;

fn main() -> Result<()> {
    enable_raw_mode()?; // Enable raw mode for precise control
    let mut stdout = stdout();
    execute!(stdout, Hide, Clear(ClearType::All))?;

    let start_time = Instant::now();
    let mut updated_positions = HashSet::new();
    let mut original_screen = HashMap::new();

    // Capture initial screen content
    capture_screen_content(&mut original_screen)?;

    let mut rng = rand::thread_rng();

    loop {
        let (width, height) = size()?; // Get terminal size dynamically
        let width = width as i32;
        let height = height as i32;

        // Handle key events for exiting and other commands
        while event::poll(Duration::from_millis(1))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
                        // restore_screen(&original_screen)?;
                        execute!(stdout, Show)?; // Show cursor on exit
                        disable_raw_mode()?;
                       // Set background to white and text to black on exit
                        let white_bg_black_text = 0xF0; // White background (0xF) and black text (0x0)
                        // SetConsoleTextAttribute(h_console, white_bg_black_text); 
                        execute!(stdout,SetBackgroundColor(Color::Rgb { r:0u8, g:0u8, b:0u8 }));
                       execute!(stdout, SetBackgroundColor(Color::White))?;
            execute!(stdout, crossterm::style::SetForegroundColor(Color::Black))?; 
                       // Get the terminal size to calculate middle-left position
            let (_, height) = size()?;
            let middle_row = height / 2;

            // Move cursor to the middle-left position (X=0, Y=middle_row)
            execute!(stdout, MoveTo(0, middle_row))?;
            println!("Goodbye"); 
                        return Ok(()); // Exit and restore original screen content
                    },
                    _ => {}
                }
            }
        }

        // Update random positions with a color gradient
        for _ in 0..2048 {
            let row = rng.gen_range(0..height);
            let col = rng.gen_range(0..width);

            let pos = (row, col);
            // Skip if already painted in this position
            if updated_positions.contains(&pos) {
                continue;
            }

            // Calculate a color gradient for RGB values
            let r = (row * 255 / height) as u8;
            let g = (col * 255 / width) as u8;
            let b = 127;

            // Print gradient color block at the random position
            execute!(
                stdout,
                MoveTo(col as u16, row as u16),
                SetBackgroundColor(Color::Rgb { r, g, b }),
                Print(" ")
            )?;
            updated_positions.insert(pos);
        }

        stdout.flush()?; // Flush to render changes
        sleep(Duration::from_millis(DELAY_MS));
    }
}

fn capture_screen_content(screen: &mut HashMap<(i32, i32), (Color, char)>) -> Result<()> {
    let (width, height) = size()?;
    for row in 0..height {
        for col in 0..width {
            screen.insert((row as i32, col as i32), (Color::Reset, ' '));
        }
    }
    Ok(())
}

fn restore_screen(screen: &HashMap<(i32, i32), (Color, char)>) -> Result<()> {
    let mut stdout = stdout();
    for (&(row, col), &(color, ch)) in screen {
        execute!(
            stdout,
            MoveTo(col as u16, row as u16),
            SetForegroundColor(color),
            Print(ch)
        )?;
    }
    stdout.flush()?;
    Ok(())
}
