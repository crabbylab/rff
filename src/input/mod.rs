use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use std::io;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq)]
pub enum InputEvent {
    Key(KeyEvent),
    Backspace,
    Enter,
    Esc,
    Tick,
}

pub struct InputReader;

impl InputReader {
    pub fn new() -> Self {
        Self
    }

    pub fn read(&self) -> io::Result<Option<InputEvent>> {
        if event::poll(Duration::from_millis(50))? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Enter => Ok(Some(InputEvent::Enter)),
                    KeyCode::Esc => Ok(Some(InputEvent::Esc)),
                    KeyCode::Backspace => Ok(Some(InputEvent::Backspace)),
                    _ => Ok(Some(InputEvent::Key(key))),
                },
                _ => Ok(None),
            }
        } else {
            Ok(Some(InputEvent::Tick))
        }
    }
}
