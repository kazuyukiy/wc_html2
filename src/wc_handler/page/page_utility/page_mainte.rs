use super::dom_utility;
use super::fs_write;
use super::href_on;
use super::json_from_dom;
use super::page_from_json;
pub use super::page_json;
use super::page_url;
use super::Page;

pub use super::page_child_new;

use std::cell::RefCell;
use std::rc::Rc;
// use tracing::{error, info};

pub mod page_backup_clean;
pub mod page_form_update;

pub fn page_mainte(
    page: &mut super::Page,
    recursive: bool,
    log: Option<Rc<RefCell<page_form_update::Log>>>,
) {
    page_form_update::page_form_update(page, recursive, log);

    page_backup_clean::page_backup_clean(page, recursive);
}
