// ---------- Data Models ----------
// Aggregated data models for generic TUI components

use crate::tescrust::tui::view::{DynSize, TCComponent};

/// ## GraphData
/// A struct that represents graph-able data.
enum GraphData {
        String(String),
        Int(usize),

}

/// ## Planar
/// A struct that contains various data related to
/// sizes and positions
pub struct Planar {
        /// Starting position of the component in the TUI representing
        /// Row and Column
        pub(crate) position        : (i32, i32),
        /// Size of the component in the tui, representing DynSize
        pub(crate) size            : DynSize,
}


// ---------- TUI Models ----------
// The core building block of every TUI component present in a window


/// ## ComponentBlock
/// The most primitive render-able TUI element
pub struct ComponentBlock<'F> {
        frame           : Frame<'F>,
        child           : Vec<String>,

        scroll          : ScrollHandler,
        tui_size        : DynSize,
}

pub enum Frame<'p> {
        Merged(&'p Frame<'p>),
        New(u8),
}

// ---------- TCComponent ----------
impl TCComponent for ComponentBlock<'_> {
        fn Build(&self) -> ComponentBlock<'static> {
                todo!()
        }
}

