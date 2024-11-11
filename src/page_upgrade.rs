use super::wc_handler;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use tracing::info;
// {event, info, instrument, span, Level, Node, error, } //  error, event, instrument, span, Level debug,, info, info_span
use wc_handler::page;

pub fn pages_upgrade(stor_root: &str) {
    // top page
    // let page_path = "wc_top.html";
    // let page_path = "/Computing/Html/html_basic.html";
    // let page_path = "/Computing/computing_iroiro.html";
    let page_path = "/Computing/computing_index.html";

    let upres = Upres {
        already: vec![],
        upgraded: vec![],
        failed: vec![],
        handled: HashSet::new(),
    };

    let upres = Rc::new(RefCell::new(upres));
    let mut page = wc_handler::page::Page::new(stor_root, page_path);
    page.upgrade(true, Some(Rc::clone(&upres)));
    upres.borrow().tracing_info();
}

pub struct Upres {
    already: Vec<String>,
    upgraded: Vec<String>,
    failed: Vec<String>,
    handled: std::collections::HashSet<String>,
}

impl Upres {
    pub fn handled(&self, page: &mut page::Page) -> bool {
        self.handled.contains(page.page_path())
    }

    pub fn already(&mut self, page: &mut page::Page) {
        if self.handled(page) {
            return;
        }
        let page_path = page.page_path();
        self.already.push(page_path.to_string());
        self.handled.insert(page_path.to_string());
    }

    pub fn upgraded(&mut self, page: &mut page::Page) {
        if self.handled(page) {
            return;
        }
        let page_path = page.page_path();
        self.upgraded.push(page_path.to_string());
        self.handled.insert(page_path.to_string());
    }

    pub fn failed(&mut self, page: &mut page::Page) {
        if self.handled(page) {
            return;
        }
        let page_path = page.page_path();
        self.failed.push(page_path.to_string());
        self.handled.insert(page_path.to_string());
    }

    fn tracing_info(&self) {
        if 0 < self.upgraded.len() {
            let mut mv = vec![];
            for path in &self.upgraded {
                mv.push(path.as_str());
            }
            info!("upgraded: \n{}", mv.join("\n"));
        }

        if 0 < self.already.len() {
            let mut mv = vec![];
            for path in &self.already {
                mv.push(path.as_str());
            }
            info!("already: \n{}", mv.join("\n"));
        }

        if 0 < self.failed.len() {
            let mut mv = vec![];
            for path in &self.failed {
                mv.push(path.as_str());
            }
            info!("failed: \n{}", mv.join("\n"));
        }
    }
}
