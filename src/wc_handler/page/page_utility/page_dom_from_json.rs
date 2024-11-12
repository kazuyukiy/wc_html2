use markup5ever_rcdom::{Node, RcDom}; // Handle, , NodeData, SerializableHandle
use std::collections::HashSet;
use std::rc::Rc;
use tracing::{error, info}; // ,, warn

pub fn page_dom_from_json(page_json: &json::JsonValue) -> Result<RcDom, String> {
    // let html_plain = super::page_html_plain();

    // DBG
    // info!("fn page_dom_from_json");

    // let page_dom = super::to_dom(html_plain());
    let page_dom = super::to_dom(super::page_html_plain());

    page_title_set(&page_dom, page_json);

    page_json_set(&page_dom, page_json)?;

    // DBG
    // return Ok(page_dom);

    // let _r = page_html_static_set(&page_dom, page_json);
    if let Err(e) = page_html_static_set(&page_dom, page_json) {
        error!("Failed to set html_static, {}", e,);
    };

    Ok(page_dom)
}

// fn html_plain() -> &'static str {
//     r#"<html>
// <head>
// <title></title><meta charset="UTF-8">
// <script src="/wc.js"></script>
// <link rel="stylesheet" href="/wc.css"></link>
// <style type="text/css"></style>
// </head>
// <body onload="bodyOnload()">
// <span id="page_json_str" style="display: none"></span>
// </body></html>"#
// }

// fn _html_plain_() -> &'static str {
//     r#"<html>
// <head>
// <title></title><meta charset="UTF-8"></head>
// <script src="/wc.js"></script>
// <link rel="stylesheet" href="/wc.css"></link>
// <style type="text/css"></style>
// <body onload="bodyOnload()">
// <span id="page_json_str" style="display: none"></span>
// </body></html>"#
// }

// Get page title from page_json and
// set it to page_dom.
fn page_title_set(page_dom: &RcDom, page_json: &json::JsonValue) {
    // Get page title or "".
    let title_str = page_json["data"]["page"]["title"]
        .as_str()
        .or(Some(""))
        .unwrap();

    let title_ptn = super::dom_utility::node_element("title", &vec![]);
    if let Some(title_node) =
        super::dom_utility::child_match_first(&page_dom.document, &title_ptn, true)
    {
        let title_text = super::dom_utility::node_text(title_str);
        title_node.children.borrow_mut().push(title_text);
    }
}

// Set page_json value into a span.
fn page_json_set(page_dom: &RcDom, page_json: &json::JsonValue) -> Result<(), String> {
    let attrs = &vec![("id", "page_json_str")];
    let span_ptn = super::dom_utility::node_element("span", &attrs);

    if let Some(span_node) =
        super::dom_utility::child_match_first(&page_dom.document, &span_ptn, true)
    {
        let json_str = page_json.dump();
        let json_node_text = super::dom_utility::node_text(&json_str);
        span_node.children.borrow_mut().push(json_node_text);
    }

    Ok(())
}

fn page_html_static_set(page_dom: &RcDom, page_json: &json::JsonValue) -> Result<(), String> {
    let body_ptn = super::dom_utility::node_element("body", &vec![]);

    let body_node = super::dom_utility::child_match_first(&page_dom.document, &body_ptn, true)
        .ok_or("Failedto get body element".to_string())?;

    // top_node <div id="page_static">
    let attrs = &vec![("id", "page_top_node")];
    let top_node = super::dom_utility::node_element("div", attrs);

    let title_text = super::dom_utility::node_text("page_html_static_set");
    top_node.children.borrow_mut().push(title_text);

    // navi
    let navi_node = navi(page_json)?;
    top_node.children.borrow_mut().push(navi_node);

    // index
    let index_node = index(page_json)?;
    top_node.children.borrow_mut().push(index_node);

    // subsection
    let subsections_node = super::dom_utility::node_element("div", &vec![]);

    let subsections_json = &page_json["data"]["subsection"]["data"];
    if subsections_json.is_null() {
        return Err("Failed to get subsection data!".to_string());
    }

    // Check dublication of parent_index_key to avoid endlessloop.
    let mut parent_handled = HashSet::new();
    subsections(subsections_json, &subsections_node, &mut parent_handled, &0)?;
    top_node.children.borrow_mut().push(subsections_node);

    // temporary space
    let div_space = super::dom_utility::node_element("div", &vec![]);
    let title_text = super::dom_utility::node_text("----");
    div_space.children.borrow_mut().push(title_text);
    top_node.children.borrow_mut().push(div_space);

    body_node.children.borrow_mut().push(top_node);

    Ok(())
}

