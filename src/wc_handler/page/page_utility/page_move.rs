// use super::page_from_json;
use super::page_json;
use super::Page;
use std::collections::HashMap;
use tracing::{error, info}; // {event, info, instrument, span, Level, Node}

/// Move org_page to dest_url as a child of dest_parent_url.
/// dest_parent_url can be None in a case dest_url is the top page.
pub fn page_move(
    stor_root: &str,
    org_url: &url::Url,
    dest_url: url::Url,
    dest_parent_url: Option<&url::Url>,
) -> Result<(), String> {
    let mut org_page = Page::new(stor_root, org_url.path());

    let mut dest_parent_page = dest_parent_url.and_then(|url| {
        let dest_parent_page = Page::new(stor_root, url.path());
        Some(dest_parent_page)
    });

    let dest_parent_page_json = match dest_parent_page.as_mut() {
        Some(page) => page.json(),
        None => None,
    };

    let dest_parent_json = dest_parent_page_json.and_then(|page_json| page_json.value());

    let mut page_moving = PageMoving::new();

    let org_json = org_page
        .json()
        .and_then(|page_json| page_json.value())
        .ok_or("Failed to get page_Json.")?;

    page_move_json(
        &mut page_moving,
        stor_root,
        org_json,
        org_url,
        &dest_url,
        dest_parent_url,
        dest_parent_json,
    )?;

    dest_page_save(stor_root, &page_moving);
    org_page_save(stor_root, &page_moving);

    Ok(())
}

/// Data about page moving
/// data: HashMap avoids same file handling and endless loop.
struct PageMoving {
    // page order to handle
    org_path_list: Vec<String>,
    // page data key: org_url.path()
    // <org_path, (org_url, dest_url, dest_json)>
    data: HashMap<String, (url::Url, url::Url, json::JsonValue)>,
}

impl PageMoving {
    fn new() -> PageMoving {
        PageMoving {
            org_path_list: vec![],
            data: HashMap::new(),
        }
    }

    fn insert(
        &mut self,
        org_url: url::Url,
        dest_url: url::Url,
        json: json::JsonValue,
    ) -> Result<(), String> {
        // Use key of url.path().
        // url.as_str() starts http or https those become different keys.
        // if self.data.contains_key(org_url.as_str()) {
        if self.data.contains_key(org_url.path()) {
            return Err(format!("org_url recurred: {}", org_url.path()));
        }

        self.org_path_list.push(org_url.path().to_string());
        self.data
            .insert(org_url.path().to_string(), (org_url, dest_url, json));

        Ok(())
    }

    fn contains_org_url(&self, org_url: &url::Url) -> bool {
        self.data.contains_key(org_url.path())
    }

    fn get(&self, org_path: &str) -> Option<(url::Url, url::Url, &json::JsonValue)> {
        if let Some((org_url, dest_url, dest_json)) = self.data.get(org_path) {
            return Some((org_url.clone(), dest_url.clone(), dest_json));
        };

        None
    }

    fn org_path_list(&self) -> Vec<&str> {
        self.org_path_list
            .iter()
            .map(|path| path.as_str())
            .collect()
    }
}

fn page_move_json(
    page_moving: &mut PageMoving,
    stor_root: &str,
    org_json: &json::JsonValue,
    org_url: &url::Url,
    dest_url: &url::Url,
    dest_parent_url: Option<&url::Url>,
    dest_parent_json: Option<&json::JsonValue>,
) -> Result<(), String> {
    info!("\n org_url: {} to\ndest_url: {}", org_url, dest_url);

    // org_url duplication avoiding endlessloop
    if page_moving.contains_org_url(org_url) {
        return Err(format!("Duplicated org_url: {}", org_url.as_str()));
    }

    // Already moved page since "moved_to" is not empty
    if !org_json["data"]["page"]["moved_to"].is_empty() {
        return Err(format!("Already moved : {}", org_url));
    }

    page_move_dest_already_data(stor_root, dest_url)?;

    // let mut dest_json = super::page_json::page_json_plain();
    let mut dest_json = page_json::page_json_plain();

    page_move_system_and(&mut dest_json, &org_json);

    page_move_navi(&mut dest_json, dest_parent_url, dest_parent_json);

    let org_children_href = page_move_subsections(&mut dest_json, org_json, &org_url)?;

    // Not save dest_json in a file at herer,
    // just save the data in page_moving, and save it later.
    page_moving.insert(org_url.clone(), dest_url.clone(), dest_json.clone())?;

    page_move_children(
        page_moving,
        stor_root,
        org_children_href,
        org_url,
        dest_url,
        &dest_json,
    )?;

    Ok(())
}

