use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{self, ClearType},
};
use std::io;
use std::time::Duration;

pub struct Terminal {
    pub width: u16,
    pub height: u16,
}

impl Terminal {
    /// Enter alternate screen, enable raw mode, hide cursor.
    pub fn init() -> io::Result<Self> {
        terminal::enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(
            stdout,
            terminal::EnterAlternateScreen,
            cursor::Hide,
            terminal::Clear(ClearType::All),
        )?;
        let (w, h) = terminal::size()?;
        Ok(Terminal {
            width: w,
            height: h,
        })
    }

    /// Restore terminal to original state.
    pub fn cleanup() {
        let mut stdout = io::stdout();
        let _ = execute!(
            stdout,
            cursor::Show,
            terminal::LeaveAlternateScreen,
        );
        let _ = terminal::disable_raw_mode();
    }

    /// Poll for a key event. Returns Some(action) if quit or resize detected.
    pub fn poll_event(&mut self, timeout: Duration) -> Option<Action> {
        if event::poll(timeout).unwrap_or(false) {
            if let Ok(ev) = event::read() {
                return match ev {
                    Event::Key(KeyEvent {
                        code: KeyCode::Char('q'),
                        ..
                    })
                    | Event::Key(KeyEvent {
                        code: KeyCode::Esc, ..
                    }) => Some(Action::Quit),
                    Event::Key(KeyEvent {
                        code: KeyCode::Char('c'),
                        modifiers: KeyModifiers::CONTROL,
                        ..
                    }) => Some(Action::Quit),
                    Event::Resize(w, h) => {
                        self.width = w;
                        self.height = h;
                        Some(Action::Resize)
                    }
                    _ => None,
                };
            }
        }
        None
    }
}

pub enum Action {
    Quit,
    Resize,
}
