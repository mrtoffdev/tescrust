#![allow(non_snake_case)]
#![allow(unused)]

// ---------- <crate layout> -----------
mod tescrust {
        pub(crate) mod algo {
                pub mod order;
                pub mod search;
        }
        pub(crate) mod core {
                pub mod crud;
                pub mod data;
                pub mod guard;
        }
        pub(crate) mod tui {
                pub mod nav;
                pub mod view;
        }

        pub(crate) mod util {
                pub mod crust;
        }
}

use std::path::Path;
use crate::tescrust::{
        core::{
                crud::TCInput,
                data::Crust
        }, // prod version
        tui::{nav::*, view::TuiCtx},
};

// ---------- <dev utils> -----------
use crate::tescrust::util::crust as dbg_crust;

// ---------- <dev config> ----------
const PATH: &str = "test.txt";

fn main() {
        // ---------- <binary entry point> ----------
        let path: TCInput       = TCInput::Path(Path::new(PATH));

        // ------------- globals -------------

        //Todo: Replace w/ prod version
        let crust: Crust        = dbg_crust::new_tc(path);
        let tui_ctx: TuiCtx     = TuiCtx::new()
                .expect("tui_ctx gen err...");

        // ---------- init handlers ----------
        iostream_handler(crust, tui_ctx)
                .expect("iostream_handler err...");

        // ---------- on exit ----------


}
