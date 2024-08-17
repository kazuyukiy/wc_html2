use html5ever::driver::parse_document; // , serialize
use html5ever::tendril::TendrilSink;
use markup5ever_rcdom::RcDom;
use std::fs;
// use std::str::FromStr;
// use tracing::{instrument, Level}; // event, info, , span , debug
use tracing::info;
// info!("hooll");

// mod contents_json;
mod page_json;
pub mod page_utility;
// use crate::wc_handler2::page::page_utility::json_from_dom;

/// path: the path of the page.
/// source: None: not tried to read, Some(None): Tried to read with faile,
/// Some(Some(v)) : Tried to read and get the contents.
///
pub struct Page {
    // page_root: String,
    path: String,
    source: Option<Option<Vec<u8>>>,
    // json: Option<contents_json::ContentsJson>,
    dom: Option<Option<RcDom>>,
    // json: Option<Option<json::JsonValue>>,
    json: Option<Option<page_json::PageJson>>,
    // page_json: Option<page_json::PageJson>,
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
            // contents: None,
            dom: None,
            json: None,
            // page_json: None,
        }
    }
    ///
    /// Open the file.
    ///
    /// # Error
    ///
    /// This function will return an error if a file does not exists in `path`.
    ///
    fn _open(&mut self, path: &str) -> Page {
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

    fn source(&mut self) -> Option<&Vec<u8>> {
        if self.source.is_none() {
            let _ = self.read();
        }
        match self.source.as_ref().unwrap().as_ref() {
            Some(v) => return Some(v),
            None => return None,
        }
    }

    fn dom_parse(&mut self) -> Result<(), ()> {
        let source = match self.source() {
            Some(v) => v.to_owned(),
            None => return Err(()),
        };

        let source = match String::from_utf8(source) {
            Ok(v) => v,
            Err(_) => return Err(()),
        };
        let dom =
            parse_document(markup5ever_rcdom::RcDom::default(), Default::default()).one(source);

        self.dom.replace(Some(dom));

        // temp
        Ok(())
    }

    fn dom(&mut self) -> Option<&RcDom> {
        if self.dom.is_none() {
            if let Err(_) = self.dom_parse() {
                return None;
            }
        };
        Some(self.dom.as_ref().unwrap().as_ref().unwrap())
    }

    fn json_parse(&mut self) -> Result<(), ()> {
        let dom = match self.dom() {
            Some(v) => v,
            None => return Err(()),
        };

        match page_utility::json_from_dom(dom) {
            Some(v) => {
                let page_json = page_json::PageJson::from(v);
                self.json.replace(Some(page_json));
                return Ok(());
            }
            None => {
                self.json.replace(None);
                return Err(());
            }
        }
    }

    fn json(&mut self) -> Option<&page_json::PageJson> {
        if self.json.is_none() {
            if let Err(_) = self.json_parse() {
                return None;
            }
        }

        self.json.as_ref().unwrap().as_ref()
    }

    pub fn json_replace_save(&mut self, mut json_data2: json::JsonValue) -> Result<(), &str> {
        if !page_utility::json_rev_match(self, &json_data2) {
            info!("rev not match.");
            return Err("rev not match.");
        }

        let page_json = match self.json() {
            Some(v) => v,
            None => return Err("Failed to get page_json."),
        };

        let rev_uped = match page_json.rev_uped() {
            Some(v) => v,
            None => return Err("Failed to get rev_uped"),
        };
        json_data2["data"]["page"]["rev"] = rev_uped.into();

        let mut page2 = match page_utility::page_from_json(&self.path, json_data2) {
            Ok(v) => v,
            Err(_) => return Err("Failed to create a page from json posted."),
        };

        let _ = page2.file_save();

        match page2.file_save_rev() {
            Ok(_) => Ok(()),
            Err(_) => Err("Failed to save page with rev."),
        }
    }

    /// Save self.source data to the file.
    pub fn file_save(&mut self) -> Result<(), ()> {
        let path = self.path.clone();

        let source = match self.source() {
            Some(v) => v,
            None => return Err(()),
        };

        match fs::write(&path, source) {
            Ok(_) => {
                info!("save: {}", path);
                Ok(())
            }
            Err(_) => Err(()),
        }
    }

    fn path_rev(&mut self) -> Result<String, ()> {
        let page_json = match self.json() {
            Some(v) => v,
            None => return Err(()),
        };

        let rev = match page_json.rev() {
            Some(v) => v.to_string(),
            None => return Err(()),
        };

        // String + "." + &str
        Ok(self.path.clone() + "." + &rev)
    }

    pub fn file_save_rev(&mut self) -> Result<(), ()> {
        let path_rev = match self.path_rev() {
            Ok(v) => v,
            Err(()) => return Err(()),
        };

        let source = match self.source() {
            Some(v) => v,
            None => return Err(()),
        };

        match fs::write(&path_rev, source) {
            Ok(_) => {
                info!("save: {}", path_rev);
                Ok(())
            }
            Err(_) => Err(()),
        }
    }
}
