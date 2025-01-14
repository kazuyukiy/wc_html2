use super::dom_utility;
use super::{PageJson, Subsection};
use html5ever::serialize::SerializeOpts;
use markup5ever_rcdom::{Handle, Node, NodeData, SerializableHandle}; // Handle, RcDom;
use std::rc::Rc;

// <div class="subsection">
// <a name="meta"></a>

pub fn json_from_dom_html(page_node: &Handle) -> Option<json::JsonValue> {
    let mut page_json = PageJson::new();
    let page_json_data = page_json.value_mut()?;

    json_html_title(page_node, page_json_data)?;
    json_html_navi(page_node, page_json_data)?;
    let parent_id = 0;
    json_html_ul(page_node, &mut page_json, &parent_id);

    subsections_contents_set(page_node, &mut page_json);

    Some(page_json.value_take()?)
}

// <title>title_name</title>
fn json_html_title(page_node: &Rc<Node>, page_json: &mut json::JsonValue) -> Option<()> {
    let ptn = dom_utility::node_element("title", &vec![]);
    let title_node = dom_utility::child_match_first(&page_node, &ptn, true)?;
    for child in title_node.children.borrow().iter() {
        if let NodeData::Text { contents } = &child.data {
            let value = contents.borrow().to_string();
            page_json["data"]["page"]["title"] = value.as_str().into();
            break;
        }
    }

    Some(())
}

fn json_html_navi(page_node: &Rc<Node>, page_json: &mut json::JsonValue) -> Option<()> {
    let navis_json = &mut page_json["data"]["navi"];
    // div for navi
    let ptn = dom_utility::node_element("div", &vec![]);
    let div = dom_utility::child_match_first(&page_node, &ptn, true)?;
    let ptn = dom_utility::node_element("a", &vec![]);
    let a_list = dom_utility::child_match_list(&div, &ptn, false, false);
    let mut navi_list = vec![];
    for a_node in a_list {
        let (href, text) = a_node_href_title(&a_node);
        navi_list.push((href, text));
    }

    // Correct top page file name.
    if 0 < navi_list.len() {
        navi_list[0].0 = navi_list[0].0.replace("WC_top.html", "wc_top.html");
    }

    for navi in navi_list {
        let mut navi_json = json::JsonValue::Array(vec![]);
        let _ = navi_json.push::<json::JsonValue>(navi.1.into());
        let _ = navi_json.push::<json::JsonValue>(navi.0.into());

        let _ = navis_json.push(navi_json);
    }
    Some(())
}

/// Get href and text content of a node;
fn a_node_href_title(a_node: &Rc<Node>) -> (String, String) {
    let mut href = None;
    if let NodeData::Element { attrs, .. } = &a_node.data {
        for attr in attrs.borrow().iter() {
            if &attr.name.local == "href" {
                let value = String::from(&attr.value);
                href.replace(value);
                break;
            }
        }
    }
    let href = href.or(Some("".to_string())).unwrap();

    let mut title = None;
    for child in a_node.children.borrow().iter() {
        if let NodeData::Text { contents } = &child.data {
            let value = contents.borrow().to_string();
            title.replace(value);
            break;
        }
    }
    let title = title.or(Some("".to_string())).unwrap();

    (href, title)
}

fn json_html_ul(parent_node: &Rc<Node>, page_json: &mut PageJson, parent_id: &usize) {
    let ptn = dom_utility::node_element("ul", &vec![]);

    let Some(ul_node) = dom_utility::child_match_first(&parent_node, &ptn, true) else {
        return;
    };

    let ptn = dom_utility::node_element("li", &vec![]);
    let li_list = dom_utility::child_match_list(&ul_node, &ptn, false, false);
    for li in li_list {
        let Some(subsection_id) = json_html_li(&li, parent_id, page_json) else {
            continue;
        };
        json_html_ul(&li, page_json, &subsection_id);
    }
}

fn json_html_li(li_node: &Rc<Node>, parent_id: &usize, page_json: &mut PageJson) -> Option<usize> {
    let ptn = dom_utility::node_element("a", &vec![]);
    let a_node = dom_utility::child_match_first(&li_node, &ptn, false)?;

    let (href, title) = a_node_href_title(&a_node);
    if href.len() == 0 || href == "#" {
        return None;
    }

    let mut subsection = page_json.subsection_new(parent_id)?;

    subsection.title_set(&title);
    subsection.href_set(&href);
    let subsection_id = subsection.id();

    Some(subsection_id)
}

fn subsections_contents_set(page_node: &Rc<Node>, page_json: &mut PageJson) -> Option<()> {
    let ptn = dom_utility::node_element("body", &vec![]);
    let body_node = dom_utility::child_match_first(&page_node, &ptn, true)?;

    // <div class="subsection">
    let attrs = &vec![("class", "subsection")];
    let ptn = dom_utility::node_element("div", &attrs);
    let div_list = dom_utility::child_match_list(&body_node, &ptn, false, false);

    for subsection_node in div_list {
        subsection_contents_set(&subsection_node, page_json);
    }

    Some(())
}

