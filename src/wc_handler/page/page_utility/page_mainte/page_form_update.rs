use super::dom_utility;
use super::fs_write;
use super::href_on;
use super::json_from_dom;
use super::page_child_new;
use super::page_json;
use super::page_url;
use tracing::info; // {error, event, info, instrument, span, Level, Node}
                   // use super::page_utility;
use super::Page;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use tracing::error; // {event, info, instrument, span, Level, Node, info}

/// Update page form converting from old page style to the latest one.
pub fn page_form_update(page: &mut Page, recursive: bool, log: Option<Rc<RefCell<Log>>>) {
    let (log_org, log) = log_ini(log);

    // alredy handled
    if log.borrow().handled(page) {
        return;
    }

    let page_dom = match page.dom() {
        Some(v) => v,
        None => {
            // page_upgrade_failed(page, &upres);
            log.borrow_mut().failed(page);
            error!("Failed to get page_dom: {}", &page.file_path());
            return;
        }
    };
    let page_node = &page_dom.document;

    // check if page type is the latest or to be updated.

    // page_top found
    // It is current page style, not for upgrade
    if dom_utility::get_div_page_top(&page_node).is_some() {
        // page_upgrade_already(page, &upres);
        log.borrow_mut().already(page);
        return;
    }

    // let json_value = super::json_from_dom(page_node);
    let json_value = json_from_dom(page_node);

    // Failed to get page_json
    if json_value.is_none() {
        // page_upgrade_failed(page, &upres);
        log.borrow_mut().failed(page);
        error!("Failed to get page_json: {}", page.page_path());
        return;
    }

    // Save the page updated.

    let mut json_value = json_value.unwrap();

    // Get the last_rev, otherwise use 1.
    // Not sure if page has a valud rev in page.json.
    // So you can not use page_json.rev(), get it from json_value.
    let rev = super::page_json::to_usize(&json_value["data"]["page"]["rev"]).ok();

    // Confirm last rev on real files.
    let last_rev = last_rev_of_files(page, rev).or(Some(1)).unwrap();

    // Make an origin backup of the file before changes.
    // last_rev +_1 will be used for back up of the original file and
    // rev_backup (last_rev + 1) will be retuned.
    let Some(rev_backup) = page_org_backup(page, &last_rev) else {
        // page_upgrade_failed(page, &upres);
        log.borrow_mut().failed(page);
        error!("Failed to make an original backup: {}", &page.file_path());
        return;
    };
    json_value["data"]["page"]["rev"] = rev_backup.into();

    // // rev_backup;

    // // rev_upded was used for the original backup.
    // // Get new rev: rev_upgrade = rev_upded + 1
    // let rev_upgrade = match rev_backup.checked_add(1) {
    //     Some(v) => v,
    //     None => {
    //         // page_upgrade_failed(page, &upres);
    //         log.borrow_mut().failed(page);
    //         error!(
    //             "Failed to get new rev: {} + 1, on {}",
    //             rev_backup,
    //             &page.file_path()
    //         );
    //         return;
    //     }
    // };

    // Set last rev not to overwrite on old file.
    // json_value["data"]["page"]["rev"] = rev_upgrade.into();

    // page.json_replace_save(json_data) does not work
    // because it needs original json value of the page
    // in span element of the body that does not exists.
    let mut page2 = Page::from_json(page.stor_root(), page.page_path(), &json_value);
    let _ = page2.rev_replace_one_up();

    if let Ok(_) = page2.file_save_and_rev() {
        // page_upgrade_upgraded(page, &upres);
        log.borrow_mut().updated(page);
    }

    // It will call page_form_update_children() and json_from_dom() later;
    // To avoid same procedure again, set json_value on the page.
    let page_json = page_json::PageJson::from(json_value.take());
    page.json.replace(Some(page_json));

    //
    if recursive {
        // page_form_update_children(page, recursive, Some(log));
        page_form_update_children(page, recursive, Some(Rc::clone(&log)));
    }

    if log_org {
        tracing_page_save(page, Rc::clone(&log));
    }
}

