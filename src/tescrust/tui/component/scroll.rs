// ---------- Scroll ----------
struct Scroll {
        scr_buff        : i32,
        scr_idx         : i32,
}

impl Scroll {
        pub(crate) fn Build() -> ComponentBlock {
                ComponentBlock {
                        frame: Frame::New(1),
                        child: vec![],
                        scroll: (),
                        tui_size: DynSize::Parent,
                }
        }

        /// Returns the size of the scrollview buffer
        fn get_size(&self, callback: Option<fn()>) -> i32 {
                self.scr_buff
        }

        /// Sets the selected index within a scrollview
        fn set_idx(&mut self, index: i32, callback: Option<fn()>) {
                self.scr_idx = index;
                match callback {
                        Some(callback) => callback(),
                        None => ()
                }
        }
}
