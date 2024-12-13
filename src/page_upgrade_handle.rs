use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use tracing::{error, info};
// {event, info, instrument, span, Level, Node, error, }
use super::wc_handler::page::{self, Page};

pub fn pages_upgrade_handle_and_backup_delete(stor_root: &str, page_top_path: &str) {
    let upres = Upres {
        already: vec![],
        upgraded: vec![],
        failed: vec![],
        handled: HashSet::new(),
    };

    let upres = Rc::new(RefCell::new(upres));

    // upgrade this page and its children as recursive option is ture.
    let mut page_top = page_top(stor_root, page_top_path);
    // page_top.upgrade(true, Some(Rc::clone(&upres)));
    page_top.upgrade_and_backup_delete(true, Some(Rc::clone(&upres)));

    tracing_page_save(&mut page_top, Rc::clone(&upres));
}

/// Return top page.
/// If not exists, create top page.
fn page_top(stor_root: &str, page_top_path: &str) -> Page {
    // let page_path = "/wc_top.html";

    // DBG
    // let page_path = "/Computing/Html/html_basic.html";
    // let page_path = "/Computing/computing_iroiro.html";
    // let page_path = "/Computing/computing_index.html";
    // let page_path = "/Computing/windows/windows10/windows10openssh.html";
    // let page_path = "/Computing/Linux/Package/Yum/linux_yum_index.html";
    // let page_path = "/Computing/Language/computer_language_index.html";
    // let page_path = "/pages/Computing/Windows/windows10/windows10openssh.html";

    let mut page = Page::new(stor_root, page_top_path);
    // Already exists.
    if page.source().is_some() {
        return page;
    }

    page_top_new(stor_root, page_top_path)

    // // Create a new page Wc_top.html
    // let title = "Top";

    // // json plain
    // let mut page_json = page::page_json::page_json_plain();

    // // title
    // page_json["data"]["page"]["title"] = title.into();

    // // navi
    // let mut navi = json::JsonValue::Array(vec![]);
    // let navi_top: Vec<json::JsonValue> = vec![title.into(), "".into()];

    // let _ = navi.push(json::JsonValue::Array(navi_top));
    // page_json["data"]["navi"] = navi;

    // let mut page = page::page_utility::page_from_json(stor_root, page_top_path, &page_json);
    // let _ = page.file_save_and_rev();

    // page
}

/// Create a new page Wc_top.html
fn page_top_new(stor_root: &str, page_top_path: &str) -> Page {
    let title = "Top";

    // json plain
    let mut page_json = page::page_json::page_json_plain();

    // title
    page_json["data"]["page"]["title"] = title.into();

    // navi
    let mut navi = json::JsonValue::Array(vec![]);
    let navi_top: Vec<json::JsonValue> = vec![title.into(), "".into()];

    let _ = navi.push(json::JsonValue::Array(navi_top));
    page_json["data"]["navi"] = navi;

    // let mut page = page::page_utility::page_from_json(stor_root, page_top_path, &page_json);
    let mut page = Page::from_json(stor_root, page_top_path, &page_json);
    let _ = page.file_save_and_rev();

    page
}

pub struct Upres {
    already: Vec<String>,
    upgraded: Vec<String>,
    failed: Vec<String>,
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

    fn upgraded_list(&self) -> Vec<String> {
        let mut mv = vec![];
        for path in &self.upgraded {
            mv.push(path.as_str().to_string());
        }
        mv
    }

    fn already_list(&self) -> Vec<String> {
        let mut mv = vec![];
        for path in &self.already {
            mv.push(path.as_str().to_string());
        }
        mv
    }

    fn failed_list(&self) -> Vec<String> {
        let mut mv = vec![];
        for path in &self.failed {
            mv.push(path.as_str().to_string());
        }
        mv
    }
}

fn tracing_page_save(page_top: &mut Page, upres: Rc<RefCell<Upres>>) {
    let file_name = "page_upgrade_log.html";
    let page_path = "/".to_string() + file_name;
    let mut page_upgrade_log =
        page::page_utility::page_child_new(page_top, "page_upgrade_log", file_name)
            .or_else(|_| {
                // "/page_upgrade_log.html"
                Ok::<Page, ()>(Page::new(page_top.stor_root(), &page_path))
            })
            .unwrap();

    let Some(page_json) = page_upgrade_log.json_value() else {
        return;
    };

    let mut page_json = page_json.clone();
    let json::JsonValue::Object(ref mut subsections_json) =
        &mut page_json["data"]["subsection"]["data"]
    else {
        return;
    };

    let subsection_top = &mut subsections_json["0"];
    subsection_top["child"] = json::array![1, 2, 3];

    // failed
    let id = 1;
    let mut subsection = subsection_new("failed");
    subsection["id"] = id.into();
    let json::JsonValue::Array(contents) = &mut subsection["content"] else {
        error!("Failed to get content from subsection");
        return;
    };
    let mut list = upres.borrow().failed_list();
    let list = list_str(&mut list);
    let _ = contents[0].insert::<json::JsonValue>("value", list.into());
    subsections_json[id.to_string().as_str()] = subsection;

    // upgraded
    let id = 2;
    let mut subsection = subsection_new("upgraded");
    subsection["id"] = id.into();
    let json::JsonValue::Array(contents) = &mut subsection["content"] else {
        return;
    };
    let mut list = upres.borrow().upgraded_list();
    let list = list_str(&mut list);
    let _ = contents[0].insert::<json::JsonValue>("value", list.into());
    subsections_json[id.to_string().as_str()] = subsection;

    // already
    let id = 3;
    let mut subsection = subsection_new("already");
    subsection["id"] = id.into();
    let json::JsonValue::Array(contents) = &mut subsection["content"] else {
        return;
    };
    let mut list = upres.borrow().already_list();
    let list = list_str(&mut list);
    let _ = contents[0].insert::<json::JsonValue>("value", list.into());
    subsections_json[id.to_string().as_str()] = subsection;

    page_json["data"]["subsection"]["id"]["id_next"] = 4.into();

    if let Err(e) = page_upgrade_log.json_replace_save(page_json) {
        error!("{}", &e);
    };

    info!("Page upgrade result: {}", page_path);
}

fn subsection_new(href: &str) -> json::JsonValue {
    let title = href;
    let href = "#".to_string() + href;
    json::object! {
    "parent" : "0",
    "id" : "1",
     "title" : title,
     "href" : href.as_str() ,
    "content" : [{"type": "text"}],
    "child" : []
    }
}

fn list_str<'a>(list: &mut Vec<String>) -> String {
    let num = list.len();
    list.push(num.to_string());
    list.rotate_right(1);
    list.join("\n")
}
