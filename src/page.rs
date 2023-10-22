// mod dom_utility;
mod page_utility;
// use html5ever::tendril::TendrilSink;
// use html5ever::serialize::SerializeOpts;
// use html5ever::{parse_document, serialize};
// use markup5ever_rcdom::{Handle, Node, NodeData, RcDom, SerializableHandle};
use markup5ever_rcdom::RcDom;
// use std::cell::RefCell;
use std::fs;
// use std::io::Read;
// use std::rc::Rc;
// use std::io::Write;
// use std::fs::OpenOptions;
// use std::io::Write;
use std::io::{Error, ErrorKind};
// use std::sync::{Arc, Mutex};

pub struct Page {
    path: String,
    // contents: Option<String>,
    // file: std::fs::File,
    source: Option<Vec<u8>>,
    // dom: Option<RcDom>, // this causes error, seems
    // error[E0277]: the trait bound `hyper::common::exec::Exec: hyper::common::exec::ConnStreamExec<impl Future<Output = Result<Response<Body>, hyper::Error>>, Body>` is not satisfied
    json: Option<json::JsonValue>,
    rev: Option<u32>,
    rev_uped: Option<u32>,
}

impl Page {
    pub fn from_path(path: &str) -> Result<Page, std::io::Error> {
        let path = String::from(path);

        // let file = match std::fs::File::open(&path) {
        //     Ok(f) => f,
        //     Err(e) => return Err(e),
        // };

        let page = Page {
            path,
            // contents: None,
            // file,
            source: None,
            json: None,
            rev: None,
            rev_uped: None,
        };

        Ok(page)
    }

    // fn contents(&mut self) -> Result<&str, std::io::Error> {
    //     if self.contents.is_none() {
    //         match std::fs::read_to_string(&self.path) {
    //             Ok(s) => {
    //                 let _ = self.contents.replace(s);
    //             }
    //             Err(e) => return Err(e),
    //         };
    //     }
    //     Ok(self.contents.as_ref().unwrap())
    // }

    // pub fn read(&mut self) -> Result<Vec<u8>, std::io::Error> {
    // fn read(&mut self) -> Result<Vec<u8>, std::io::Error> {
    //     let mut file = match fs::File::open(&self.path) {
    //         Ok(f) => f,
    //         Err(e) => return Err(e),
    //     };

    //     let mut buffer = Vec::new();
    //     // let _ = self.file.read_to_end(&mut buffer)?;
    //     let _ = file.read_to_end(&mut buffer)?;
    //     Ok(buffer)
    // }

    pub fn source(&mut self) -> Result<&Vec<u8>, std::io::Error> {
        if self.source.is_none() {
            // match self.read() {
            match fs::read(&self.path) {
                Ok(s) => self.source.replace(s),
                Err(e) => return Err(e),
            };
        }
        Ok(&self.source.as_ref().unwrap())
    }

    fn source_utf8(&mut self) -> Result<String, std::io::Error> {
        match self.source() {
            Ok(s) => match String::from_utf8(s.to_vec()) {
                Ok(s) => return Ok(s),
                Err(_) => return Err(Error::new(ErrorKind::Other, "Can not convert to UTF8")),
            },
            Err(e) => return Err(e),
        }
    }

