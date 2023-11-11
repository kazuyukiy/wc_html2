use html5ever::parse_document; // , serialize
use html5ever::tendril::TendrilSink;
use markup5ever_rcdom::RcDom;
use regex::Regex;
use std::fs;

mod page_utility;

// url::Url have a data for path, so path and url are duplicated in some part
// but
pub struct Page {
    page_root: String,
    path: String,
    url: Option<url::Url>,
    source: Option<Vec<u8>>,
    dom: Option<RcDom>,
    json: Option<json::JsonValue>,
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
            json: None,
        }
    }

    ///
    /// Read the file and return `Page`.
    ///
    /// # Error
    ///
    /// This function will return an error if a file does not already exists in  `path`.
    ///
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

    pub fn page_root(&self) -> &str {
        self.page_root.as_ref()
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

    fn dom(&self) -> Option<&RcDom> {
        match &self.dom {
            Some(v) => Some(&v),
            None => None,
        }
    }

    pub fn json_set(&mut self) {
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

        self.json.replace(json);
    }

    // to use fn json(), do fn json_set() previously
    // self.json.as_ref().unwrap() may couse a panic
    // if json_set() was not called
    // the panic may let you know json_set() was not done
    fn json(&self) -> Option<&json::JsonValue> {
        match self.json.as_ref() {
            Some(v) => Some(v),
            None => {
                eprintln!("Failed to get json");
                None
            }
        }
    }

    // xxx.html.01
    // It mean backup file
    pub fn name_end_num(&self) -> bool {
        // xxx.html.01
        let re = Regex::new(r"\d+$").unwrap();
        re.is_match(&self.path)
    } // end of fn name_end_num

    // current rev
    pub fn rev(&self) -> Option<u32> {
        match self.json()?["data"]["page"]["rev"].as_u32() {
            Some(v) => Some(v),
            None => {
                eprintln!("Failed to get rev");
                None
            }
        }
    }

    // rev counted up from current rev
    fn rev_uped(&self) -> Option<u32> {
        let rev = self.rev()?;
        Some(rev + 1)
    }

    fn path_rev(&self) -> Option<String> {
        let rev = self.rev()?;
        // file_path + "." + rev
        // Some(format!("{}.{}", &self.path, &rev))
        //String::from(self.path) + rev
        Some(self.path.clone() + &rev.to_string())
    }

    /// Save self.source to the file.
    fn file_save(&self) -> Result<(), ()> {
        // make a String to avoid error
        // &mut self will be used in self.source()
        // to avoid borrowing &mut self and &self in a time

        // let path = &self.path.to_string();

        let source = match self.source() {
            Some(s) => s,
            None => return Err(()),
        };

        // match fs::write(&path, source) {
        match fs::write(&self.file_path(&self.path), source) {
            Ok(_) => {
                // println!("write: {}", &path);
                println!("write: {}", &self.path);
                Ok(())
            }
            Err(_) => {
                // eprintln!("Failed to save page: {}", &path);
                eprintln!("Failed to save page: {}", &self.path);
                Err(())
            }
        }
    }

    /// Save contents to a file with rev on its file name.
    pub fn file_save_rev(&mut self) -> Result<(), ()> {
        let path_rev = match self.path_rev() {
            Some(v) => v,
            None => {
                eprintln!("Failed to get path_rev");
                return Err(());
            }
        };

        // if path_rev already exits, no need to save it again
        // match fs::File::open(&path_rev) {
        match fs::File::open(&self.file_path(&path_rev)) {
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

        match fs::write(&self.file_path(&path_rev), &source) {
            Ok(_) => {
                println!("write: {}", &path_rev);
                Ok(())
            }
            Err(_) => {
                eprintln!("Failed to save page: {}", &path_rev);
                Err(())
            }
        }
    }

    // check if rev is match to rev in json posted
    // if json posted was updated from the current page,
    // the both of rev must match.
    fn json_post_rev_match(&self, json_post: &json::JsonValue) -> Result<(), ()> {
        // let check_sw = true;
        let check_sw = false;
        if check_sw == false {
            return Ok(());
        }

        let rev = match self.rev() {
            Some(v) => v,
            None => return Err(()),
        };

        let rev_post = match &json_post["data"]["page"]["rev"].as_u32() {
            Some(r) => *r,
            None => {
                return Err(());
            }
        };

        if rev != rev_post {
            return Err(());
        }

        Ok(())
    }

    pub fn json_post_save(&mut self, mut json_post: json::JsonValue) -> Result<(), ()> {
        // does not match rev number
        if let Err(_) = self.json_post_rev_match(&json_post) {
            return Err(());
        }

        // set new rev counted up from current rev
        let rev_uped = match self.rev_uped() {
            Some(v) => v,
            None => return Err(()),
        };
        json_post["data"]["page"]["rev"] = rev_uped.into();

        // get a new page from json_post
        let mut page_post =
            match page_utility::page_from_json(&self.page_root, &self.path, json_post) {
                Ok(v) => v,
                Err(_) => return Err(()),
            };

        // to set page.dom, page.json from contents
        // It is needed to handle rev and rev to save files
        page_post.json_set();

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
        child_page.json_set();
        if let Err(e) = child_page.file_save() {
            eprintln!("file_save err: {:?}", e);
        }
        if let Err(e) = child_page.file_save_rev() {
            eprintln!("file_save_rev err: {:?}", e);
        }
        Ok(())
    }
}
