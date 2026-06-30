use crate::editor::{
    buffer::Buffer,
    constants::{
        EDITOR_NAME, EDITOR_VERSION, HEADER_DISPLAY_DIVIDER, SIDE_EDITOR_CHAR,
        TERMINAL_INIT_POSITION, TERMINAL_NEW_LINE,
    },
    terminal::Terminal,
};

#[derive(Debug)]
pub struct View {
    buffer: Buffer,
}

impl View {
    pub fn new() -> Self {
        Self {
            buffer: Buffer::new(),
        }
    }

    pub fn render(&self) -> Result<(), std::io::Error> {
        self.draw_rows()?;
        Ok(())
    }

    fn draw_rows(&self) -> Result<(), std::io::Error> {
        // Stdout buffer
        let (columns, rows) = Terminal::size()?;
        let header_row: u16 = rows / HEADER_DISPLAY_DIVIDER;
        Terminal::hide_cursor()?;
        Terminal::move_cursor_to(TERMINAL_INIT_POSITION)?;

        // Fill left buffer with Chars
        for i in 0..rows {
            // Always clear line
            Terminal::clear_line()?;

            if let Some(line) = self.buffer.lines.get(i as usize) {
                Terminal::print(&line).unwrap();
                continue;
            }

            if i == header_row {
                Self::draw_header_line(columns)?
            } else {
                Self::draw_empty_line()?
            }

            // Print eol if not matched height
            if i + 1 < rows {
                Terminal::print(TERMINAL_NEW_LINE)?;
            }
        }

        // Move home again
        Terminal::move_cursor_to(TERMINAL_INIT_POSITION)?;
        Terminal::show_cursor()?;

        // Flush buffer
        Terminal::flush()?;

        Ok(())
    }

    fn draw_header_line(columns: u16) -> Result<(), std::io::Error> {
        let header_text = format!("{}-{}", EDITOR_NAME, EDITOR_VERSION);
        let padding = columns.saturating_sub(header_text.len() as u16) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1) as usize);

        Terminal::print(&format!("{}{}{}", SIDE_EDITOR_CHAR, spaces, header_text))?;
        Ok(())
    }

    fn draw_empty_line() -> Result<(), std::io::Error> {
        Terminal::print(&format!("{}", SIDE_EDITOR_CHAR))?;
        Ok(())
    }
}
