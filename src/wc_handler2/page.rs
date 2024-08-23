use html5ever::driver::parse_document; // , serialize
use html5ever::tendril::TendrilSink;
use markup5ever_rcdom::RcDom;
use std::fs;
// use std::str::FromStr;
// use tracing::{instrument, Level}; // event, info, , span , debug
use tracing::info;
// info!("hooll");

mod page_json;
pub mod page_utility;
// use crate::wc_handler2::page::page_utility::json_from_dom;

/// path: the path of the page.
/// source: None: not tried to read, Some(None): Tried to read with faile,
/// Some(Some(v)) : Tried to read and get the contents.
///
pub struct Page {
    stor_root: String,
    page_path: String,
    source: Option<Option<Vec<u8>>>,
    // host: Option<String>,
    dom: Option<Option<RcDom>>,
    // json: Option<Option<json::JsonValue>>,
    json: Option<Option<page_json::PageJson>>,
}

impl Page {
    /// Returns `Page`.
    /// It is used for further creation of 'Page'
    pub fn new(stor_root: &str, page_path: &str) -> Page {
        // let path = String::from(path);

        Page {
            stor_root: String::from(stor_root),
            page_path: String::from(page_path),
            source: None,
            // host: None,
            // dom: None,
            // contents: None,
            dom: None,
            json: None,
        }
    }

    // ///
    // /// Open the file.
    // ///
    // /// # Error
    // ///
    // /// This function will return an error if a file does not exists in `path`.
    // ///
    // fn _open(&mut self, path: &str) -> Page {
    //     let mut page = Page::new(path);

    //     let _ = page.read();

    //     page
    // }

    fn stor_root(&self) -> &str {
        self.stor_root.as_str()
    }

    fn file_path(&self) -> String {
        page_utility::file_path(&self.stor_root, &self.page_path)
        // String + "." + &str
        // self.stor_root.to_string() + self.page_path.as_str()
    }

    pub fn read(&mut self) -> Result<&Vec<u8>, ()> {
        let file_path = &self.file_path();
        // let file_path = page_utility::file_path(&self.stor_root, &self.page_path);
        match fs::read(&file_path) {
            Ok(s) => {
                self.source.replace(Some(s));
                return Ok(self.source.as_ref().unwrap().as_ref().unwrap());
            }
            // file not found
            Err(e) => {
                self.source.replace(None);
                eprintln!("Faile to read: {}, {:?}", &file_path, e.kind());
                return Err(());
            }
        }
    }

    // Create dirs saving this file.
    pub fn dir_build(&self) -> Result<(), ()> {
        // file_path : abc/def/ghi.html (Contains a file name.)
        let file_path = self.file_path();
        let path = std::path::Path::new(&file_path);
        // parent: abc/def (remain only directory path.)
        let parent = path.parent().ok_or(())?;

        // Already exists.
        if let Ok(true) = parent.try_exists() {
            return Ok(());
        }

        let parent_path = parent.to_str().ok_or(())?;
        match std::fs::DirBuilder::new()
            .recursive(true)
            .create(parent_path)
        {
            Ok(_) => {
                info!("dir created: {}", parent_path);
                Ok(())
            }
            Err(_) => {
                eprintln!("Failed to create dir: {}", parent_path);
                Err(())
            }
        }
    }

    pub fn source(&mut self) -> Option<&Vec<u8>> {
        if self.source.is_none() {
            let _ = self.read();
        }

        self.source.as_ref().unwrap().as_ref()
    }

    fn dom_parse(&mut self) -> Result<(), ()> {
        let source = match self.source() {
            Some(v) => v.to_owned(),
            None => return Err(()),
        };

        let source = String::from_utf8(source).or(Err(()))?;
        let dom =
            parse_document(markup5ever_rcdom::RcDom::default(), Default::default()).one(source);

        self.dom.replace(Some(dom));

        // temp
        Ok(())
    }

    fn dom(&mut self) -> Option<&RcDom> {
        if self.dom.is_none() {
            let _ = self.dom_parse().ok()?;
        };
        Some(self.dom.as_ref().unwrap().as_ref().unwrap())
    }

    fn json_parse(&mut self) -> Result<(), ()> {
        let dom = self.dom().ok_or(())?;
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

        let page_json = self.json().ok_or("Failed to get page_json.")?;
        let rev_uped = page_json.rev_uped().ok_or("Failed to get rev_uped")?;
        json_data2["data"]["page"]["rev"] = rev_uped.into();

        let mut page2 = page_utility::page_from_json(&self.stor_root, &self.page_path, json_data2)
            .or(Err("Failed to create a page from json posted."))?;

        page2.file_save_and_rev().or(Err("failed to save"))
    }

    /// Save self.source data to the file.
    pub fn file_save(&mut self) -> Result<(), ()> {
        let file_path = &self.file_path();
        let source = self.source().ok_or(())?;

        // dbg
        info!("Got source.");

        page_utility::fs_write(file_path, source)
    }

    fn path_rev(&mut self) -> Result<String, ()> {
        let page_json = self.json().ok_or(())?;
        let rev = page_json
            .rev()
            .and_then(|v| Some(v.to_string()))
            .ok_or(())?;
        // let rev = match page_json.rev() {
        //     Some(v) => v.to_string(),
        //     None => return Err(()),
        // };

        Ok(self.file_path() + "." + &rev)
    }

    pub fn file_save_rev(&mut self) -> Result<(), ()> {
        let path_rev = self.path_rev()?;
        let source = self.source().ok_or(())?;
        page_utility::fs_write(&path_rev, source)
    }

    pub fn file_save_and_rev(&mut self) -> Result<(), ()> {
        let mut saved = true;

        if self.file_save().is_err() {
            saved = false;
        }
        if self.file_save_rev().is_err() {
            saved = false;
        }

        if saved {
            Ok(())
        } else {
            Err(())
        }
    }
}
