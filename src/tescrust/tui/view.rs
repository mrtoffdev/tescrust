#![allow(non_snake_case)]
#![allow(unused)]

use crossterm::{execute, terminal};
use std::io;
use std::io::Stdout;

// ================== models ==================
pub struct TuiCtx {
        window: Stdout,

        // ---------- resolution ----------
        ui_x: u16,      // Width
        ui_y: u16,      // Height

        // ---------- Scroll View ----------
        buff_i: char,   // ScrollView Index
        buff_s: i32,    // ScrollView Buffer Min
        buff_e: i32,    // ScrollView Buffer Max

        // ---------- Indices ----------
        n_focus: u8,    // Panel Index
        n_key: u8,      // Nav Key (getch)

        // ---------- Operations ----------
        h_mode: u8,     // Getch Handler Mode
        o_mode: u8,     // RW Operation Mode
        sid: u8,        // Session Panel ID

        // ---------- Configuration ----------

        /// Interface
        wrap_lim: i32,

        /// Color Scheme
        c0: u32,         // Foreground
        c5: u32,         // Background

        c1: u32,         // Highlight
        c4: u32,         // Accent

        c2: u32,         // Error
        c3: u32,         // Success
}

impl TuiCtx {
        pub fn Default() -> Result<Self, io::Error> {
                // ---------- blank ctx ----------
                let mut term = io::stdout();
                execute!(term, terminal::EnterAlternateScreen)?;
                let (ui_x, ui_y) = terminal::size().unwrap();
                execute!(term, terminal::LeaveAlternateScreen)?;
                Ok(TuiCtx {
                        window: term,
                        ui_x,
                        ui_y,
                        buff_i: ' ',
                        n_focus: 0,
                        n_key: 0,
                        h_mode: 0,
                        o_mode: 0,
                        sid: 0,
                        buff_s: 0,
                        buff_e: 0,
                        wrap_lim: 0,

                        c0: 0xFFFFFF,
                        c5: 0x111111,

                        c1: 0xAAAAAA,
                        c4: 0xAADDFF,

                        c2: 0,
                        c3: 0,
                })
        }

        pub fn new() -> Result<Self, io::Error> {
                // ---------- blank ctx ----------
                TuiCtx::Default()
        }
}

enum DynSize {
        Parent,
        Block,
        Usize(u8),
}

enum Frame<'p> {
        Merged(&'p Frame<'p>),
        New(u8),
}

/// The core building block of every TUI component present
/// in a window
struct ComponentBlock<'F> {
        frame: Frame<'F>,
        child: Vec<String>,
}

impl TCComponent for ComponentBlock<'_> {
        fn indent(size: DynSize) {
                let parent: ComponentBlock = ComponentBlock {
                        frame: Frame::New(1),
                        child: vec![],
                };

                let sub: ComponentBlock = ComponentBlock {
                        frame: Frame::Merged(&parent.frame),
                        child: vec![],
                };

                todo!()
        }

        fn whitespace(size: DynSize) {
                todo!()
        }

        fn divider<S>(size: S) {
                todo!()
        }
}

pub fn refresh() {}

pub fn clear() {}

// ================== char utils ==================
trait TCComponent {
        fn indent(size: DynSize);
        fn whitespace(size: DynSize);
        fn divider<S>(size: S);
}

// ================== components ==================
fn graph() -> ComponentBlock<'static> {
        return ComponentBlock {
                frame: Frame::New(1),
                child: vec![],
        };
}

fn table() -> ComponentBlock<'static> {
        return ComponentBlock {
                frame: Frame::New(1),
                child: vec![],
        };
}

fn helper() -> ComponentBlock<'static> {
        return ComponentBlock {
                frame: Frame::New(1),
                child: vec![],
        };
}
