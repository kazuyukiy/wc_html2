use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use tracing::info;
// {event, info, instrument, span, Level, Node, error, }
use super::wc_handler::page::{self, Page};

pub fn pages_upgrade_handle(stor_root: &str) {
    let upres = Upres {
        already: vec![],
        upgraded: vec![],
        failed: vec![],
        handled: HashSet::new(),
    };
    let upres = Rc::new(RefCell::new(upres));

    // upgrade this page and its children as recursive option is ture.
    let mut page_top = page_top(stor_root);
    page_top.upgrade(true, Some(Rc::clone(&upres)));
    upres.borrow().tracing_info();
}

/// Return top page.
/// If not exists, create top page.
fn page_top(stor_root: &str) -> Page {
    let page_path = "/wc_top.html";

    // let page_path = "/Computing/Html/html_basic.html";
    // let page_path = "/Computing/computing_iroiro.html";
    // let page_path = "/Computing/computing_index.html";

    let mut page = Page::new(stor_root, page_path);
    // Already exists.
    if page.source().is_some() {
        return page;
    }

    // Create a new pate
    // let title = "Wc top";
    let title = "Top";

    // json plain
    let mut page_json = page::page_json::page_json_plain();

    // title
    page_json["data"]["page"]["title"] = title.into();

    // navi
    let mut navi = json::JsonValue::Array(vec![]);
    let navi_top: Vec<json::JsonValue> = vec![title.into(), "".into()];

    // if navi.push(json::JsonValue::Array(navi_top)).is_err() {
    //     return Err(());
    // }
    let _ = navi.push(json::JsonValue::Array(navi_top));
    page_json["data"]["navi"] = navi;

    let mut page = page::page_utility::page_from_json(stor_root, page_path, &page_json);
    let _ = page.file_save_and_rev();

    page
}

pub struct Upres {
    already: Vec<String>,
    upgraded: Vec<String>,
    failed: Vec<String>,
    // handled: std::collections::HashSet<String>,
    handled: HashSet<String>,
}

impl Upres {
    pub fn handled(&self, page: &mut Page) -> bool {
        self.handled.contains(page.page_path())
    }

    pub fn already(&mut self, page: &mut Page) {
        if self.handled(page) {
            return;
        }
        let page_path = page.page_path();
        self.already.push(page_path.to_string());
        self.handled.insert(page_path.to_string());
    }

    pub fn upgraded(&mut self, page: &mut Page) {
        if self.handled(page) {
            return;
        }
        let page_path = page.page_path();
        self.upgraded.push(page_path.to_string());
        self.handled.insert(page_path.to_string());
    }

    pub fn failed(&mut self, page: &mut Page) {
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
