#![allow(non_snake_case)]
#![allow(unused)]

use crate::tescrust::{io::data::Crust, tui::view::TuiCtx};
use std::io;
use std::io::Write;

use crossterm::event::{read, KeyEventKind};

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
                })) = event::read()
                {
                        return Ok(c);
                }
        }
}

enum KeyBinds {}

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
