use std::io::{Result, stdout};

use crossterm::{
    cursor::{MoveDown, MoveTo},
    event::{Event, KeyCode, KeyEvent, KeyModifiers, read},
    execute,
    terminal::{Clear, disable_raw_mode, enable_raw_mode},
};

use crate::editor::constants::{KEYBOARD_CLOSE_SHORTCUT, SIDE_EDITOR_CHAR, TERMINATION_MESSAGE};

#[derive(Debug)]
pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn new() -> Editor {
        Editor { should_quit: false }
    }

    // Handle screen clean
    fn clear_screen() -> Result<()> {
        let mut stdout = stdout();
        execute!(stdout, Clear(crossterm::terminal::ClearType::All))
    }

    // Editor initialization
    fn initialize() -> Result<()> {
        enable_raw_mode()?;
        Self::clear_screen();
        Self::draw_rows()
    }

    // Termination actions
    fn terminate() -> Result<()> {
        disable_raw_mode()
    }

    pub fn run(&mut self) {
        Self::initialize().unwrap();
        let result = self.repl();
        Self::terminate().unwrap();
        result.unwrap()
    }

    fn repl(&mut self) -> Result<()> {
        loop {
            let mut event = read()?;

            self.evaluate_event(&mut event);
            self.refresh_screen()?;

            if self.should_quit {
                break;
            }
        }
        Ok(())
    }

    fn refresh_screen(&self) -> Result<()> {
        // Quit if it should not be active
        if self.should_quit == true {
            Self::clear_screen()?;
            print!("{TERMINATION_MESSAGE}");
        }

        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Event::Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            //Handle for now  code + modifier
            match code {
                KeyCode::Char(KEYBOARD_CLOSE_SHORTCUT) if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                _ => (),
            }
        }
    }

    fn draw_rows() -> Result<()> {
        let (rows, _cols) = crossterm::terminal::size()?;

        // Move home
        execute!(stdout(), MoveTo(0, 0))?;

        // Fill with Chars
        for _ in 0..rows {
            print!("{SIDE_EDITOR_CHAR}\r");
            execute!(stdout(), MoveDown(1))?;
        }

        // Move home again
        execute!(stdout(), MoveTo(1, 0))?;
        Ok(())
    }
}
