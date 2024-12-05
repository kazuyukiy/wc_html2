use super::super::page_upgrade_handle::Upres;
use html5ever::driver::parse_document; // , serialize
use html5ever::tendril::TendrilSink; // parse_document(...).one() needs this
use markup5ever_rcdom::RcDom;
use std::cell::RefCell;
use std::fs;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use tracing::{error, info}; //  error, event, info_span, instrument, span, Level debug,, warn
                            // pub mod page_backup_delete;
pub mod page_json;
pub mod page_utility;

/// path: the path of the page. ie: ./stor_root/page_path
/// source: None: not tried to read, Some(None): Tried to read with faile,
/// Some(Some(v)) : Tried to read and get the source.
///
pub struct Page {
    stor_root: String,
    page_path: String,
    path: PathBuf,
    source: Option<Option<Vec<u8>>>,
    dom: Option<Option<RcDom>>,
    json: Option<Option<page_json::PageJson>>,
}

impl Page {
    /// Returns `Page`.
    /// It is used for further creation of 'Page'
    /// page_path should start with "/" eg: "/Computing/computing.html".
    pub fn new(stor_root: &str, page_path: &str) -> Page {
        let path = String::from(stor_root) + page_path;
        let path = PathBuf::from(path);

        Page {
            stor_root: String::from(stor_root),
            page_path: String::from(page_path),
            path,
            source: None,
            dom: None,
            json: None,
        }
    }

    pub fn stor_root(&self) -> &str {
        self.stor_root.as_str()
    }

    pub fn page_path(&self) -> &str {
        self.page_path.as_str()
    }

    // pub fn page_path2(&self) -> Option<&str> {
    //     self.path.to_str()
    // }

    pub fn path(&self) -> &Path {
        self.path.as_path()
    }

    pub fn file_path(&self) -> String {
        page_utility::file_path(&self.stor_root, &self.page_path)
    }

