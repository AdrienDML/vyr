use crossterm::Command;
use crossterm::style::SetBackgroundColor;


pub struct Panel {
    size : (u16, u16),
    background : style::SetBackgroundColor,
}
