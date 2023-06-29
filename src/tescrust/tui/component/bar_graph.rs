// ---------- Visuals | Graphs ----------
struct BarGraph {
        x: Vec<GraphData>,
        y: Vec<GraphData>,
}

impl TCComponent for BarGraph {
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

impl BarGraph {
        pub(crate) fn Build(&self) -> ComponentBlock {
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
