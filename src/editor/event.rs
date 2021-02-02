use std::collections::HashMap;
use super::EditorMode;


pub struct EventHandler {
    bind_maps : HashMap<String, Event>,
    current_selection : Selector, 
    current_envent : Event,
    counter : u32,
}




pub enum Event {
    Window(Window),
    Edit(Edit),
    Move(Move),
    ChangeModeEvent(EditorMode),
}

pub enum Window {
    Quit,
    New,
    HSplit,
    VSplit,
}

pub struct Selector(Vec<Move>);

pub enum Move {
    Next (Scope),
    Prev (Scope),
}

pub enum Edit {
    Indent, Unindent,
    Cut, Insert,
    Copy, Paste,
    Move,
    Undo, Redo,
}



/// Enum that represent the differents text elements
pub enum Scope {
    Line,
    Word, 
    Char,
    None,
}