fn navi(page_json: &json::JsonValue) -> Result<Rc<Node>, String> {
    // let attrs = &vec![("class", "abc")];
    let ele_top = super::dom_utility::node_element("div", &vec![]);

    // info!("navi.len: {}", page_json["data"]["navi"].len());
    // page_json["navi"].len();

    let navis_json = match page_json["data"]["navi"] {
        json::JsonValue::Array(ref vec) => vec,
        _ => {
            return Err("Failed to get navi data from page_json".to_string());
        }
    };

    let mut navi_json_iter = navis_json.iter();
    loop {
        let navi_item_json = match navi_json_iter.next() {
            Some(v) => v,
            None => break,
        };

        let title = navi_item_json[0].as_str().or(Some("no title")).unwrap();

        let navi_item = if 0 < navi_json_iter.size_hint().0 {
            let href = navi_item_json[1].as_str().or(Some("")).unwrap();
            let attrs = &vec![("href", href)];
            let navi_item = super::dom_utility::node_element("a", attrs);
            let title_text = super::dom_utility::node_text(title);
            navi_item.children.borrow_mut().push(title_text);
            navi_item
        } else {
            super::dom_utility::node_text(title)
        };

        ele_top.children.borrow_mut().push(navi_item);

        if navi_json_iter.size_hint().0 < 1 {
            continue;
        }

        let delimiter_text = super::dom_utility::node_text(" > ");
        ele_top.children.borrow_mut().push(delimiter_text);
    }

    Ok(ele_top)
}

fn index(page_json: &json::JsonValue) -> Result<Rc<Node>, String> {
    let subsections_json = &page_json["data"]["subsection"]["data"];
    if subsections_json.is_null() {
        return Err("Failed to get subsection data!".to_string());
    }

    // ul / li
    let index_ul = index_ul(subsections_json, &0)?;
    Ok(index_ul)
}

// index_key: usize(number)
// Now, index type is string "0":page_json["data"]["subsection"]["data"]["0"]
// but children index type is number :page_json["data"]["subsection"]["data"]["0"]["child"] = [1, 3, 4];
// It is better all those to be numbers in future.
//
fn index_ul(subsections_json: &json::JsonValue, index_key: &usize) -> Result<Rc<Node>, String> {
    let children_i = subsection_children_indexes(subsections_json, index_key)?;

    let ul = super::dom_utility::node_element("ul", &vec![]);

    for child_i in children_i.iter() {
        let li = index_li(subsections_json, child_i)?;
        ul.children.borrow_mut().push(li);
    }
    Ok(ul)
}

fn subsection_children_indexes(
    subsections_json: &json::JsonValue,
    index_key: &usize,
) -> Result<Vec<usize>, String> {
    let key = index_key.to_string();
    let parent_json = &subsections_json[&key];
    if parent_json.is_null() {
        return Err(format!("Failed to get parent subsection: {}", index_key));
    }

    let mut children_i2 = vec![];

    let children_i = match &parent_json["child"] {
        json::JsonValue::Array(ref vec) => vec,
        // case parent_json["child"] is not defined
        _ => {
            return Ok(children_i2);
        }
    };

    for child_i in children_i.iter() {
        let i = if child_i.is_number() {
            child_i.as_usize().unwrap()
        } else if child_i.is_string() {
            child_i.as_usize().unwrap()
        } else {
            return Err(format!("Failed to get number of: {}", child_i));
        };

        children_i2.push(i);
    }

    Ok(children_i2)
}