    // since RcDom can not be in struct Page with the err,
    // handle dom of page html individually from struct Page
    pub fn dom(&mut self) -> Result<RcDom, std::io::Error> {
        let source = match self.source_utf8() {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        // Ok(dom_utility::to_dom(&source))
        // Ok(page_utility::dom_utility::to_dom(&source))
        Ok(page_utility::to_dom(&source))
    }

    fn json(&mut self) -> Result<&mut json::JsonValue, std::io::Error> {
        if self.json.is_none() {
            let dom = match self.dom() {
                Ok(d) => d,
                Err(e) => return Err(e),
            };
            match page_utility::json_from_dom(&dom) {
                // Ok(j) => self.json.replace(j),
                Ok(j) => {
                    self.json.replace(j);
                }
                Err(e) => return Err(e),
            }
        }

        Ok(self.json.as_mut().unwrap())
    }

    fn rev(&mut self) -> Result<u32, std::io::Error> {
        if self.rev.is_none() {
            let page_json = match self.json() {
                Ok(j) => j,
                Err(e) => return Err(e),
            };

            match &page_json["data"]["page"]["rev"].as_u32() {
                Some(r) => {
                    self.rev.replace(*r);
                }
                None => return Err(Error::new(ErrorKind::Other, "Can not get page rev no")),
            }
        }

        Ok(*self.rev.as_ref().unwrap())
    }

    fn rev_uped(&mut self) -> Result<u32, std::io::Error> {
        if self.rev_uped.is_none() {
            let rev = match self.rev() {
                Ok(r) => r,
                Err(e) => return Err(e),
            };
            let rev_uped = match rev.checked_add(1) {
                Some(i) => i,
                None => return Err(Error::new(ErrorKind::Other, "rev integer over flow")),
            };
            self.rev_uped.replace(rev_uped);
        }

        Ok(*self.rev_uped.as_ref().unwrap())
    }

    fn path_rev(&mut self) -> Result<String, std::io::Error> {
        let rev = match self.rev() {
            Ok(r) => r,
            Err(e) => return Err(e),
        };

        // file_path + "." + rev
        Ok(format!("{}.{}", &self.path, &rev))
    }

    fn _path_rev_uped(&mut self) -> Result<String, std::io::Error> {
        let rev_uped = match self.rev_uped() {
            Ok(r) => r,
            Err(e) => return Err(e),
        };

        // file_path + "." + rev_uped
        Ok(format!("{}.{}", &self.path, &rev_uped))
    }

    fn page_save(&mut self) -> Result<(), std::io::Error> {
        // make a String to avoid error
        // &mut self will be used in self.source()
        // to avoid borrowing &mut self and &self in a time
        let path = &self.path.to_string();

        let source = match self.source() {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        match fs::write(&path, source) {
            Ok(_) => {
                println!("write: {}", &path);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    // save contents without change to file name with current rev
    fn page_save_rev(&mut self) -> Result<(), std::io::Error> {
        let path_rev = match self.path_rev() {
            Ok(r) => r,
            Err(e) => return Err(e),
        };

        let source = match self.source() {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        match fs::write(&path_rev, &source) {
            Ok(_) => {
                println!("write: {}", &path_rev);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    // Replace whole json given in the parameter
    // and save the file
    pub fn json_post_save(&mut self, mut json_post: json::JsonValue) -> Result<(), std::io::Error> {
        // save contents without change to file name with current rev
        // it should be saved in prvious update, and expected to get err
        // ignore if it is err
        let _ = self.page_save_rev();

        // let rev_check = true;
        let rev_check = false;
        if rev_check {
            // does not match rev number
            if let Err(e) = self.json_post_rev_match(&json_post) {
                return Err(e);
            }
        }

        // set new rev number to json_post
        let rev_uped = match self.rev_uped() {
            Ok(r) => r,
            Err(e) => return Err(e),
        };
        json_post["data"]["page"]["rev"] = rev_uped.into();

        // get a new page from json_post
        let mut pate_post = match page_utility::page_from_json(json_post, &self.path) {
            Ok(p) => p,
            Err(e) => return Err(e),
        };

        if let Err(e) = pate_post.page_save() {
            eprintln!("page_save err: {:?}", e);
        }
        if let Err(e) = pate_post.page_save_rev() {
            eprintln!("page_save_rev err: {:?}", e);
        }

        Ok(())
    }

    fn json_post_rev_match(&mut self, json_post: &json::JsonValue) -> Result<(), std::io::Error> {
        let rev = match self.rev() {
            Ok(r) => r,
            Err(e) => return Err(e),
        };

        let rev_replace = match &json_post["data"]["page"]["rev"].as_u32() {
            Some(r) => *r,
            None => {
                return Err(Error::new(
                    ErrorKind::Other,
                    "Can not find rev in json given",
                ));
            }
        };
        if rev != rev_replace {
            // dbg
            // eprintln!(
            //     "page.rs fn json_post_rev_match rev(not match): {}:{}",
            //     rev, rev_replace
            // );
            return Err(Error::new(ErrorKind::Other, "rev does not match"));
        }

        Ok(())
    }
}

// Arc, Mutex do not help
// pub struct Test {
//     // dom: Option<RcDom>,
//     // dom_ac: Option<Arc<Mutex<String>>>,
//     dom_ac: Option<Arc<Mutex<RcDom>>>,
// }

// impl Test {
//     fn from() -> Test {
//         //         let html = "<html></html>";
//         //         let dom = dom_utility::to_dom(&html);
//         //         Test { dom }
//         Test {
//             // dom: None,
//             dom_ac: None,
//         }
//     }
// }
