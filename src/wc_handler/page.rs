use html5ever::parse_document; // , serialize
use html5ever::tendril::TendrilSink;
use markup5ever_rcdom::RcDom;
use regex::Regex;
use std::fs;

mod page_utility;

pub struct Page {
    path: String,
    source: Option<Vec<u8>>,
    dom: Option<RcDom>,
    json: Option<json::JsonValue>,
}

impl Page {
    pub fn from_path(path: &str) -> Result<Page, ()> {
        let path = String::from(path);

        let source = match fs::read(&path) {
            Ok(s) => Some(s),
            Err(e) => {
                eprintln!("{:?}", e.kind());
                None
            }
        };

        let page = Page {
            path,
            source,
            dom: None,
            json: None,
        };

        Ok(page)
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

    fn json(&self) -> Option<&json::JsonValue> {
        self.json.as_ref()
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
        self.json()?["data"]["page"]["rev"].as_u32()
    }

    // rev counted up from current rev
    fn rev_uped(&self) -> Option<u32> {
        let rev = self.rev()?;
        Some(rev + 1)
    }

    fn path_rev(&self) -> Option<String> {
        let rev = self.rev()?;
        // file_path + "." + rev
        Some(format!("{}.{}", &self.path, &rev))
    }

    fn page_save(&mut self) -> Result<(), ()> {
        // make a String to avoid error
        // &mut self will be used in self.source()
        // to avoid borrowing &mut self and &self in a time
        let path = &self.path.to_string();

        let source = match self.source() {
            Some(s) => s,
            None => return Err(()),
        };

        match fs::write(&path, source) {
            Ok(_) => {
                println!("write: {}", &path);
                Ok(())
            }
            Err(_) => {
                eprintln!("Failed to save page: {}", &path);
                Err(())
            }
        }
    }

    // save contents without changes to file name with current rev
    pub fn page_save_rev(&mut self) -> Result<(), ()> {
        let path_rev = match self.path_rev() {
            Some(v) => v,
            None => {
                return Err(());
            }
        };

        let source = match self.source() {
            Some(s) => s,
            None => return Err(()),
        };

        match fs::write(&path_rev, &source) {
            Ok(_) => {
                println!("write: {}", &path_rev);
                Ok(())
            }
            // Err(_) => Err(()),
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
        //
        // let rev_check = true;
        let rev_check = false;
        if rev_check {
            // does not match rev number
            if let Err(_) = self.json_post_rev_match(&json_post) {
                return Err(());
            }
        }

        // set new rev counted up from current rev
        let rev_uped = match self.rev_uped() {
            Some(v) => v,
            None => return Err(()),
        };
        json_post["data"]["page"]["rev"] = rev_uped.into();

        // get a new page from json_post
        let mut page_post = match page_utility::page_from_json(json_post, &self.path) {
            Ok(v) => v,
            Err(_) => return Err(()),
        };

        // to set page.dom, page.json
        page_post.json_set();

        if let Err(e) = page_post.page_save() {
            eprintln!("page_save err: {:?}", e);
        }
        if let Err(e) = page_post.page_save_rev() {
            eprintln!("page_save_rev err: {:?}", e);
        }

        Ok(())
    }
}
