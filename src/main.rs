use std::{env};
use std::fs::File;
use crossterm::{Result};

mod action;
mod editor;
use editor::{Editor, EditorMode};

fn main() -> Result<()> {

    let mut args = env::args().skip(1);
    let current_dir = std::env::current_dir().unwrap();
    
    let mut file = {
        if let Some(file) = args.next() {
            let path = current_dir.join(file);
            if path.exists() {
                File::open(path).unwrap()
            } else {
                File::create(path).unwrap()
            } 
        } else {
            File::create(current_dir.join(std::path::Path::new("unnamed.txt"))).unwrap()
        }
    };
    
    
    
    // Entering the editor
    let mut editor = Editor::init().unwrap();


    editor.print_file(&mut file)?;
    while !editor.should_quit {
        use crossterm::event;
        match editor.get_mode() {
            EditorMode::Normal => {
                match event::read()? {
                    event::Event::Key(event::KeyEvent{code : event::KeyCode::Tab,..}) => {
                            editor.switch_mode(EditorMode::Command);
                    },
                    event::Event::Key(event::KeyEvent{code : event::KeyCode::Char('e'),..}) => {
                            editor.switch_mode(EditorMode::Command);
                    },
                    _ => continue,
                }
            },
            EditorMode::Command => {
                let mut command_buffer = String::new();
                let mut command_validated = false;
                let mut command_canceled = false;
                while !command_validated && !command_canceled {
                    match event::read()? {
                        event::Event::Key(event::KeyEvent{code : event::KeyCode::Esc,..}) => {
                            editor.switch_mode(EditorMode::Normal);
                            command_canceled = true;
                        },
                        event::Event::Key(event::KeyEvent{code : event::KeyCode::Enter,..}) => {
                            editor.switch_mode(EditorMode::Normal);
                            command_validated = true;
                        },
                        event::Event::Key(event::KeyEvent{code : event::KeyCode::Char(c),..}) => command_buffer.push(c),
                        _ => continue,
                    }
                }
                if command_validated {
                    if command_buffer == 'q'.to_string() {
                        editor.should_quit = true;
                    }
                }
            },
            //_ => continue,
        }

    }
    Ok(())
}
