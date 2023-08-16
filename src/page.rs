use std::io::Read;

pub struct Page {
    file: std::fs::File,
}

impl Page {
    pub fn from(page_path: &str) -> Result<Page, std::io::Error> {
        let file = match std::fs::File::open(page_path) {
            Ok(f) => f,
            Err(e) => return Err(e),
        };

        Ok(Page { file: file })
    }

    pub fn body(&mut self) -> Result<Vec<u8>, std::io::Error> {
        let mut buf = Vec::new();
        self.file.read_to_end(&mut buf).and(Ok(buf))
    }
}
