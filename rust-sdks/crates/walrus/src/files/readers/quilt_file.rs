use crate::files::file::WalrusFile;

#[derive(Debug, Clone)]
pub struct QuiltFileReader {
    files: Vec<WalrusFile>,
}

impl QuiltFileReader {
    pub fn new(files: Vec<WalrusFile>) -> Self {
        Self { files }
    }

    pub fn files(&self) -> &[WalrusFile] {
        &self.files
    }
}
