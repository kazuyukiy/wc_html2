use super::super::page_json::{PageJson, Subsection};
use super::dom_utility;
use markup5ever_rcdom::Handle;
// use tracing::info; // {error, event, info, instrument, span, Level, Node}
mod json_from_dom_html_type01;
mod json_from_dom_html_type02;

/// Convert page_dom that represent page contents, not page data in json as text,
/// to page_json data
pub fn json_from_dom_html(page_node: &Handle) -> Option<json::JsonValue> {
    if let Some(json) = json_from_dom_html_type02::json_from_dom_html(page_node) {
        return Some(json);
    };

    if let Some(json) = json_from_dom_html_type01::json_from_dom_html(page_node) {
        return Some(json);
    }

    None
}
