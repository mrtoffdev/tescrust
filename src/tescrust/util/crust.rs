#![allow(non_snake_case)]
#![allow(unused)]

// ---------- <intern> -----------
use std::{
        fs::File,
        io::Read,
        path::{Path, PathBuf},
};
use uuid::Uuid;

// ---------- <pkg> -----------
use crate::tescrust::core::{
        crud::TCInput,
        data::{Crust, TCAction, TCSheet},
};

// ---------- <debug use case only> -----------
// ---------- <debug use case only> -----------
// ---------- <debug use case only> -----------

pub fn new_tc(path: TCInput) -> Crust {
        let TCInput::Path(x) = path else { panic!("wrr") };
        let mut rw_file = match File::open(x) {
                Ok(file) => file,
                Err(e) => {
                        eprintln!("Error during read: {e}");
                        std::process::exit(1);
                }
        };

        let mut contents: Vec<u8> = Vec::new();
        rw_file.read_to_end(&mut contents)
                .expect("cannot read contents");

        println!("Contents: {:?}", String::from_utf8(contents).unwrap());

        Crust {
                author: String::new(),
                id: Uuid::new_v4(),
                auth_key: String::new(),
                path: PathBuf::new(),
                history: Vec::new(),
                m_cache_raw: String::new(),
                m_cache_db: Vec::new(),
        }
}
