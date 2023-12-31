#![allow(non_snake_case)]

use crate::tescrust::core::crud::{new_tc, TCInput};
use std::path::{Path, PathBuf};
use uuid::Uuid;

// primitive
pub type TCCell         = (Uuid, String, usize);
pub type TCCache        = (String, Vec<TCCell>);
pub type TCAction       = (Uuid, (TCCell, TCCell));

// contiguous
pub type TCSheet        = Vec<TCCell>;
pub type TCHistory      = Vec<TCAction>;

pub struct Crust {
        pub author      : String,
        pub id          : Uuid,
        pub auth_key    : String,
        pub path        : PathBuf,
        pub history     : TCHistory,

        pub m_cache_raw : String,
        pub m_cache_db  : TCSheet,
}

pub trait CrustIO {
        fn new(path: TCInput) -> Self;

        // ---------- auth ----------
        fn rw_auth_key(&mut self);
        fn rw_lock(&mut self);

        // ---------- mem ----------
        fn flush(&mut self);

        // ---------- actions ----------
        fn revert(&mut self, depth: usize);

        // ---------- core ----------
        fn serialize(&mut self);
}

impl CrustIO for Crust {
        fn new(path: TCInput) -> Self {
                todo!();
        }

        // ---------- auth ----------
        fn rw_auth_key(&mut self) {
                todo!()
        }
        fn rw_lock(&mut self) {
                // ...
                self.flush();
                todo!();
        }

        // ---------- mem ----------
        fn flush(&mut self) {
                self.m_cache_db = Vec::new();
                todo!()
        }

        // ---------- actions ----------
        fn revert(&mut self, depth: usize) {
                todo!();
        }

        // ---------- core ----------
        fn serialize(&mut self) {
                todo!();
        }

}