pub fn page_form_update_(page: &mut Page, recursive: bool, log: Option<Rc<RefCell<Log>>>) {
    let (log_org, log) = log_ini(log);

    // alredy handled
    if log.borrow().handled(page) {
        return;
    }

    let page_dom = match page.dom() {
        Some(v) => v,
        None => {
            // page_upgrade_failed(page, &upres);
            log.borrow_mut().failed(page);
            error!("Failed to get page_dom: {}", &page.file_path());
            return;
        }
    };
    let page_node = &page_dom.document;

    // check if page type is the latest or to be updated.

    // page_top found
    // It is current page style, not for upgrade
    if dom_utility::get_div_page_top(&page_node).is_some() {
        // page_upgrade_already(page, &upres);
        log.borrow_mut().already(page);
        return;
    }

    // let json_value = super::json_from_dom(page_node);
    let json_value = json_from_dom(page_node);

    // Failed to get page_json
    if json_value.is_none() {
        // page_upgrade_failed(page, &upres);
        log.borrow_mut().failed(page);
        error!("Failed to get page_json: {}", page.page_path());
        return;
    }

    // Save the page updated.

    let mut json_value = json_value.unwrap();

    // Get the last_rev, otherwise use 1.
    // Not sure if page has a valud rev in page.json.
    // So you can not use page_json.rev(), get it from json_value.
    let rev = super::page_json::to_usize(&json_value["data"]["page"]["rev"]).ok();

    // Confirm last rev on real files.
    let last_rev = last_rev_of_files(page, rev).or(Some(1)).unwrap();

    // Make an origin backup of the file before changes.
    // last_rev +_1 will be used for back up of the original file and
    // rev_backup (last_rev + 1) will be retuned.
    let Some(rev_backup) = page_org_backup(page, &last_rev) else {
        // page_upgrade_failed(page, &upres);
        log.borrow_mut().failed(page);
        error!("Failed to make an original backup: {}", &page.file_path());
        return;
    };

    // rev_backup;

    // rev_upded was used for the original backup.
    // Get new rev: rev_upgrade = rev_upded + 1
    let rev_upgrade = match rev_backup.checked_add(1) {
        Some(v) => v,
        None => {
            // page_upgrade_failed(page, &upres);
            log.borrow_mut().failed(page);
            error!(
                "Failed to get new rev: {} + 1, on {}",
                rev_backup,
                &page.file_path()
            );
            return;
        }
    };

    // Set last rev not to overwrite on old file.
    json_value["data"]["page"]["rev"] = rev_upgrade.into();

    // page.json_replace_save(json_data) does not work
    // because it needs original json value of the page
    // in span element of the body that does not exists.
    let mut page2 = Page::from_json(page.stor_root(), page.page_path(), &json_value);

    if let Ok(_) = page2.file_save_and_rev() {
        // page_upgrade_upgraded(page, &upres);
        log.borrow_mut().updated(page);
    }

    // It will call page_form_update_children() and json_from_dom() later;
    // To avoid same procedure again, set json_value on the page.
    let page_json = page_json::PageJson::from(json_value.take());
    page.json.replace(Some(page_json));

    //
    if recursive {
        // page_form_update_children(page, recursive, Some(log));
        page_form_update_children(page, recursive, Some(Rc::clone(&log)));
    }

    if log_org {
        tracing_page_save(page, Rc::clone(&log));
    }
}

fn page_form_update_children(page: &mut Page, recursive: bool, log: Option<Rc<RefCell<Log>>>) {
    // To avoid
    // error[E0499]: cannot borrow `*page` as mutable more than once at a time
    // get page_url at here previously
    let Ok(page_url) = page_url(page) else {
        return;
    };

    let stor_root = page.stor_root().to_string();

    let page_json = page.json_mut();
    if page_json.is_none() {
        return;
    }
    let Some(subsections_json) = page_json.unwrap().subsections() else {
        return;
    };

    for (_, subsection_json) in subsections_json.iter() {
        // subsection_json["href"]
        // info!("href: {}", subsection_json["href"]);

        let Some(href) = subsection_json["href"].as_str() else {
            continue;
        };

        // href is not to child of the page
        let Some((_, is_child)) = href_on(&page_url, href) else {
            continue;
        };
        if !is_child {
            continue;
        }

        let Ok(href_url) = page_url.join(href) else {
            continue;
        };

        // info!("href_url: {}", href_url);

        let mut child_page = super::Page::new(&stor_root, href_url.path());
        let log_child = log.as_ref().and_then(|ref v| Some(Rc::clone(v)));
        // child_page.upgrade(recursive, log_child);
        // child_page.mainte(recursive, log_child);
        page_form_update(&mut child_page, recursive, log_child);
    }
    // pag_utility.rs pub fn page_upgrade_and_delete_children(
}

pub struct Log {
    already: Vec<String>,
    updated: Vec<String>,
    failed: Vec<String>,
    handled: HashSet<String>,
}

impl Log {
    pub fn new() -> Log {
        Log {
            already: vec![],
            updated: vec![],
            failed: vec![],
            handled: HashSet::new(),
        }
    }

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