fn subsection_contents_set(subsection_node: &Rc<Node>, page_json: &mut PageJson) -> Option<()> {
    // <div class="subsection">

    // name(key)
    // <a name="subsection_name"></a>
    let mut name = subsection_name(&subsection_node)?;

    if name.len() == 0 {
        return None;
    }

    // # + subsection_name
    name.insert_str(0, "#");
    // let mut subsection = page_json.subsection_by_name(&name)?;
    let mut subsection = page_json.subsection_by_name(&name);
    if subsection.is_none() {
        subsection = subsection_by_subsection_node(subsection_node, page_json, &name);
    }
    if subsection.is_none() {
        return None;
    }
    let mut subsection = subsection.unwrap();

    // title <div class="subsectionTitle">

    // contents
    // <div class="subsectionBody">
    let attrs = &vec![("class", "subsectionBody")];
    let ptn = dom_utility::node_element("div", &attrs);
    let contents_node = dom_utility::child_match_first(subsection_node, &ptn, false)?;

    let subsection_content = subsection.contents_mut();

    for content in contents_node.children.borrow().iter() {
        subsection_content_set(subsection_content, content);
    }

    Some(())
}

fn subsection_by_subsection_node<'a>(
    subsection_node: &'a Rc<Node>,
    page_json: &'a mut PageJson,
    name: &'a str,
) -> Option<Subsection<'a>> {
    // Create a new subseciton.
    // Since name is not found in li elements, consume that parent_id = 0.
    let parent_id = 0;
    let mut subsection = page_json.subsection_new(&parent_id)?;
    // title
    let attrs = &vec![("class", "subsectionTitle")];
    let ptn = dom_utility::node_element("div", &attrs);
    let div_node = dom_utility::child_match_first(subsection_node, &ptn, true);
    let mut title = None;
    if let Some(div_node) = div_node {
        for child in div_node.children.borrow().iter() {
            if let NodeData::Text { contents } = &child.data {
                title.replace(contents.borrow().to_string());
                break;
            }
        }
    }
    let title = title.or(Some("".to_string())).unwrap();
    subsection.href_set(name);
    subsection.title_set(&title);

    // subsection
    Some(subsection)
}

fn subsection_content_set(subsection_content: &mut json::JsonValue, content: &Rc<Node>) {
    match &content.data {
        // type: script, text, html
        NodeData::Element { name, .. } => {
            // type: script
            // <textarea class="scriptSample" readonly style="height: 15em;">
            if *name.local == *"textarea" {
                let content_children = &content.children;
                let mut content_script = String::new();
                for content_child in content_children.borrow().iter() {
                    if let NodeData::Text { contents } = &content_child.data {
                        // < to \<, > to \>
                        // let value = text_contents_gt_lt_escape(contents);
                        let value = contents.borrow().to_string();
                        let value = text_contents_gt_lt_escape(&value);
                        content_script.push_str(&value);
                    }
                }
                let _ = subsection_content
                    .push(json::object! {"type": "script", "value": content_script.as_str()});
            }
            // type: html, serialize the content as html text.
            else {
                let sh = SerializableHandle::from(content.clone());
                let mut bytes = vec![];
                let r = html5ever::serialize(&mut bytes, &sh, SerializeOpts::default());
                if r.is_err() {
                    return;
                }
                if let Ok(v) = String::from_utf8(bytes) {
                    let _ = subsection_content
                        .push(json::object! {"type": "html", "value": v.as_str()});
                }
            };
        }

        NodeData::Text { contents } => {
            // < to \<, > to \>
            // let value = text_contents_gt_lt_escape(contents);
            let value = contents.borrow().to_string();
            let value = text_contents_gt_lt_escape(&value);
            let _ =
                subsection_content.push(json::object! {"type": "text", "value": value.as_str()});
        }
        _ => (),
    }
}

/// < to \<, > to \>
// fn text_contents_gt_lt_escape(contents: &RefCell<Tendril<UTF8>>) -> String {
fn text_contents_gt_lt_escape(contents: &str) -> String {
    // let value = contents.borrow().to_string();
    let value = contents.to_string();
    let value = value.replace("<", "\\<");
    let value = value.replace(">", "\\>");
    value
}

/// Return subsection name (key)
/// <a name="subsection_name"></a>
fn subsection_name(subsection_node: &Rc<Node>) -> Option<String> {
    // let mut name = None;
    let ptn = dom_utility::node_element("a", &vec![]);
    let a_list = dom_utility::child_match_list(subsection_node, &ptn, false, false);
    for a_node in a_list {
        if let NodeData::Element { attrs, .. } = &a_node.data {
            for attr in attrs.borrow().iter() {
                if &attr.name.local == "name" {
                    let value = String::from(&attr.value);
                    return Some(value);
                }
            }
        }
    }
    None
}
