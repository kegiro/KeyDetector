use crossterm::{
    cursor::MoveTo,
    event::{self, KeyCode},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use std::io::{stdout};
use std::time::Duration;

fn main() {
    intro();

    let mut last_pressed: Option<char> = None;

    refresh_screen(last_pressed);

    loop {
        // Check if a key is pressed
        if event::poll(Duration::from_millis(10)).unwrap() {
            if let Ok(event::Event::Key(key_event)) = event::read() {
                match key_event.code {
                    KeyCode::Char(c) => {
                        if Some(c) != last_pressed {
                            last_pressed = Some(c);
                            refresh_screen(last_pressed);
                        }
                    }
                    KeyCode::Esc => break, // Exit
                    _ => {}
                }
            }
        }
    }
}

fn intro() {
    println!("Hi, my name is Kegiro.");
    println!("I'm a beginner at Rust. But today I'm here because this is my first project.");
    println!("I hope you like it.");
    println!("\nPress any key to continue...");
    wait_for_key();
}

fn display_alphabet(highlight: Option<char>) {
    let mut stdout = stdout();
    
    for c in 'a'..='z' {
        if Some(c) == highlight {
            execute!(
                stdout,
                SetForegroundColor(Color::Red),
                Print(format!("{} ", c)),
                ResetColor
            )
            .unwrap();
        } else {
            print!("{} ", c);
        }
    }
    println!();
}

fn refresh_screen(highlight: Option<char>) {
    let mut stdout = stdout();
    execute!(
        stdout,
        Clear(ClearType::All),
        MoveTo(0, 0)
    )
    .unwrap();
    
    println!("Press a key to highlight it. Press 'Esc' to exit.\n");
    display_alphabet(highlight);
}

fn wait_for_key() {
    loop {
        if event::poll(Duration::from_millis(10)).unwrap() {
            if let Ok(event::Event::Key(_)) = event::read() {
                break; // Exit
            }
        }
    }
}
