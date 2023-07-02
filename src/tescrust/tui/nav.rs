#![allow(non_snake_case)]
#![allow(unused)]

use crate::tescrust::{core::data::Crust, tui::view::TuiCtx};
use std::io;
use std::io::Write;
use std::collections::HashMap;

use crossterm::event::{read, KeyEventKind, MouseEvent};

use crossterm::{
        cursor,
        event::{self, Event, KeyCode, KeyEvent},
        execute, queue, style,
        terminal::{self, ClearType},
        Command,
};

fn read_char() -> io::Result<char> {
        loop {
                if let Ok(Event::Key(KeyEvent {
                        code: KeyCode::Char(c),
                        kind: KeyEventKind::Press,
                        modifiers: _,
                        state: _,
                })) = event::read() {
                        return Ok(c);
                }
        }
}

// # Keybindings & Mappings

/// An exhaustive list of actions that can be mapped to keys
pub(crate) enum KeyAction {
        // Navigation
        Up,
        Down,
        Left,
        Right,

        // Actions
        Quit,
        Delete,
        Edit,
        Create,

        // Debug
        Print
}

/// A collection of key characters mapped to specific KeyActions
pub(crate) static KEY_MAP: HashMap<char, KeyAction> = HashMap::from([
        /// Navigation
        ('w', KeyAction::Up),
        ('s', KeyAction::Down),
        ('a', KeyAction::Left),
        ('d', KeyAction::Right),

        // Actions
        ('q', KeyAction::Quit),
        ('r', KeyAction::Delete),
        ('e', KeyAction::Edit),
        ('t', KeyAction::Create),

        // Debug
        ('1', KeyAction::Print),
]);

/// Keymap-independent event handler. Used to trigger events according to KeyAction
pub(crate) fn handle_event(key: &char) {

        match KEY_MAP.get(key).unwrap() {
                // Navigation
                KeyAction::Up => {

                },
                KeyAction::Down => {

                },
                KeyAction::Left => {

                }
                KeyAction::Right => {

                }

                // Operations
                KeyAction::Quit => {

                },
                KeyAction::Delete => {

                },
                KeyAction::Edit => {

                }
                KeyAction::Create => {

                }


                // Debug Operations
                KeyAction::Print => {

                }

                _ => ()
        }
}

// ---------- runtime entry pont ----------
pub(crate) fn iostream_handler(crust: Crust, TuiCtx: TuiCtx) -> io::Result<()> {
        let mut screen = io::stdout();

        execute!(screen, terminal::EnterAlternateScreen)?;

        let menu: &str = r#"[#] Tescrust: TUI Edition [#]"#;

        terminal::enable_raw_mode()?;

        queue!(
                screen,
                cursor::MoveTo(1, 1),
                style::Print(menu),
                cursor::MoveToNextLine(1)
        )?;

        screen.flush()?;

        loop {
                let key = read()?;

                if key == Event::Key(KeyCode::Char('1').into()) {
                        print!("Checked")
                };
                if key == Event::Key(KeyCode::Char('2').into()) {
                        print!("Checked2")
                };
                if key == Event::Key(KeyCode::Char('q').into()) {
                        break;
                };

                queue!(
                        screen,
                        style::ResetColor,
                        cursor::Hide,
                        cursor::MoveToNextLine(1)
                )?;

                screen.flush()?;
        }

        terminal::disable_raw_mode()?;

        screen.flush();

        Ok(())
}
