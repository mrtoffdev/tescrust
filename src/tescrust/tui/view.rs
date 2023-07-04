#![allow(non_snake_case)]
#![allow(unused)]

use crossterm::{execute, terminal};
use std::io;
use std::io::Stdout;
use uuid::Uuid;
use crate::tescrust::tui::{
        view::DynSize::Usize,
        component::base::ComponentBlock,
};

/// ## Simplified TUI Render Process:
/// ### The TUI is built in a three level process
///     1. An abstract tree which contains the interface layout is
///     bisected into individual abstract components
///
///     2. Each individual component is rendered into a generic
///     ComponentBlock, which is a struct that contains a vector
///     that represents a frame of the characters that are sent to
///     the TUI handler to be consolidated with other UI components.
///
///     3. Consolidate all the char elements in all the ComponentBlocks
///     into one frame that can be rendered into Stdout


// ================== models ==================

/// ## TUI Context
/// This is the highest level representation of state within the TUI.
/// Data held within this struct is used by different components within
/// the layout.
///
/// Properties that are dynamically changed by the backend are prefixed
/// with **`in_mut`**, while properties that can be mutated by interrupts
/// are prefixed with **`out_mut`**. Properties that can be mutated by both
/// the backend and the user are prefixed with **`in_out_mut`**.
pub struct TUIEnv {
        in_window: Stdout,

        // ---------- resolution ----------
        out_mut_planar          : Planar,
        out_c_key               : u8,           // Nav Key (getch)

        // ---------- Indices ----------
        in_out_mut_pidx         : u8,           // Panel Index

        // ---------- Operations ----------
        in_out_mut_hmod         : u8,           // Getch Handler Mode
        in_out_mut_opmod        : u8,           // RW Operation Mode
        in_out_mut_sid          : u8,           // Session Panel ID


        // ========== Configuration ==========

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

impl TUIEnv {
        /// ## Default
        /// Implements the default state of the TUI context. Calls new()
        pub fn Default() -> Result<Self, io::Error> {
                // ---------- blank ctx ----------
                TUIEnv::new()
        }

        pub fn new() -> Result<Self, io::Error> {// ---------- blank ctx ----------
                let mut term = io::stdout();
                execute!(term, terminal::EnterAlternateScreen)?;
                let (ui_x, ui_y) = terminal::size().unwrap();
                execute!(term, terminal::LeaveAlternateScreen)?;
                Ok(TUIEnv {
                        in_window: std::io::stdout(),
                        out_mut_planar: Planar {
                                position: (0, 0),
                                size: DynSize::Parent,
                        },
                        out_c_key: 0,
                        in_out_mut_pidx: 0,
                        in_out_mut_hmod: 0,
                        in_out_mut_opmod: 0,
                        in_out_mut_sid: 0,
                        wrap_lim: 0,
                        c0: 0,
                        c5: 0,
                        c1: 0,
                        c4: 0,
                        c2: 0,
                        c3: 0,
                })
        }
}

pub(crate) enum DynSize {
        /// Parent will return as a Usize(ParentX,ParentY) during
        /// the parsing phase when building the ComponentBlock
        /// representation. This serves as a directive to the parser
        /// to save the parent component's size and clone it to this
        /// Component.
        Parent,
        /// Block will return as a Usize(ParentX, LocalY) during the
        /// parsing phase when building ComponentBlock representation.
        /// This serves as a directive to the parser to save only the
        /// parent's X size and clone it to this component
        Block,
        /// Usize serves as the most primitive variant of DynSize and is, by
        /// definition, the only directly mutable / assignable size. All
        /// DynSize variants are eventually converted to Usize when building
        /// the ComponentBlock representation. However, explicit Usize assignments
        /// before parsing will **always** retain its values after CBR building,
        /// unlike the Parent or Block variants, which are regenerated and may or
        /// may not retain its values after parsing.
        Usize(u8, u8),
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
// fn graph() -> ComponentBlock<'static> {
//         return ComponentBlock {
//                 frame: Frame::New(1),
//                 child: vec![],
//         };
// }
//
// fn table() -> ComponentBlock<'static> {
//         return ComponentBlock {
//                 frame: Frame::New(1),
//                 child: vec![],
//         };
// }
//
// fn helper() -> ComponentBlock<'static> {
//         return ComponentBlock {
//                 frame: Frame::New(1),
//                 child: vec![],
//         };
// }
