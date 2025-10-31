use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::io::Result;
use std::time::Duration;

#[derive(Debug, Clone)]
pub enum InputEvent {
    Char(char),
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

    pub fn read(&self) -> Result<Option<InputEvent>> {
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    return Ok(None);
                }
                match key.code {
                    KeyCode::Char(c) => Ok(Some(InputEvent::Char(c))),
                    KeyCode::Backspace => Ok(Some(InputEvent::Backspace)),
                    KeyCode::Enter => Ok(Some(InputEvent::Enter)),
                    KeyCode::Esc => Ok(Some(InputEvent::Esc)),
                    _ => Ok(None),
                }
            } else {
                Ok(None)
            }
        } else {
            Ok(Some(InputEvent::Tick))
        }
    }
}