    pub fn is_end_with_rev(&self) -> bool {
        let reg = regex::Regex::new(r#"html.[0-9]+$"#).unwrap();
        reg.is_match(&self.page_path)
    }

    pub fn read(&mut self) -> Result<&Vec<u8>, ()> {
        let file_path = &self.file_path();
        match fs::read(&file_path) {
            Ok(s) => {
                self.source_replace_some(s);
                return Ok(self.source.as_ref().unwrap().as_ref().unwrap());
            }
            // file not found
            Err(e) => {
                self.source.replace(None);
                self.source_origined_clear();
                error!("Faile to read: {}, {:?}", &file_path, e.kind());
                return Err(());
            }
        }
    }

    // Create dirs saving this file.
    pub fn dir_build(&self) -> Result<(), ()> {
        let recursive = true;
        page_utility::dir_build(self.path.as_path(), recursive)
        // return page_utility::dir_build(self.path.as_path(), recursive);

        // // file_path : abc/def/ghi.html (Contains a file name.)
        // let file_path = self.file_path();
        // let path = Path::new(&file_path);
        // // parent: abc/def (remain only directory path.)
        // let parent = path.parent().ok_or(())?;

        // // This count() counts depth of directory.
        // // Consider how avoid too match deep directorys making.

        // // Already exists.
        // if let Ok(true) = parent.try_exists() {
        //     return Ok(());
        // }

        // let parent_path = parent.to_str().ok_or(())?;
        // match std::fs::DirBuilder::new()
        //     .recursive(true)
        //     .create(parent_path)
        // {
        //     Ok(_) => {
        //         info!("dir created: {}", parent_path);
        //         Ok(())
        //     }
        //     Err(_) => {
        //         error!("Failed to create dir: {}", parent_path);
        //         Err(())
        //     }
        // }
    }

    pub fn source(&mut self) -> Option<&Vec<u8>> {
        if self.source.is_none() {
            let _ = self.read();
        }

        self.source.as_ref().unwrap().as_ref()
    }

    fn source_replace_some(&mut self, source: Vec<u8>) {
        self.source.replace(Some(source));
        self.source_origined_clear();
    }

    fn source_origined_clear(&mut self) {
        self.dom = None;
        self.json = None;
    }

    fn dom_parse(&mut self) -> Result<(), ()> {
        let source = match self.source() {
            Some(v) => v.to_owned(),
            None => {
                return Err(());
            }
        };

        let source = String::from_utf8(source).or(Err(()))?;
        let dom =
            parse_document(markup5ever_rcdom::RcDom::default(), Default::default()).one(source);

        self.dom.replace(Some(dom));

        Ok(())
    }

    pub fn dom(&mut self) -> Option<&RcDom> {
        if self.dom.is_none() {
            let _ = self.dom_parse().ok()?;
        };
        Some(self.dom.as_ref().unwrap().as_ref().unwrap())
    }

    fn json_parse(&mut self) -> Result<(), ()> {
        let dom = self.dom().ok_or(())?;
        match page_utility::json_from_dom(&dom.document) {
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

    fn json_mut(&mut self) -> Option<&mut page_json::PageJson> {
        if self.json.is_none() {
            if let Err(_) = self.json_parse() {
                return None;
            }
        }
        self.json.as_mut().unwrap().as_mut()
    }

    ///
    pub fn json_value(&mut self) -> Option<&json::JsonValue> {
        self.json().and_then(|page_json| page_json.value())
    }

    /// Updata the page with json_data2
    /// rev no (json_data2["data"]["page"]["rev"]) should match with the current no.
    /// Return Ok(rev_uped), new rev number
    pub fn json_replace_save(&mut self, mut json_data2: json::JsonValue) -> Result<usize, String> {
        page_utility::json_rev_match(self, &json_data2)?;

        // Updata rev no.
        let page_json = self.json().ok_or("Failed to get page_json.")?;
        let rev_uped = page_json.rev_uped().ok_or("Failed to get rev_uped")?;
        json_data2["data"]["page"]["rev"] = rev_uped.into();

        let mut page2 = page_utility::page_from_json(&self.stor_root, &self.page_path, &json_data2);

        page2
            .file_save_and_rev()
            .and(Ok(rev_uped))
            .or(Err(format!("Failed to save json of  {}", &self.page_path)))
    }

    /// Save self.source data to the file.
    pub fn file_save(&mut self) -> Result<String, String> {
        // DBG
        // warn!("pub fn file_save returning Err in DBG");
        // return Err("".into());

        let file_path = &self.file_path();
        let source = match self.source() {
            Some(v) => v,
            None => return Err(format!("Failed to get source: {}", &self.file_path())),
        };
        page_utility::fs_write(file_path, source)
    }

    pub fn rev(&mut self) -> Result<usize, ()> {
        let page_json = self.json().ok_or(())?;
        // let rev = page_json.rev().ok_or(())?;
        let rev = match page_json.rev() {
            Some(v) => v,
            None => {
                error!("Failed to get rev on {}", self.file_path());
                return Err(());
            }
        };
        Ok(rev)
    }

    fn path_rev(&mut self) -> Result<String, ()> {
        let page_json = self.json().ok_or(())?;
        let rev = page_json.rev().ok_or(())?;
        // Ok(self.path_rev_form(rev))

        let path_rev = self.path_rev_form(rev);
        match path_rev.to_str() {
            Some(v) => Ok(v.to_string()),
            None => Err(()),
        }
        // or(Some("")).unwrap().to_string();

        //
        // Ok(self.path_rev_form(rev))
    }

    /// This function takes rev in arguments, not consering with self.json.rev().
    /// This is only for a path format with rev numaber.
    /// ./stor_root/page_path.html + "." + rev_no
    // pub fn path_rev_form(&self, rev: usize) -> String {
    pub fn path_rev_form(&self, rev: usize) -> PathBuf {
        // self.file_path() + "." + rev.to_string().as_str()
        let path = self.file_path() + "." + rev.to_string().as_str();
        PathBuf::from(path)
    }

    /// Save self.source value to self.path_rev().
    /// Return self.path_revp() on sucsess as Ok
    /// Return Err in fail.
    pub fn file_save_rev(&mut self) -> Result<String, String> {
        let path_rev = match self.path_rev() {
            Ok(v) => v,
            Err(()) => return Err(format!("Failed to get path_ref: {}", &self.file_path())),
        };

        let source = match self.source() {
            Some(v) => v,
            None => return Err(format!("Failed to get source: {}", &self.file_path())),
        };

        page_utility::fs_write(&path_rev, source)
    }

    pub fn file_backup_delete(&mut self) {
        page_utility::page_backup_delete::backup_delete(self);
    }

    /// Save the file and its backup file wit rev suffix.
    pub fn file_save_and_rev(&mut self) -> Result<(), ()> {
        let mut saved = true;

        if let Err(emsg) = self.file_save() {
            error!("{}", emsg.as_str());
            saved = false;
        }

        // file_save_rev is for backup.
        match self.file_save_rev() {
            Ok(v) => info!("Saved: {}", v),
            Err(e) => error!("{}", e),
            // comment out as intended because file_save_rev is for backup
            // Dicide result on self.file_save()
            // saved = false;
        }

        if saved {
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn json_subsections_data_exists(&mut self) -> bool {
        self.json_mut()
            .is_some_and(|page_json| page_json.subsections_data_exists())
    }

    /// Upgrade the page of url, not self.
    pub fn upgrade(&mut self, recursive: bool, upres: Option<Rc<RefCell<Upres>>>) {
        let upres2 = upres.as_ref().and_then(|v| Some(Rc::clone(v)));
        page_utility::page_upgrade(self, upres2);

        if recursive {
            page_utility::page_upgrade_children(self, recursive, upres);
        }
    }

    /// Move this page to dest_url as a child of parent_url.
    /// parent_url is an optional. If it is None, this page is a top page.
    pub fn page_move(
        &mut self,
        page_url: url::Url,
        dest_url: url::Url,
        parent_url: Option<url::Url>,
    ) -> Result<(), String> {
        // Case page alredy moved, return Err.
        if self
            .json()
            .and_then(|page_json| page_json.moved_to())
            .is_some()
        {
            return Err(format!("This page already moved to: {}", self.page_path));
        }

        page_utility::page_move(
            self.stor_root.as_str(),
            &page_url,
            dest_url.clone(),
            parent_url.as_ref(),
        )
    }
}
