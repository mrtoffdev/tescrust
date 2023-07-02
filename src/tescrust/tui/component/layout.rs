use crate::tescrust::tui::{
        view::{
                TCComponent,
                DynSize
        },
        nav::KeyAction,
        component::base::ComponentBlock,
};


/// ---------- Horizontal Layout ----------
/// Layout where the flow of components move horizontally
///
/// Default behavior is L -> R
struct HorizontalLayout {
        position        : (i32, i32),
        size            : DynSize,
        children        : Vec<dyn TCComponent>,
}

impl TCComponent for HorizontalLayout {
        fn Build(&self) -> ComponentBlock<'static> {
                todo!()
        }
}


/// ---------- Vertical Layout ----------
/// Layout where the flow of components move vertically
///
/// Default behavior is T -> B <= Lim
struct VerticalLayout {
        position        : (i32, i32),
        size            : DynSize,
        children        : Vec<dyn TCComponent>,
}

impl TCComponent for VerticalLayout {
        fn Build(&self) -> ComponentBlock<'static> {
                todo!()
        }
}


/// ---------- Scroll Layout ----------
/// Layout where the flow of components move vertically
/// However, unlike Vertical Layouts, Scroll Layout contents are
/// allowed to overflow, creating a scrollable view to accommodate
/// excess data.
///
/// Default behavior is T -> B
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
struct ScrollLayout {
        position        : (i32, i32),
        size            : DynSize,
        children        : Vec<dyn TCComponent>,

        buff_size       : i32,
        buff_offset     : i32,
        cursor_pos      : i32,
}

impl ScrollLayout {
        fn move_buff(&mut self, direction: KeyAction){
                match direction {
                        KeyAction::Up   => {self.cursor_pos += 1},
                        KeyAction::Down => {self.cursor_pos -= 1},
                        _ => ()
                }
                let mut s = self.clone();
                let size = s.children.len();
        }
}

impl TCComponent for ScrollLayout {
        fn Build(&self) -> ComponentBlock<'static> {
                todo!()
        }
}