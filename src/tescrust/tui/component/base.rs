// ---------- Data Models ----------
enum GraphData {
        String(String),
        Int(usize),

}

// ---------- TUI Models ----------

/// # Core Elements
/// The core building block of every TUI component present
/// in a window
struct ComponentBlock<'F> {
        frame           : Frame<'F>,
        child           : Vec<String>,

        scroll          : ScrollHandler,
        tui_size        : DynSize,
}

enum Frame<'p> {
        Merged(&'p Frame<'p>),
        New(u8),
}

// ---------- TCComponent ----------
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