/// Return Err if page in dest_url has subsection data.
fn page_move_dest_already_data(
    // a_page: &mut Page,
    stor_root: &str,
    dest_url: &url::Url,
) -> Result<(), String> {
    // Return Err if dest_page already exists, except no data in the page.
    let mut dest_page = Page::new(stor_root, dest_url.path());
    // Case the page already has subsection data, abort moving.
    // It consider if no subsecitons data, it can be over written.
    if dest_page.json_subsections_data_exists() {
        return Err(format!("The file data already exists: {}", dest_url.path()));
    }
    return Ok(());
}

fn page_move_system_and(dest_json: &mut json::JsonValue, org_json: &json::JsonValue) {
    dest_json["system"] = org_json["system"].clone();
    dest_json["data"]["page"] = org_json["data"]["page"].clone();
}

/// Set page title in dest_json["data"]["page"]["title"] before call this.
fn page_move_navi(
    dest_json: &mut json::JsonValue,
    dest_parent_url: Option<&url::Url>,
    dest_parent_json: Option<&json::JsonValue>,
) {
    // Get title at here before calling page_move_navi_parent
    // to avoid a borrow err.
    let title = dest_json["data"]["page"]["title"]
        .as_str()
        .or(Some("no title"))
        .unwrap()
        .to_string();

    let dest_navi = &mut dest_json["data"]["navi"];

    // navi to the parent
    page_move_navi_parent(dest_parent_url, dest_parent_json, dest_navi);

    // navi to this page
    let title = json::JsonValue::from(title);
    let href = json::JsonValue::from("");
    let navi = json::array![title, href];
    let _ = dest_navi.push(navi);
}

fn page_move_navi_parent(
    dest_parent_url: Option<&url::Url>,
    dest_parent_json: Option<&json::JsonValue>,
    dest_navi: &mut json::JsonValue,
) -> Option<()> {
    let dest_parent_url = dest_parent_url?;
    let dest_parent_navi = match dest_parent_json?["data"]["navi"] {
        json::JsonValue::Array(ref vec) => Some(vec),
        _ => None,
    }?;

    for p_navi in dest_parent_navi.iter() {
        let title = p_navi[0].as_str().or(Some("no title")).unwrap();
        let title = json::JsonValue::from(title);

        let href = p_navi[1].as_str().or(Some(""))?;
        // href based on org_parent_url
        // href: String of url.path()
        let href = dest_parent_url
            .join(href)
            .and_then(|url| Ok(url.path().to_string()))
            // "" if failed.
            .or::<Result<&str, ()>>(Ok("".to_string()))
            .unwrap();
        let href = json::JsonValue::from(href);
        let navi = json::array![title, href];
        let _ = dest_navi.push(navi);
    }

    Some(())
}

///
fn page_move_subsections(
    // dest_url: &url::Url,
    dest_json: &mut json::JsonValue,
    org_json: &json::JsonValue,
    org_url: &url::Url,
) -> Result<Vec<String>, String> {
    let subsections = match &org_json["data"]["subsection"]["data"] {
        json::JsonValue::Object(ref object) => Some(object),
        _ => None,
    }
    .ok_or("Failed to get subsection data".to_string())?;

    let mut dest_subsections = json::object! {};
    let mut children_href: Vec<String> = vec![];

    for (id, org_subsection) in subsections.iter() {
        dest_subsections[id] = page_move_subsection(org_url, org_subsection, &mut children_href)?;
    }

    dest_json["data"]["subsection"]["data"] = dest_subsections;

    Ok(children_href)
}

