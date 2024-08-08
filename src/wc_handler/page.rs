use html5ever::parse_document; // , serialize
use html5ever::tendril::TendrilSink;
use markup5ever_rcdom::RcDom;
use regex::Regex;
use std::fs;
use tracing::{event, info, instrument, span, Level};

mod contents;
mod page_utility;

// url::Url have a data for path, so path and url are duplicated in some part
// but
pub struct Page {
    page_root: String,
    path: String,
    url: Option<url::Url>,
    source: Option<Vec<u8>>,
    dom: Option<RcDom>,
    // json: Option<json::JsonValue>,
    contents: Option<contents::Contents>,
}

impl Page {
    /// Return `Page` with ounly page_root and path data.
    /// It is to be used further creation of 'Page'
    fn plain(page_root: &str, path: &str) -> Page {
        let page_root = String::from(page_root);
        let path = String::from(path);

        Page {
            page_root,
            path,
            url: None,
            source: None,
            dom: None,
            // json: None,
            contents: None,
        }
    }

    ///
    /// Read the file and return `Page`.
    ///
    /// # Error
    ///
    /// This function will return an error if a file does not already exists in  `path`.
    ///
    /// Fields url, dom, json are None. You need to set those if need.
    ///
    /// page_root: path where page files are strored in the server
    /// path: url path that a client is requesting
    pub fn open(page_root: &str, path: &str) -> Result<Page, ()> {
        let mut page = Page::plain(page_root, path);

        let file_path = &page.file_path(&page.path);
        match fs::read(&file_path) {
            Ok(s) => {
                page.source.replace(s);
                Ok(page)
            }
            // file not found
            Err(e) => {
                eprintln!("Faile to open: {}, {:?}", &file_path, e.kind());
                Err(())
            }
        }
    }

    pub fn new(page_root: &str, path: &str) -> Result<Page, ()> {
        let page = Page::plain(page_root, path);

        // Return Err if a file already exists
        match fs::File::open(&page.file_path(&page.path)) {
            Ok(_) => {
                // to prove if file exists, so not eprintln!() at here.
                // handle an Err where calling fn new()
                // eprintln!("page already exists: {}", &page.path);
                return Err(());
            }
            Err(_) => (),
        }
        Ok(page)
    }

    /// Returns a Page that inherites page_root and url of self.
    pub fn inherited(&self, path: &str) -> Page {
        let mut page = Page::open(&self.page_root, path);
        if page.is_err() {
            page = Page::new(&self.page_root, path);
        }
        let mut page = page.unwrap();

        if let Some(url) = self.url() {
            let url = url.join(path);
            if url.is_ok() {
                page.url_set(url.unwrap());
            }
        }
        page
    }

