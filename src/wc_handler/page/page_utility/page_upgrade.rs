use super::dom_utility;
use super::page_json;
use super::Page;
use super::Upres;
use std::cell::RefCell;
use std::rc::Rc;
use tracing::error; // {event, info, instrument, span, Level, Node, info}

pub fn page_upgrade(page: &mut Page, upres: Option<Rc<RefCell<Upres>>>) {
    // if the page already handle
    if page_upgrade_handled(page, &upres) {
        return;
    }

    let page_dom = match page.dom() {
        Some(v) => v,
        None => {
            page_upgrade_failed(page, &upres);
            error!("Failed to get page_dom: {}", &page.file_path());
            return;
        }
    };
    let page_node = &page_dom.document;

    // check if page type is the latest or to be upgraded.

    // page_top found
    // It is current page style, not for upgrade
    if dom_utility::get_div_page_top(&page_node).is_some() {
        page_upgrade_already(page, &upres);
        return;
    }

    let json_value = super::json_from_dom(page_node);
    // // Get json_value from span element.
    // // <span id="page_json_str" style="display: none"></span>
    // let mut json_value = super::json_from_dom_span(page_node);

    // // <script type="text/javascript" class="page_json">let page_json = {}</script>
    // if json_value.is_none() {
    //     json_value = super::json_from_dom_script(page_node);
    // }

    // // json_value not found in the page, create it from page html.
    // if json_value.is_none() {
    //     // parse html page in old style to josn_value
    //     json_value = super::json_from_dom_html(page_node);
    // }

    // Failed to get page_json
    if json_value.is_none() {
        page_upgrade_failed(page, &upres);
        error!("Failed to get page_json: {}", page.page_path());
        return;
    }

    // Save the page upgraded.

    let mut json_value = json_value.unwrap();

    // Get the last_rev, otherwise use 1.
    // Not sure if page has a valud rev in page.json.
    // So you can not use page_json.rev(), get it from json_value.
    // let rev = page.json().and_then(|page_json| page_json.rev());
    let rev = super::page_json::to_usize(&json_value["data"]["page"]["rev"]).ok();

    // Confirm last rev on real files.
    let last_rev = last_rev_of_files(page, rev).or(Some(1)).unwrap();

    // Make an origin backup of the file before changes.
    // last_rev +_1 will be used for back up of the original file and
    // rev_backup (last_rev + 1) will be retuned.
    let Some(rev_backup) = page_org_backup(page, &last_rev) else {
        page_upgrade_failed(page, &upres);
        error!("Failed to make an original backup: {}", &page.file_path());
        return;
    };

    // rev_upded was used for the original backup.
    // Get new rev: rev_upgrade = rev_upded + 1
    let rev_upgrade = match rev_backup.checked_add(1) {
        Some(v) => v,
        None => {
            page_upgrade_failed(page, &upres);
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
    let mut page2 = super::page_from_json(page.stor_root(), page.page_path(), &json_value);

    // DBG
    // page_upgrade_upgraded(page, &upres);
    // return;

    if let Ok(_) = page2.file_save_and_rev() {
        page_upgrade_upgraded(page, &upres);
    }

    // on page.upgrade(), it will call page_upgrade_child() after this function.
    // at there, it will call page.json()
    // and page_urility::json_from_dom will be called again, that was called in this function.
    // To avoid same procedure again, set json_value on the page.
    let page_json = page_json::PageJson::from(json_value.take());
    page.json.replace(Some(page_json));

    // info!("fn page_upgrade end");
}

fn page_upgrade_handled(page: &mut Page, upres: &Option<Rc<RefCell<Upres>>>) -> bool {
    if upres.is_some() {
        upres.as_ref().unwrap().borrow_mut().handled(page)
    } else {
        false
    }
}

fn page_upgrade_upgraded(page: &mut Page, upres: &Option<Rc<RefCell<Upres>>>) {
    if upres.is_some() {
        upres.as_ref().unwrap().borrow_mut().upgraded(page);
    }
}

fn page_upgrade_already(page: &mut Page, upres: &Option<Rc<RefCell<Upres>>>) {
    if upres.is_some() {
        upres.as_ref().unwrap().borrow_mut().already(page);
    }
}

fn page_upgrade_failed(page: &mut Page, upres: &Option<Rc<RefCell<Upres>>>) {
    if upres.is_some() {
        upres.as_ref().unwrap().borrow_mut().failed(page);
    }
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
    match super::fs_write(path_rev_uped_ref, source) {
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