/// Create subsection in Json converting href values based on dest_url.
/// Push href into children_href if the href is a link to a child of org_url.
fn page_move_subsection(
    // dest_url: &url::Url,
    org_url: &url::Url,
    org_subsection: &json::JsonValue,
    children_href: &mut Vec<String>,
) -> Result<json::JsonValue, String> {
    let mut dest_subsection = json::object! {};
    page_move_subsection_title_and(org_subsection, &mut dest_subsection);
    let org_href = org_subsection["href"].as_str().or(Some("")).unwrap();
    if let Some((dest_href, is_child)) = super::href_on(org_url, org_href) {
        dest_subsection["href"] = dest_href.as_str().into();
        if is_child {
            // In case a child dest_href is relative and
            // can be used for org_url also
            children_href.push(dest_href);
        }
    };
    dest_subsection["content"] = page_move_subsection_content(&org_subsection["content"], org_url)?;

    Ok(dest_subsection)
}

fn page_move_subsection_title_and(
    subsection: &json::JsonValue,
    dest_subsection: &mut json::JsonValue,
) {
    dest_subsection["parent"] = subsection["parent"].clone();
    // dest_subsection["id"] = subsection["id"].clone();
    dest_subsection["title"] = subsection["title"].clone();
    dest_subsection["child"] = subsection["child"].clone();

    // Set id as str, converting number to str.
    let id_str = match subsection["id"] {
        json::JsonValue::Number(number) => {
            let id: f64 = number.clone().into();
            id.to_string()
        }
        _ => subsection["id"].as_str().or(Some("")).unwrap().to_string(),
    };
    dest_subsection["id"] = id_str.into();
}

/// Premise: all urls of org_children_url are children of parent_org_url.
fn page_move_children(
    page_moving: &mut PageMoving,
    stor_root: &str,
    org_children_href: Vec<String>,
    parent_org_url: &url::Url,
    parent_dest_url: &url::Url,
    parent_dest_json: &json::JsonValue,
) -> Result<(), String> {
    for child_org_href in org_children_href {
        let (child_org_json, child_org_url, child_dest_url) = match page_move_children_prepare(
            stor_root,
            &child_org_href,
            parent_org_url,
            parent_dest_url,
        ) {
            Ok(v) => v,
            Err(e) => {
                error!("{}", e);
                continue;
            }
        };
        page_move_json(
            page_moving,
            stor_root,
            &child_org_json,
            &child_org_url,
            &child_dest_url,
            Some(parent_dest_url),
            Some(parent_dest_json),
        )?;
    }

    Ok(())
}

fn page_move_children_prepare(
    stor_root: &str,
    child_org_href: &str,
    parent_org_url: &url::Url,
    parent_dest_url: &url::Url,
) -> Result<(json::JsonValue, url::Url, url::Url), String> {
    let child_org_url = match parent_org_url.join(child_org_href) {
        Ok(v) => v,
        Err(_) => return Err(format!("Failed to get url for : {}", child_org_href)),
    };

    let mut child_org_page = Page::new(stor_root, child_org_url.path());

    // If child_prg_page does not exists, child_org_page.json returns None
    let child_org_json = match child_org_page
        .json()
        .and_then(|page_json| page_json.value())
    {
        Some(v) => v,
        None => {
            return Err(format!(
                "Failed to get page_json of {}",
                child_org_page.file_path()
            ))
        }
    };

    let child_dest_url = match parent_dest_url.join(child_org_href) {
        Ok(v) => v,
        Err(_) => {
            return Err(format!(
                "Failed to get child_dest_url of {}",
                child_org_href
            ));
        }
    };

    Ok((child_org_json.clone(), child_org_url, child_dest_url))
}