    pub fn page_root(&self) -> &str {
        self.page_root.as_ref()
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    fn file_path(&self, path: &str) -> String {
        self.page_root().to_string() + path
    }

    pub fn source(&self) -> Option<&Vec<u8>> {
        self.source.as_ref()
    }

    fn source_utf8(&self) -> Option<String> {
        let vec = self.source()?.to_owned();

        match String::from_utf8(vec) {
            Ok(v) => Some(v),
            Err(_) => {
                eprintln!("Failed to convert file source to UTF8");
                None
            }
        }
    }

    // fn dom_set set dom data form self.source with &mut self
    // once do this, you can call fn dom(), fn json() with &self (immutable)
    fn dom_set(&mut self) {
        let source_utf8 = match self.source_utf8() {
            Some(v) => v,
            None => {
                eprintln!("Failed to set dom");
                return;
            }
        };

        let dom = parse_document(markup5ever_rcdom::RcDom::default(), Default::default())
            .one(source_utf8);
        self.dom.replace(dom);
    }

    /// It returns Dom as an Option.
    /// To call self.dom(), do self.dom_set() in previously.
    /// To call seld.fom_set(), mutable reference is required.
    /// But to call self.dom(). immutable reference is enough.
    fn dom(&self) -> Option<&RcDom> {
        match &self.dom {
            Some(v) => Some(&v),
            None => None,
        }
    }

    pub fn contents_set(&mut self) {
        let dom = match self.dom() {
            Some(v) => v,
            None => {
                self.dom_set();
                match self.dom() {
                    Some(v) => v,
                    None => return,
                }
            }
        };
        let json = match page_utility::json_from_dom(dom) {
            Some(v) => v,
            None => return,
        };

        // self.json.replace(json);

        let contents = contents::Contents::from(json);
        self.contents.replace(contents);
    }

    fn contents_plain_set(&mut self) {
        self.contents.replace(contents::Contents::new());
        // dest_page.contents.replace(contents::Contents::new());
    }

    pub fn contents(&self) -> Option<&contents::Contents> {
        self.contents.as_ref()
    }

    pub fn contents_mut(&mut self) -> Option<&mut contents::Contents> {
        self.contents.as_mut()
    }

    fn contents_data(&self) -> Option<&json::JsonValue> {
        self.contents.as_ref()?.data()
    }

    fn contents_data_mut(&mut self) -> Option<&mut json::JsonValue> {
        self.contents.as_mut()?.data_mut()
    }

    fn rev(&self) -> Option<String> {
        Some(self.contents()?.rev()?.to_string())
    }

    // fn rev_uped(&self) -> Option<String> {
    //     Some(self.contents()?.rev_uped()?.to_string())
    // }
    fn rev_uped(&self) -> Option<u32> {
        Some(self.contents()?.rev_uped()?.try_into().ok()?)
    }

    // xxx.html.01
    // It mean backup file
    pub fn name_end_num(&self) -> bool {
        // xxx.html.01
        let re = Regex::new(r"\d+$").unwrap();
        re.is_match(&self.path)
    } // end of fn name_end_num

    /// Returns file_path + "." + rev
    fn path_rev(&self) -> Option<String> {
        // Returns None if no rev value.
        // let rev = self.contents()?.rev()?;
        // let file_path_rev = self.file_path(&self.path) + "." + &rev.to_string();
        // let file_path_rev = self.file_path(&self.path) + "." + &self.rev()?;
        // Some(file_path_rev)

        Some(self.file_path(&self.path) + "." + &self.rev()?)
    }

    /// Save self.source to the file.
    fn file_save(&self) -> Result<(), ()> {
        // println!("page.rs fn file_save");

        info!("fn file_save");

        let source = match self.source() {
            Some(s) => s,
            None => return Err(()),
        };

        let path = self.file_path(&self.path);
        match page_utility::file_write(&path, source) {
            Ok(_) => {
                println!("write: {}", &path);
                Ok(())
            }
            Err(_) => {
                eprintln!("page.rs Failed to save page: {}", &path);
                Err(())
            }
        }
    }

    /// Save contents to a file with rev on its file name.
    pub fn file_save_rev(&self) -> Result<(), ()> {
        let path_rev = match self.path_rev() {
            Some(v) => v,
            None => {
                eprintln!("Failed to get path_rev");
                return Err(());
            }
        };

        // if path_rev already exits, no need to save it again
        match fs::File::open(&path_rev) {
            Ok(_) => {
                // eprintln!("alreqady exists: {}", &self.file_path(&path_rev));
                return Err(());
            }
            Err(_) => (),
        }

        let source = match self.source() {
            Some(s) => s,
            None => return Err(()),
        };

        match page_utility::file_write(&path_rev, source) {
            Ok(_) => {
                println!("write: {}", &path_rev);
                Ok(())
            }
            Err(_) => {
                eprintln!("page.rs Failed to save page: {}", &path_rev);
                Err(())
            }
        }
    }

    // check if rev is match to rev in json posted
    // if json posted was updated from the current page,
    // the both of rev must match.
    fn json_post_rev_match(&self, json_post: &json::JsonValue) -> bool {
        // let check_sw = true;
        let check_sw = false;
        if check_sw == false {
            return true;
        }

        let rev = match self.contents().map(|contents| contents.rev()).flatten() {
            Some(v) => v,
            None => return false,
        };

        let rev_post = match &json_post["data"]["page"]["rev"].as_u32() {
            Some(r) => *r,
            None => {
                return false;
            }
        };

        rev == rev_post
    }

    pub fn json_post_save(&mut self, mut json_post: json::JsonValue) -> Result<(), ()> {
        // does not match rev number
        //if let Err(_) = self.json_post_rev_match(&json_post) {
        if self.json_post_rev_match(&json_post) == false {
            return Err(());
        }

        // let rev_uped = match self
        //     .contents()
        //     .map(|contents| contents.rev_uped())
        //     .flatten()
        // {
        //     Some(v) => v,
        //     None => return Err(()),
        // };

        // set new rev counted up from current rev
        let rev_uped = match self.rev_uped() {
            Some(v) => v,
            None => return Err(()),
        };

        // let i: u32 = rev_uped;

        // let rev_uped: json::number::Number = 30.into();
        // let rev_uped: json::number::Number = rev_uped.into();
        // let rev_uped = json::JsonValue::from(rev_uped);

        json_post["data"]["page"]["rev"] = rev_uped.into();
        // json_post["data"]["page"]["rev"] = rev_uped;
        // json_post["data"]["page"]["rev"]: json::number::Number = rev_uped.into();

        // println!(
        //     "page.rs fn json_post_save [\"rev\"]: {:?}",
        //     json_post["data"]["page"]["rev"]
        // );

        // Create a new page from json_post
        let mut page_post =
            match page_utility::page_from_json(&self.page_root, &self.path, json_post) {
                Ok(v) => v,
                Err(_) => return Err(()),
            };

        // Set page.dom, page.json from contents to use rev
        // that is used to save file.
        // page_post.json_set();
        page_post.contents_set();

        //
        if let Err(e) = page_post.file_save() {
            eprintln!("file_save err: {:?}", e);
        }
        if let Err(e) = page_post.file_save_rev() {
            eprintln!("file_save_rev err: {:?}", e);
        }
        Ok(())
    }

    pub fn url_set(&mut self, url: url::Url) {
        self.url.replace(url);
    }

    pub fn url(&self) -> Option<&url::Url> {
        self.url.as_ref()
    }

    //    pub page_from_path

    /// Create a new `Page` on json_post(title, href).
    /// json_post: { "title":"title_name,"href":"href_data"}
    /// Return Err if a file already exists.
    pub fn page_sub_new_save(&self, json_post: json::JsonValue) -> Result<(), ()> {
        // title
        let title = match json_post["title"].as_str() {
            Some(s) => s,
            None => {
                eprintln!("title not found");
                return Err(());
            }
        };

        // href
        let href = match json_post["href"].as_str() {
            Some(s) => s,
            None => {
                eprintln!("href not found");
                return Err(());
            }
        };

        let mut child_page = page_utility::page_sub_new(&self, title, href)?;
        // to set page.dom, page.json from contents
        // It is needed to handle rev and rev to save files
        // child_page.json_set();
        child_page.contents_set();
        if let Err(e) = child_page.file_save() {
            eprintln!("file_save err: {:?}", e);
        }
        if let Err(e) = child_page.file_save_rev() {
            eprintln!("file_save_rev err: {:?}", e);
        }
        Ok(())
    }

    /// Move this page to dest_url as a child of parent_url.
    /// Some page might have not parent page.
    // pub fn page_move(&self, parent_url: &url::Url, dest_url: &url::Url) {}
    pub fn page_move(&self, dest_url: &str, parent_url: &str) -> Result<(), ()> {
        // dest_url (destination url) is necessary.
        // But parent_url is not necessary because some page might have parent.
        if dest_url.len() == 0 {
            return Err(());
        }

        // let post_url = match self.url().as_ref() {
        let post_url = match self.url() {
            Some(v) => v,
            None => return Err(()),
        };

        let dest_url = match post_url.join(dest_url) {
            Ok(v) => v,
            Err(_) => return Err(()),
        };

        // None: no parent page
        let parent_url = if parent_url.len() == 0 {
            None
        } else {
            match post_url.join(parent_url) {
                Ok(v) => Some(v),
                Err(_) => None,
            }
        };

        page_utility::page_move(&self, dest_url, parent_url)

        // temp
        // Ok(())
    }
}
