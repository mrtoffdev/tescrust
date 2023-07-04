#![allow(non_snake_case)]

use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use crate::tescrust::core::data::{Crust, CrustIO};

pub enum TCInput<'A> {
        Path(&'A Path),
        File(File),
}

pub fn new_tc<T: CrustIO>(path: TCInput) -> T {
        // ---------- parsing ----------
        let mut file_path: &Path;
        let mut crust_file: File;

        match path {
                TCInput::File(file) => {
                        crust_file = file;
                }
                TCInput::Path(path) => {
                        file_path = Path::new(path.as_os_str());
                        crust_file = File::open(file_path).unwrap();
                }
        }

        let mut file_raw: Vec<u8> = vec![];
        let mut m_cache_raw: String = String::new();

        match crust_file.read_to_end(&mut file_raw) {
                Ok(_) => m_cache_raw = String::from_utf8(file_raw).unwrap(),
                Err(e) => {
                        eprintln!("{e}")
                }
        }

        // ---------- auth ----------
        todo!();
}

pub fn test() {}