fn index_li(subsections_json: &json::JsonValue, child_i: &usize) -> Result<Rc<Node>, String> {
    let subsection_json = &subsections_json[child_i.to_string().as_str()];

    let li = super::dom_utility::node_element("li", &vec![]);
    let href = match subsection_json["href"].as_str() {
        Some(v) => v,
        None => return Err(format!("Failed to get subsection href of : {}", child_i)),
    };
    let attrs = &vec![("href", href)];
    let a_node = super::dom_utility::node_element("a", attrs);

    let title = match subsection_json["title"].as_str() {
        Some(v) => v,
        None => return Err(format!("Failed to get subsection title of : {}", child_i)),
    };
    let title_node = super::dom_utility::node_text(title);
    a_node.children.borrow_mut().push(title_node);

    li.children.borrow_mut().push(a_node);

    let children_ul = index_ul(subsections_json, child_i)?;
    li.children.borrow_mut().push(children_ul);

    Ok(li)
}

fn subsections(
    subsections_json: &json::JsonValue,
    subsections_node: &Rc<Node>,
    parent_handled: &mut HashSet<usize>,
    parent_index_key: &usize,
) -> Result<(), String> {
    if parent_handled.contains(parent_index_key) {
        info!("Key duplicated {}", parent_index_key);
        return Ok(());
    }
    parent_handled.insert(*parent_index_key);

    subsections_node.children.borrow_mut().push(navi_back());
    let subsection_children_i = subsection_children_indexes(subsections_json, &parent_index_key)?;
    for child_i in subsection_children_i.iter() {
        subsection(subsections_json, &subsections_node, parent_handled, child_i)?;
    }

    Ok(())
}

/// If href starts with #, returns Ok(None). It is not error but not content of the page.
fn subsection(
    subsections_json: &json::JsonValue,
    subsections_node: &Rc<Node>,
    parent_handled: &mut HashSet<usize>,
    id: &usize,
) -> Result<(), String> {
    let subsection_json = &subsections_json[id.to_string().as_str()];

    // href not starts with #
    if !subsection_start_with_sharp(subsection_json, id)? {
        return Ok(());
    }

    // top element and id
    let id_str = id.to_string();
    let attrs = &vec![("id", id_str.as_str())];
    let subsection_node = super::dom_utility::node_element("div", attrs);

    // title
    let attrs = &vec![("class", "subsectionTitle")];
    let title_node = super::dom_utility::node_element("div", attrs);
    let title_str = subsection_json["title"]
        .as_str()
        .ok_or(format!("Failed to get title of index: {}", id))?;
    let title_text = super::dom_utility::node_text(title_str);
    title_node.children.borrow_mut().push(title_text);
    subsection_node.children.borrow_mut().push(title_node);

    // contents
    let contents_node = super::dom_utility::node_element("div", &vec![]);

    // "content" : [ {"type" : "text", "value" : "sample"} ],
    let contents_json = match subsection_json["content"] {
        json::JsonValue::Array(ref v) => v,
        _ => {
            return Err(format!(
                "Failed to get contents from subsection data of id: {}",
                id
            ));
        }
    };

    for content_json in contents_json {
        let content_node = content(&content_json)?;
        contents_node.children.borrow_mut().push(content_node);
    }

    subsection_node.children.borrow_mut().push(contents_node);

    subsections_node.children.borrow_mut().push(subsection_node);

    // children
    subsections(subsections_json, &subsections_node, parent_handled, &id)?;

    return Ok(());
}

fn subsection_start_with_sharp(
    subsection_json: &json::JsonValue,
    id: &usize,
) -> Result<bool, String> {
    let href = subsection_json["href"].as_str();
    if href.is_none() {
        return Err(format!("Failed to get href on id: {}", id));
    }

    let href = href.unwrap();

    if href.starts_with("#") {
        return Ok(true);
    }

    Ok(false)
}

