use crossterm::Command;
use crossterm::terminal;

pub struct Quit;
impl Command for Quit {
    fn write_ansi(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
        terminal::LeaveAlternateScreen.write_ansi(f)?;
        terminal::disable_raw_mode().unwrap();
        Ok(())
    }
}

