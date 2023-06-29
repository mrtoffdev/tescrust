// ---------- Table ----------
enum TableData {
        // common
        str_int(Vec<(String, usize)>),
        int_str(Vec<(usize, String)>),
        str_str(Vec<(String, String)>),
        int_int(Vec<(usize, usize)>),

}

struct Table {
        // data
        model   : TableData,

        // tui
        scroll  : Scroll,
        rows    : i32,
        cols    : i32,
}

impl TCComponent for TableData {
        fn indent(size: DynSize) {
                todo!()
        }

        fn whitespace(size: DynSize) {
                todo!()
        }

        fn divider<S>(size: S) {
                todo!()
        }
}

impl Table {
        pub(crate) fn Build() -> ComponentBlock {
                ComponentBlock {
                        frame: Frame::New(1),
                        child: vec![],
                        scroll: (),
                        tui_size: DynSize::Parent,
                }
        }

        fn get(&self) {

        }
}
