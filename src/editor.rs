use crossterm::{
    event::{
        Event::Key,
        KeyCode::{self},
        read,
    },
    terminal::disable_raw_mode,
};

#[derive(Debug)]
pub struct Editor {}

impl Editor {
    pub fn new() -> Editor {
        Editor {}
    }

    pub fn run(&self) {
        loop {
            match read() {
                Ok(Key(event)) => match event.code {
                    KeyCode::Char('q') => {
                        break;
                    }
                    _ => {
                        println!("{}", event.code)
                    }
                },
                Err(err) => println!("Error: {}", err),
                _ => (),
            }
        }
        disable_raw_mode().unwrap();
    }
}
