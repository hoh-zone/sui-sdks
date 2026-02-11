use std::fs;

use crate::files::file::WalrusFile;

pub struct LocalFileReader;

impl LocalFileReader {
    pub fn read(path: &str) -> std::io::Result<WalrusFile> {
        let data = fs::read(path)?;
        Ok(WalrusFile::new(path.to_string(), data))
    }
}