    pub fn updated(&mut self, page: &mut Page) {
        if self.handled(page) {
            return;
        }
        let page_path = page.page_path();
        self.updated.push(page_path.to_string());
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

    fn updated_list(&self) -> Vec<String> {
        let mut mv = vec![];
        for path in &self.updated {
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

/// useage let (log_org, log) = log_ini(log);
/// Return values,
/// log_org:
///   true: new empty Log was created at here.
///   false: log already has Log in Some
/// log: the argument it self or new empty Log
// fn log_ini(log: Option<Rc<RefCell<Log>>>) -> (bool, Option<Rc<RefCell<Log>>>) {
fn log_ini(log: Option<Rc<RefCell<Log>>>) -> (bool, Rc<RefCell<Log>>) {
    //
    if log.is_some() {
        // return (false, log);
        return (false, log.unwrap());
    }

    let log = Rc::new(RefCell::new(Log::new()));
    // (true, Some(log))
    (true, log)
}

/// Find max rev number of the page in existing files.
/// rev: current rev
fn last_rev_of_files(page: &mut super::Page, rev: Option<usize>) -> Option<usize> {
    // rev is some number that is known as the last.
    // otherwise set 1.
    let rev = rev.or(Some(1)).unwrap();

    let mut rev_last = rev;
    // fm: rev + 1
    let fm = rev.checked_add(1).or(Some(usize::MAX)).unwrap();
    let to = rev.checked_add(100).or(Some(usize::MAX)).unwrap();
    for rev in fm..to {
        // rev_last = rev;
        // let path_rev = page.file_path() + "." + rev.to_string().as_str();
        let path_rev = page.path_rev_form(rev);
        if let Ok(exists) = std::fs::exists(&path_rev) {
            if exists {
                rev_last = rev;
                continue;
            }
        }
        break;
    }
    Some(rev_last)
}

/// Make a page file bakckup.
/// This function should work even the file contents is not for wc_noe page type.
/// arguments rev_crt + 1 will be used for this backup file name and
/// returns new rev: rev_crt + 1 in Some() if sucseeded.
fn page_org_backup(page: &mut Page, rev_crt: &usize) -> Option<usize> {
    // backup with new rev number.
    let rev_uped = rev_crt.checked_add(1)?;

    // let path_rev_uped = page.file_path() + "." + rev_uped.to_string().as_str();
    let path_rev_uped = page.path_rev_form(rev_uped);
    let path_rev_uped_ref = match path_rev_uped.to_str() {
        Some(v) => v,
        None => {
            error!("Failed to get str from: {:?}", &path_rev_uped);
            return None;
        }
    };

    let source = page.source()?;
    // match super::fs_write(&path_rev_uped, source) {
    match fs_write(path_rev_uped_ref, source) {
        Ok(_) => {
            // info!("Original backup: {}", &path_rev_uped_ref);
            //
            Some(rev_uped)
        }
        Err(e) => {
            error!("Failed, Original backup: {}, {}", &path_rev_uped_ref, e);
            None
        }
    }
}

fn tracing_page_save(page_some: &mut Page, log: Rc<RefCell<Log>>) {
    let stor_root = page_some.stor_root();
    let mut page_top = Page::new(stor_root, "/wc_top.html");

    let file_name = "page_form_updated_log.html";
    let page_path = "/".to_string() + file_name;
    let mut page_form_update_log =
        // page::page_utility::page_child_new(page_top, "page_form_update_log", file_name)
        page_child_new(&mut page_top, "page_form_updated_log", file_name)
            .or_else(|_| {
                // "/page_form_updated_log.html"
                Ok::<Page, ()>(Page::new(page_top.stor_root(), &page_path))
            })
            .unwrap();

    let Some(page_json) = page_form_update_log.json_value() else {
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
    let mut list = log.borrow().failed_list();
    let list = list_str(&mut list);
    let _ = contents[0].insert::<json::JsonValue>("value", list.into());
    subsections_json[id.to_string().as_str()] = subsection;

    // updated
    let id = 2;
    let mut subsection = subsection_new("updated");
    subsection["id"] = id.into();
    let json::JsonValue::Array(contents) = &mut subsection["content"] else {
        return;
    };
    let mut list = log.borrow().updated_list();
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
    let mut list = log.borrow().already_list();
    let list = list_str(&mut list);
    let _ = contents[0].insert::<json::JsonValue>("value", list.into());
    subsections_json[id.to_string().as_str()] = subsection;

    page_json["data"]["subsection"]["id"]["id_next"] = 4.into();

    if let Err(e) = page_form_update_log.json_replace_save(page_json) {
        error!("{}", &e);
    };

    info!("Page form update result: {}", page_path);
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