fn page_move_subsection_content(
    org_contents: &json::JsonValue,
    org_url: &url::Url,
) -> Result<json::JsonValue, String> {
    let org_contents = match org_contents {
        json::JsonValue::Array(ref v) => v,
        _ => {
            let msg = format!("Failed to get content of {} as Arrray", org_url.path());
            return Err(msg);
        }
    };

    let mut dest_contents = json::array![];
    for org_content in org_contents {
        // "content" : [ {"type" : "text", "value" : "sample"} ],
        let mut dest_content = json::object! {};
        dest_content["type"] = org_content["type"].clone();

        let org_content_value = org_content["value"].as_str().or(Some("")).unwrap();
        let dest_content_value = page_move_content_href_convert(org_content_value, org_url);
        dest_content["value"] = dest_content_value.into();

        dest_contents.push(dest_content).or_else(|e| {
            let msg = format!(
                "Failed to push content {}\n with {:?}",
                org_content_value, e
            );
            Err(msg)
        })?;
    }

    Ok(dest_contents)
}

fn dest_page_save(stor_root: &str, page_moving: &PageMoving) {
    for org_path in page_moving.org_path_list() {
        let (_org_url, dest_url, dest_json) = match page_moving.get(org_path) {
            Some(v) => v,
            None => {
                error!("{}", format!("No page2Moving for {}", org_path));
                continue;
            }
        };
        let mut dest_page = Page::from_json(stor_root, dest_url.path(), dest_json);
        if dest_page.dir_build().is_err() {
            continue;
        }

        let _r = dest_page.file_save_and_rev();
    }
}

fn org_page_save(stor_root: &str, page_moving: &PageMoving) {
    for org_path in page_moving.org_path_list() {
        let (mut org_page, org_page_json) =
            match page_org_page_moved(stor_root, org_path, &page_moving) {
                Ok(v) => v,
                Err(e) => {
                    error!("{}", e);
                    continue;
                }
            };

        if let Err(e) = org_page.json_replace_save(org_page_json) {
            error!("{}", e);
        }
    }
}

/// Set the page of org_url as moved.
fn page_org_page_moved(
    stor_root: &str,
    org_path: &str,
    page_moving: &PageMoving,
) -> Result<(Page, json::JsonValue), String> {
    let (org_url, dest_url, _dest_json) = match page_moving.get(org_path) {
        Some(v) => v,
        None => return Err(format!("No page2Moving for {}", org_path)),
    };

    let mut org_page = Page::new(stor_root, org_url.path());
    let org_json = org_page
        .json_value()
        .ok_or(format!("Failed to get page_json.data of {}", org_url))?;

    let mut org_json_uped = org_json.clone();

    // moved_to
    org_json_uped["data"]["page"]["moved_to"] = dest_url.as_str().into();

    // title
    let title = org_json["data"]["page"]["title"]
        .as_str()
        .or(Some(""))
        .unwrap();
    let title = format!("Moved({}) to {}", title, dest_url);

    // navi
    let navi = match &mut org_json_uped["data"]["navi"] {
        json::JsonValue::Array(ref mut v) => v,
        _ => return Err(format!("Failed to vet navi data of : {}", org_url)),
    };

    if navi.len() == 0 {
        return Err(format!("Failed to vet navi data of : {}", org_url));
    }

    let pos_last = navi.len() - 1;
    navi[pos_last][0] = title.into();

    Ok((org_page, org_json_uped))
}

/// convert href="xxx" in org_content by super::href_on
fn page_move_content_href_convert(org_content: &str, org_url: &url::Url) -> String {
    let mut index: usize = 0;
    let mut content = String::from(org_content);

    loop {
        if content.len() <= index {
            break;
        }

        // Search where href="xxx".
        let href_pos = href_pos(&content, index);
        // href not found.
        if href_pos.is_none() {
            break;
        }
        let (href_start, href_end, href_value_start, href_value_end) = href_pos.unwrap();

        // Convert href value for moving.
        let org_href = &content[href_value_start..href_value_end];
        let op_href_move = super::href_on(org_url, org_href);

        // Failed to convert href valuye.
        // Leave the href="xxx" as it is.
        // Keep the loop from href_value_end.
        if op_href_move.is_none() {
            index = href_value_end;
            continue;
        }

        let (dest_href, _is_child) = op_href_move.unwrap();

        // make href="converted_href_value"
        // put a space before "href=".
        let dest_href_equation = " href=\"".to_string() + &dest_href + "\"";
        content.replace_range(href_start..href_end, &dest_href_equation);

        index = match href_start.checked_add(dest_href_equation.len()) {
            Some(v) => v,
            None => break,
        }
    }
    content
}

