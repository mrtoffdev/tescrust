use crate::tescrust::tui::component::{
        base::Planar,
        bar_graph::BarGraph,
        table::{
                Table,
                TableData,
        },
        layout::{
                HorizontalLayout,
                VerticalLayout,
                Layout,
        },
};
use crate::tescrust::tui::view::DynSize;

/// ## Workspaces
/// Workspaces are defined via a function that returns the root Layout.
/// This function is then imported and called in the program's entry point
pub(crate) fn DefaultWorkspace() -> Layout {
        // Root layout
        Layout::VerticalLayout( VerticalLayout {
                planar: Planar {
                        position: (0, 0),
                        size: DynSize::Parent,
                },
                children: vec![

                        // Upper half of the workspace
                        HorizontalLayout {
                                planar: Planar {
                                        position: (0, 0),
                                        size: DynSize::Parent
                                },
                                children: vec![

                                        // Bar graph component
                                        BarGraph {
                                                x: vec![],
                                                y: vec![],
                                        },

                                        // Table component
                                        Table {
                                                model: TableData::str_int(
                                                        vec![]
                                                ),
                                                scroll: (),
                                                rows: 0,
                                                cols: 0,
                                        }
                                ],
                        },

                        // Lower half of the workspace
                        HorizontalLayout {
                                planar: Planar {
                                        position: (0, 0),
                                        size: DynSize::Parent,
                                },
                                children: vec![
                                        Table {
                                                model: TableData::str_int(
                                                        vec![]
                                                ),
                                                scroll: (),
                                                rows: 0,
                                                cols: 0,
                                        }
                                ],
                        }
                ]}
        )
}