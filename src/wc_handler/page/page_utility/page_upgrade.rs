use super::dom_utility;
use super::Page;
use super::Upres;
use std::cell::RefCell;
use std::rc::Rc;
use tracing::error; // {event, info, instrument, span, Level, Node}

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

    // Get json_value from span element.
    // <span id="page_json_str" style="display: none"></span>
    let mut json_value = super::json_from_dom_span(page_node);

    // <script type="text/javascript" class="page_json">let page_json = {}</script>
    if json_value.is_none() {
        json_value = super::json_from_dom_script(page_node);
    }

    // json_value not found in the page, create it from page html.
    if json_value.is_none() {
        // old page stype
        json_value = super::json_from_dom_html(page_node);
    }

    // Failed to get page_json
    if json_value.is_none() {
        page_upgrade_failed(page, &upres);
        error!("Failed to get page_json: {}", page.page_path());
        return;
    }

    let mut json_value = json_value.unwrap();

    // Set last rev not to overwrite on old file.
    // Otherwise set 1
    let rev = page.json().and_then(|page_json| page_json.rev());
    let last_rev = last_rev_of_files(page, rev).or(Some(1)).unwrap();
    json_value["data"]["page"]["rev"] = last_rev.into();

    // page.json_replace_save(json_data) does not work
    // because it needs original json value of the page
    // in span element of the body that does not exists.
    let mut page2 = super::page_from_json(page.stor_root(), page.page_path(), &json_value);
    if let Ok(_) = page2.file_save_and_rev() {
        page_upgrade_upgraded(page, &upres);
    }
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
    let rev = rev.or(Some(1)).unwrap();

    // Case no loop runs, returns 1.
    let mut rev_last = 1;
    // fm: rev + 1
    let fm = rev.checked_add(1).or(Some(usize::MAX)).unwrap();
    let to = rev.checked_add(100).or(Some(usize::MAX)).unwrap();
    for rev in fm..to {
        rev_last = rev;
        let path_rev = page.file_path() + "." + rev.to_string().as_str();

        if let Ok(exists) = std::fs::exists(&path_rev) {
            if exists {
                continue;
            }
        }
        break;
    }
    Some(rev_last)
}
