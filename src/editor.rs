use crossterm::{self, terminal, cursor, Result, execute, queue, style};
use event::EventHandler;
use std::{fmt::Display, fs::File};
use std::io::{Stdout, BufReader, BufRead, Write};
use crate::action::{self, Quit};

mod event;

#[derive(Clone, Copy)]
pub enum EditorMode {
    Normal,
    //Edit,
    Command,
    //Search,
}

pub struct Editor {
    pub stdout : Stdout,
    pub should_quit : bool,
    mode : EditorMode,
    view_size : (u16, u16),
}

impl Editor {
    
    pub fn init() -> Result<Self> {
        let mut stdout = std::io::stdout();
        terminal::enable_raw_mode()?;
        execute!(stdout, terminal::EnterAlternateScreen)?;
        execute!(stdout, terminal::Clear(terminal::ClearType::All), cursor::MoveTo(0u16,0u16))?;
        let view_size = terminal::size()?;
        Ok(Self {
            stdout : stdout,
            should_quit : false,
            mode : EditorMode::Normal,
            view_size,
        })
    }

    pub fn switch_mode(&mut self, mode : EditorMode) {
        self.mode = mode;
    }

    pub fn get_mode(&self) -> EditorMode {
        self.mode
    }

    pub fn print_file(&mut self, file : &mut File) -> Result<()> {
        let mut lines = BufReader::new(file).lines();
        let mut counter = 1;
        while let Some(Ok(line)) = lines.next() {
            if counter > self.view_size.1 {
                break;
            }
            queue!(self.stdout, LineNumber(counter))?;
            queue!(self.stdout, style::Print(line), cursor::MoveToNextLine(1u16))?;
            counter += 1;
        }
        self.stdout.flush()?;
        Ok(())
    }

}

impl Drop for Editor {
    fn drop(&mut self) {
        execute!(self.stdout, Quit).unwrap();
    }

}


