use std::io::{self, stdout, Write};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn run(&mut self) {
        stdout().into_raw_mode().unwrap();

        loop {
            if let Err(err) = self.refresh_screen() {
                die(&err);
            }
            if self.should_quit {
                println!("Exiting...");
                _ = io::stdout().flush();
            }
            if let Err(err) = self.process_keypress() {
                die(&err);
            }
        }
    }

    pub fn default() -> Editor {
        Self { should_quit: false }
    }

    fn process_keypress(&mut self) -> Result<Key, io::Error> {
        let pressed_key = Self::read_key()?;

        match pressed_key {
            Key::Ctrl('c') => {
                self.should_quit = true;
                Ok(pressed_key)
            }
            _ => {
                print!("{:?}", pressed_key);
                Ok(pressed_key)
            }
        }
    }

    fn read_key() -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        io::stdout().flush()
    }
}

fn die(e: &std::io::Error) {
    print!("{}", termion::clear::All);
    panic!("{}", e);
}
