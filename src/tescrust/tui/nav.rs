#![allow(non_snake_case)]
#![allow(unused)]

use crate::tescrust::{core::data::Crust, tui::view::TUIEnv};
use std::io;
use std::io::Write;
use std::collections::HashMap;
use phf::{phf_map};

use crossterm::{
        cursor,
        event::{read, KeyEventKind, MouseEvent},
        event::{self, Event, KeyCode, KeyEvent},
        execute, queue, style,
        terminal::{self, ClearType},
        Command,
};


// ============================================
// ========== Keybindings & Mappings ==========
// ============================================

/// An exhaustive list of actions that can be mapped to keys
#[derive(Clone)]
pub enum KeyAction {
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
pub static KEY_MAP: phf::Map<&'static str, KeyAction> = phf_map!{
        // Navigation
        "w" => KeyAction::Up,
        "s" => KeyAction::Down,
        "a" => KeyAction::Left,
        "d" => KeyAction::Right,

        // Operations
        "q" => KeyAction::Quit,
        "r" => KeyAction::Delete,
        "e" => KeyAction::Edit,
        "t" => KeyAction::Create,

        // Debug
        "1" => KeyAction::Print,
};

// A collection of key characters mapped to specific KeyActions (Vim Edition)
pub static KEY_MAP_VIM: phf::Map<&'static str, KeyAction> = phf_map!{
        // Navigation
        "k" => KeyAction::Up,
        "j" => KeyAction::Down,
        "h" => KeyAction::Left,
        "l" => KeyAction::Right,

        // Operations
        "q" => KeyAction::Quit,
        "r" => KeyAction::Delete,
        "e" => KeyAction::Edit,
        "w" => KeyAction::Create,

        // Debug
        "1" => KeyAction::Print,
};

/// Keymap-independent event handler. Used to trigger events according to KeyAction
pub(crate) fn handle_event(key: &str) {

        match KEY_MAP.get(key).cloned().unwrap() {
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

        /// Replacement helper function w/ similar features to _getch()
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
