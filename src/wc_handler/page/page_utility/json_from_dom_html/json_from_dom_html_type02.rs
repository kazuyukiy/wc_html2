use super::dom_utility;
use super::{PageJson, Subsection};
use html5ever::serialize::SerializeOpts;
use markup5ever_rcdom::{Handle, Node, NodeData, SerializableHandle}; // Handle, RcDom;
use std::rc::Rc;
// use tracing::info; // {error, event, info, instrument, span, Level, Node}

pub fn json_from_dom_html(page_node: &Handle) -> Option<json::JsonValue> {
    // if <ul class="listItemBase"> contained
    if !is_type02(page_node) {
        return None;
    }

    let mut page_json = PageJson::new();
    let page_json_data = page_json.value_mut()?;

    json_html_title(page_node, page_json_data)?;

    json_html_navi(page_node, page_json_data)?;

    let parent_id = 0;
    json_html_ul(page_node, &mut page_json, &parent_id);

    subsections_contents_set(page_node, &mut page_json);

    Some(page_json.value_take()?)
}

/// Return if <ul class="listItemBase"> is contained in page_node
fn is_type02(page_node: &Rc<Node>) -> bool {
    let attrs = &vec![("class", "listItemBase")];
    let ptn = dom_utility::node_element("ul", attrs);
    dom_utility::child_match_first(&page_node, &ptn, true).is_some()
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

    // <div class="naviBase"><span class="navi">
    let attrs = &vec![("class", "naviBase")];
    let ptn = dom_utility::node_element("div", attrs);
    let div = dom_utility::child_match_first(&page_node, &ptn, true)?;

    // <a class="naviAnchor" href="./../../../wc_top.html">Top</a>
    let ptn = dom_utility::node_element("a", &vec![]);
    let a_list = dom_utility::child_match_list(&div, &ptn, true, false);

    let mut navi_list = vec![];
    for a_node in a_list {
        let (href, text) = a_node_href_title(&a_node);
        navi_list.push((href, text));
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
    // the firstest ul
    // It may be <ul class="listItemBase">, but mo matter what class is.
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
    let div_list = dom_utility::child_match_list(&body_node, &ptn, true, false);

    for subsection_node in div_list {
        subsection_contents_set(&subsection_node, page_json);
    }

    Some(())
}

fn subsection_contents_set(subsection_node: &Rc<Node>, page_json: &mut PageJson) -> Option<()> {
    // name(key: id)
    // <div class="subsection" id="install">
    let mut name = subsection_name(&subsection_node)?;

    if name.len() == 0 {
        return None;
    }

    // subsection sample mignt be in the page.
    if name == "subsection_template" {
        return None;
    }

    // # + subsection_name
    name.insert_str(0, "#");
    // let mut subsection = page_json.subsection_by_name(&name)?;

    // Get subsection defined in json_html_ul referring by name.
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
    // <div class="subsectionContent">
    let attrs = &vec![("class", "subsectionContent")];
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

/// content: <div class="textContent">
/// class: "htmlContent" / "textContent" / "scriptSample"
/// type: script, text, html
fn subsection_content_set(subsection_content: &mut json::JsonValue, content: &Rc<Node>) {
    // <div class="textContent">
    // class:
    // <option value="htmlContent">HTML</option>
    // <option value="textContent">Text</option>
    // <option value="scriptSample">Script</option>

    if content.children.borrow().len() == 0 {
        return;
    }

    // class: "htmlContent" / "textContent" / "scriptSample"
    // <div class="textContent">abc<br>def<br>hij<br>klm</div>
    let mut content_type = None;
    match &content.data {
        NodeData::Element { attrs, .. } => {
            for attr in attrs.borrow().iter() {
                // DBG
                // info!("attr: {:?}", attr);

                if *attr.name.local == *"class" {
                    content_type.replace(attr.value.to_string());
                    break;
                }
            }
        }
        _ => (),
    }

    let content_type_str = content_type
        .as_ref()
        .and_then(|v| Some(v.as_str()))
        .or(Some(""))
        .unwrap();

    if "htmlContent" == content_type_str {
        let mut content_str = String::new();
        for content_child in content.children.borrow().iter() {
            let sh = SerializableHandle::from(content_child.clone());
            let mut bytes = vec![];
            let r = html5ever::serialize(&mut bytes, &sh, SerializeOpts::default());
            if r.is_err() {
                continue;
            }
            if let Ok(v) = String::from_utf8(bytes) {
                // let value = text_contents_gt_lt_escape(&v);
                content_str.push_str(v.as_str());
            }
        }
        let _ =
            subsection_content.push(json::object! {"type": "text", "value": content_str.as_str()});
        return;
    }

    let mut content_str = String::new();
    for content_child in content.children.borrow().iter() {
        if let NodeData::Text { contents } = &content_child.data {
            // text in textContent is in html style those does not contain any < or > without escaping (\>, \>).
            content_str.push_str(contents.borrow().to_string().as_str());
            continue;
        }

        // <br> as \n
        if let NodeData::Element { name, .. } = &content_child.data {
            if name.local.as_ref() == "br" {
                content_str.push_str("\n");
            }
            continue;
        }

        // others handle html as plain text
        let sh = SerializableHandle::from(content_child.clone());
        let mut bytes = vec![];
        let r = html5ever::serialize(&mut bytes, &sh, SerializeOpts::default());
        if r.is_err() {
            continue;
        }
        if let Ok(v) = String::from_utf8(bytes) {
            let value = text_contents_gt_lt_escape(&v);
            content_str.push_str(value.as_str());
        }
    }

    let content_node = if "textContent" == content_type_str {
        json::object! {"type": "text", "value": content_str.as_str()}
    } else {
        // if "scriptSample" == content_type_str {}
        json::object! {"type": "script", "value": content_str.as_str()}
    };
    let _ = subsection_content.push(content_node);
}

/// < to \<, > to \>
fn text_contents_gt_lt_escape(contents: &str) -> String {
    let value = contents.to_string();
    let value = value.replace("<", "\\<");
    let value = value.replace(">", "\\>");
    value
}

/// Return subsection name (key)
/// <div class="subsection" id="install">
fn subsection_name(subsection_node: &Rc<Node>) -> Option<String> {
    if let NodeData::Element { attrs, .. } = &subsection_node.data {
        for attr in attrs.borrow().iter() {
            if &attr.name.local == "id" {
                let value = String::from(&attr.value);
                return Some(value);
            }
        }
    }

    // id not found
    None
}
