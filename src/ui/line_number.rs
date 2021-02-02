use crossterm::Command;

pub struct LineNumber(u16);
impl Command for LineNumber {
    fn write_ansi(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
        write!(f, "{}", style::Print(self.0))?;
        let nb = (self.0 as f32).log(10.) as i8;
        for _ in nb..3 {
            write!(f, " ")?;
        }
        Ok(())
    }
}

