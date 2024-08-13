use std::fs;

mod contents_json;

/// path: the path of the page.
/// source: None: not tried to read, Some(None): Tried to read with faile,
/// Some(Some(v)) : Tried to read and get the contents.
///
pub struct Page {
    // page_root: String,
    path: String,
    source: Option<Option<Vec<u8>>>,
    json: Option<contents_json::ContentsJson>,
}

impl Page {
    /// Returns `Page`.
    /// It is used for further creation of 'Page'
    pub fn new(path: &str) -> Page {
        let path = String::from(path);

        Page {
            path,
            // url: None,
            source: None,
            // dom: None,
            json: None,
            // contents: None,
        }
    }
    ///
    /// Open the file.
    ///
    /// # Error
    ///
    /// This function will return an error if a file does not exists in `path`.
    ///
    fn open_(&mut self, path: &str) -> Page {
        let mut page = Page::new(path);

        let _ = page.read();

        page
    }

    pub fn read(&mut self) -> Result<&Vec<u8>, ()> {
        match fs::read(&self.path) {
            Ok(s) => {
                self.source.replace(Some(s));
                // self.source.as_ref().unwrap().as_ref().unwrap();
                return Ok(self.source.as_ref().unwrap().as_ref().unwrap());
            }
            // file not found
            Err(e) => {
                self.source.replace(None);
                eprintln!("Faile to read: {}, {:?}", self.path, e.kind());
                return Err(());
            }
        }
    }
}