fn content(content_json: &json::JsonValue) -> Result<Rc<Node>, String> {
    let content_value = content_json["value"]
        .as_str()
        .ok_or("Failed to get content value".to_string())?;

    // &content_json["type"]: "html", "script", "text"
    if &content_json["type"] == "text" {
        return content_text(&content_value);
    }

    if &content_json["type"] == "script" {
        return content_script(&content_value);
    }

    if content_json["type"] == "html" {
        return content_html(&content_value);
    }

    Err("type does not match".to_string())
}

fn content_html(content_value: &str) -> Result<Rc<Node>, String> {
    let attrs = &vec![("class", "html subsectionContent")];
    let content_node = super::dom_utility::node_element("div", &attrs);

    let value_doms = super::to_dom_parts(content_value);
    // to_dom_parts always returns in vec even one node.
    for value_dom in value_doms.into_iter() {
        content_node.children.borrow_mut().push(value_dom);
    }
    Ok(content_node)
}

fn content_script(content_value: &str) -> Result<Rc<Node>, String> {
    // top node
    let attrs = &vec![("class", "html subsectionContent")];
    let content_node = super::dom_utility::node_element("div", &attrs);

    // script top node
    let attrs = &vec![("class", "script")];
    let script_node = super::dom_utility::node_element("div", &attrs);

    // to_dom_parts always returns in vec even one node.
    let content_value = text_in_html(content_value);

    let value_doms = super::to_dom_parts(content_value.as_str());
    for value_dom in value_doms.into_iter() {
        script_node.children.borrow_mut().push(value_dom);
    }

    content_node.children.borrow_mut().push(script_node);

    Ok(content_node)
}

/// Show the data as text.
/// But it handle <> as html markers because it is usefull to contain html in the text.
/// So to show '<' and '>' in text, user \ to escape html markers as "\<", "\>".
fn content_text(content_value: &str) -> Result<Rc<Node>, String> {
    // top node
    let attrs = &vec![("class", "html subsectionContent")];
    let content_node = super::dom_utility::node_element("div", &attrs);

    let content_value = text_in_html(content_value);

    let value_doms = super::to_dom_parts(content_value.as_str());
    // to_dom_parts always returns in vec even one node.
    for value_dom in value_doms.into_iter() {
        content_node.children.borrow_mut().push(value_dom);
    }

    Ok(content_node)
}

// Convert text data to be shown in html.
fn text_in_html(value: &str) -> String {
    // Convert \< or \> to &lt;, &gt;
    // But escaped by \ eg: \\<, \\\\<, those are not for converting.
    let value = text_angle_entity(value);

    // Cnvert space and tab to <pre class="inline0">space and tab</pre>
    let value = text_whitespace_pre(&value);

    // no need?
    // Remove \n between > and < eg: <...>\n</...> to <...></...>
    // let value = text_remove_newline_between_elements(&value);

    // Convert \n to <br>, Split by \n\n into <p>content</p>
    let value = text_spread_parts(&value);

    value
}

fn text_angle_entity(content_value: &str) -> String {
    let mut content_value2 = String::new();

    let mut i_pos = 0;
    loop {
        if content_value.len() <= i_pos {
            break;
        }

        let (bs_pos, angle_c) = match pos_esc_angle(content_value, i_pos) {
            Some(v) => v,
            None => {
                // all remains
                content_value2 = content_value2 + &content_value[i_pos..];
                break;
            }
        };

        // info!("bs_pos: {}, angle_c: {}", bs_pos, angle_c);

        let angle_entity = if angle_c == "\\<" { "&lt;" } else { "&gt;" };

        content_value2 = content_value2 + &content_value[i_pos..bs_pos] + angle_entity;

        // info!("angle_c.len(): {}", angle_c.len());

        // i_pos = bs_pos + 4; // &lt; 4 charantors
        // i_pos = bs_pos + 2; // // i_pos based on content_value // &lt; 4 charantors
        // i_pos = bs_pos + angle_c.len(); // // i_pos based on content_value // &lt; 4 charantors

        //
        i_pos = match bs_pos.checked_add(angle_c.len()) {
            Some(v) => v,
            None => break,
        };
    }

    content_value2
}

