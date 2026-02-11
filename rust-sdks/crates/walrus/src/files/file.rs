#[derive(Debug, Clone)]
pub struct WalrusFile {
    pub path: String,
    pub data: Vec<u8>,
    pub mime_type: Option<String>,
}

impl WalrusFile {
    pub fn new(path: String, data: Vec<u8>) -> Self {
        Self {
            path,
            data,
            mime_type: None,
        }
    }

    pub fn with_mime_type(mut self, mime_type: impl Into<String>) -> Self {
        self.mime_type = Some(mime_type.into());
        self
    }
}