/// So to remake href, you may put space before href="yyy"
fn href_pos(str: &str, search_start: usize) -> Option<(usize, usize, usize, usize)> {
    // Search <a, but not escaped \<a:
    let (_a_start, a_end) = pos_not_escaped(str, search_start, "<a")?;

    // Search href="value"
    // href=" or href='
    // (?i) : not case-sensitive
    let reg_href = regex::Regex::new(r#"(?i)\s*href\s*=\s*["']"#).unwrap();
    let href_mat = reg_href.find(&str[a_end..])?;
    // begining point of href=""
    // starts a_end so add it to the result
    let href_start = a_end.checked_add(href_mat.start())?;
    // position of the first quote charactor
    // starts a_end so add it to the result
    let q1_end = a_end.checked_add(href_mat.end())?;
    // one charactor " before
    let q1_start = q1_end.checked_add_signed(-1)?;
    // Get the first quote.
    let quote = &str[q1_start..q1_end];
    // Search second quote in after href=", q1_end position.
    let (q2_start, q2_end) = pos_not_escaped(str, q1_end, quote)?;
    let href_value_start = q1_end;
    let href_value_end = q2_start;
    let href_end = q2_end;

    // Return positions of value part: abc of href="abc"
    Some((href_start, href_end, href_value_start, href_value_end))
}

/// Search ptn on str and return the first position of it as Some(start, end),
/// or None if not found or any error.
/// It does not match if `\` is found before ptn as an escape key.
/// It start serching from search_start position of str
fn pos_not_escaped(str: &str, search_start: usize, ptn: &str) -> Option<(usize, usize)> {
    // Regular expression of ptn
    let re_ptn = regex::Regex::new(&ptn).ok()?;

    // Regular expression of backslash `\` continuing more than two.
    let re_esc = regex::Regex::new(r"(\\+)$").ok()?;

    let mut index_start = search_start;

    loop {
        // Serching reaced at the end and ptn was not found.
        if str.len() <= index_start {
            return None;
        }

        // Search ptn
        let ptn_match = re_ptn.find(&str[index_start..])?;

        // index position of ptn starts.
        let ptn_start = index_start.checked_add(ptn_match.start())?;

        // Check if the ptn is escaped.
        // To do that, count number of \ before ptn.
        //
        // \\\\ptn (\ is more than one)
        // if \ exists just befor ptn
        // it might be an escape code (\ptn)
        // or just `\` charactor (\\ptn: `\` + ptn)
        //
        // In case of single `\` charactor,
        // it should be escape code \ before `\` charactor
        // so \\ is a caractor `\` with escaped code.
        //
        // If number of continuous \ is odd, the last \ is escape code for ptn.
        // eg "\\ \\ \\ \\ \ptn" (The parrern is escaped by the last \.)
        // (consider as spaces in above are not exists, those spaces are only for easy to see.)
        //
        // If number of continuous \ is even, those are some couple of
        // escape code and it means `\` charactors.
        // eg "\\ \\ \\ \\ ptn" (The parrern is not escaped by \.)
        //
        // If make some couple of \ (\\) and still remains one \
        // it means ptn is escaped.
        // In case of html, it is not an element since < is escaped with \.
        // eg: \<a\>
        //
        // Find `\` just befor ptn position.
        // &str[index_start..ptn_start]: str just before ptn, just beforeptn_start
        let escape_cap = re_esc.captures(&str[index_start..ptn_start]);

        // Set index_start position to end of ptn.
        index_start = index_start.checked_add(ptn_match.end())?;

        if let Some(cap) = escape_cap {
            // If number of `\` is odd, ptn is escaped.
            if &cap[1].len() % 2 == 1 {
                // Search ptn again after new index_start position
                continue;
            }
        }

        let ptn_end = index_start;
        return Some((ptn_start, ptn_end));
    }
}