/// Capture escaped angle bracket.
/// Returns the position of backslash just before angle bracket in Some,
/// otherwize None.
/// hay: object to be captured
/// start: start position of hay for capturing
///
fn pos_esc_angle(hay: &str, start: usize) -> Option<(usize, &str)> {
    // Charactor '\' should be escaped by \,
    // sothat it is not for controle charactors.
    // reg: (\*)(\[<>]) if not escaped by \
    let reg = regex::Regex::new(r#"(\\*)(\\[<>])"#).unwrap();

    // info!("fn pos_esc_angle");
    // info!("hay: {}", hay);
    // info!("start: {}", start);

    // info!("slice: {}", &hay[start..]);
    // &hay[start..];

    // Return None if not match.
    let caps = reg.captures(&hay[start..])?;

    // get(2) : (<|>) that must exists if caps is some.
    let angle_match = caps.get(2)?;

    // get(1) : (\\*) that should exists since * means zero or more
    let esc_match = caps.get(1)?;

    // divide number of backslash by 2
    let rem = esc_match.as_str().len().checked_rem(2)?;
    // even: backslash before \[<>] is not escaping the backslash of \[<>]
    if rem == 0 {
        Some((angle_match.start() + start, angle_match.as_str()))
    }
    // odd: backslash before \[<>] is escaping the backslash of \[<>].
    else {
        None
    }
}

/// Cnvert space and tab to <pre class="inline0">space and tab</pre>
fn text_whitespace_pre(hay: &str) -> String {
    // let reg = regex::Regex::new(r#"[ \t]{2,}|\t"#).unwrap();
    let reg = regex::Regex::new(r#"[ \t]{2,}|\t"#).unwrap();
    let hay = reg.replace_all(hay, r#"<span style="white-space: pre">$0</span>"#);
    hay.to_string()
}

// wondering if this is necessary?
/// Remove spaces between > and < if there are only spaces.
/// eg: <...>\n</...> to <...></...>
// fn text_remove_newline_between_elements(hay: &str) -> String {
//     // whitespace: \s
//     let reg = regex::Regex::new(r#">\s+<"#).unwrap();
//     let hay = reg.replace_all(hay, r#"><"#);

//     // DBG temp
//     "".to_string()

//     //
// }

fn navi_back() -> Rc<Node> {
    let navi_back_node = super::dom_utility::node_element("div", &vec![]);

    // back
    let attrs = &vec![("href", "javascript:history.back()")];
    let back_node = super::dom_utility::node_element("a", &attrs);
    let back_text = super::dom_utility::node_text("back");
    back_node.children.borrow_mut().push(back_text);
    navi_back_node.children.borrow_mut().push(back_node);

    // space
    let space_text = super::dom_utility::node_text(" ");
    navi_back_node.children.borrow_mut().push(space_text);

    // top
    let attrs = &vec![("href", "#")];
    let top_node = super::dom_utility::node_element("a", &attrs);
    let top_text = super::dom_utility::node_text("top");
    top_node.children.borrow_mut().push(top_text);
    navi_back_node.children.borrow_mut().push(top_node);

    navi_back_node
}

fn text_spread_parts(content_value: &str) -> String {
    if content_value.len() == 0 {
        return "".to_string();
    }

    // Starts with "<p>".
    let mut html = String::from("<p>");
    let mut new_paragraph = true;

    for line in content_value.split("\n") {
        // add <br> as an end of previous line
        // except beggining of paragraph
        if !new_paragraph {
            // new_paragraph = false;
            html = html + "<br>";
        }

        // case \n\n,
        // eg: xxx\n(previous line) +
        //     \n(this line, only \n without any content)
        if line.len() == 0 {
            html = html + "</p><p>";
            new_paragraph = true;
            continue;
        }
        // else
        new_paragraph = false;

        // <br> is added in beginning of the next loop
        html = html + line;
    }

    // close p element. And return it.
    html + "</p>"
}
