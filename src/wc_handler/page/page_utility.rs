use html5ever::serialize;
use html5ever::serialize::SerializeOpts;
use markup5ever_rcdom::{Handle, NodeData, RcDom, SerializableHandle}; // , Node
use std::collections::HashSet;
use std::fs;
// use tracing::{event, info, instrument, span, Level};
use tracing::info; // event, , instrument, span, Level

// mod contents;
mod dom_utility;

// Create directories for a file_path given as a parameter.
fn dir_build(file_path: &str) -> Result<(), ()> {
    // file_path : abc/def/ghi.html
    // Remove file name and remain only directory path.
    let re = regex::Regex::new(r"/[^/]+$").unwrap();
    let mat = re.find(&file_path).unwrap();

    // dir_path abc/def
    let dir_path = &file_path[..mat.start()];

    match std::fs::DirBuilder::new().recursive(true).create(dir_path) {
        Ok(_) => Ok(()),
        Err(_) => {
            eprintln!("page_utility.rs Failed to create dir: {}", dir_path);
            Err(())
        }
    }
}

pub fn file_write(path: &str, source: &Vec<u8>) -> Result<(), ()> {
    if let Err(_) = dir_build(path) {
        return Err(());
    }

    match fs::write(&path, source) {
        Ok(_) => {
            // println!("write: {}", &path);
            // println!("write: {}", &self.path);
            Ok(())
        }
        Err(_) => {
            // eprintln!("Failed to save page: {}", &path);
            // eprintln!("Failed to save page: {}", &self.path);
            Err(())
        }
    }

    // temp
    //    Ok(())
}

pub fn to_dom(source: &str) -> RcDom {
    dom_utility::to_dom(source)
}

// Get span node from page_dom
// <span id="page_json_str" style="display: none">{"json":"json_data"}</span>
pub fn span_json_node(page_dom: &RcDom) -> Handle {
    let attrs = &vec![("id", "page_json_str")];
    let ptn_span = dom_utility::node_element("span", &attrs);
    // let ptn_span = span_json_node_ptn();
    // the <span> mut exists
    dom_utility::child_match_first(&page_dom, &ptn_span, true).unwrap()
}

pub fn json_from_dom(dom: &RcDom) -> Option<json::JsonValue> {
    // span node containing json data in text
    let span = span_json_node(dom);
    let children = span.children.borrow();
    if children.len() == 0 {
        eprintln!("Failed, json contents not found in the span element");
        return None;
    }

    let contents = match &children[0].data {
        NodeData::Text { contents } => contents,
        // _ => return Err(err_no_json_value()),
        _ => {
            eprintln!("Failed, json contents not found in the span element");
            return None;
        }
    };

    let json_str = contents.borrow().to_string();

    let json_value = match json::parse(&json_str) {
        Ok(page_json_parse) => page_json_parse,
        Err(e) => {
            eprintln!("{:?}", e);
            return None;
        }
    };

    Some(json_value)
}

fn page_dom_plain() -> RcDom {
    to_dom(page_html_plain())
}

fn page_html_plain() -> &'static str {
    r#"<!DOCTYPE html><html><head><title></title><meta charset="UTF-8"></meta><script src="/wc.js"></script>
    <link rel="stylesheet" href="/wc.css"></link>
    <style type="text/css"></style>
</head><body onload="bodyOnload()"><span id="page_json_str" style="display: none"></span></body></html>
"#
}

/// Create a page source from json value.
fn source_from_json(page_json: json::JsonValue) -> Vec<u8> {
    let page_dom = page_dom_plain();

    // title
    if let Some(title_str) = page_json["data"]["page"]["title"].as_str() {
        let title_ptn = dom_utility::node_element("title", &vec![]);
        if let Some(title_node) = dom_utility::child_match_first(&page_dom, &title_ptn, true) {
            let title_text = dom_utility::node_text(title_str);
            title_node.children.borrow_mut().push(title_text);
        }
    }

    // put json value into span as str
    let span = span_json_node(&page_dom);
    let _ = &span.children.borrow_mut().clear();
    let json_str = json::stringify(page_json);
    let json_node_text = dom_utility::node_text(&json_str);
    let _ = &span.children.borrow_mut().push(json_node_text);

    //
    let sh = SerializableHandle::from(page_dom.document);
    let mut page_bytes = vec![];
    let _r = serialize(&mut page_bytes, &sh, SerializeOpts::default());

    page_bytes
}

/// Create `supser::Page` from page_json and return it.
/// // If a file exists, open the file and overwrite page_json.
/// // If no file exists, create a `super::Page` from page_json.
///
pub fn page_from_json(
    page_root: &str,
    path: &str,
    page_json: json::JsonValue,
) -> Result<super::Page, ()> {
    let page_bytes = source_from_json(page_json);

    let mut page = match super::Page::new(page_root, path) {
        Ok(v) => v,
        Err(_) => match super::Page::open(page_root, path) {
            Ok(v) => v,
            Err(_) => {
                eprintln!("Failed to get `Page`.");
                return Err(());
            }
        },
    };

    page.source.replace(page_bytes);

    Ok(page)
}

pub fn json_plain() -> json::JsonValue {
    // ~/projects/wc/wc/src/page_json_utility.rs
    json::object! {
        // "syttem" :
        "system" : {
            // "version" : "0.0.1",
            // "version" : "0.0.2",
            "version" : "0.0.3",
        },

        "data" : {
            "page" : {
                "title" : "",
                "rev" : 0,
                "rev_speculation" : 0,
                "group_top" : false,
        // consider to add path , uri data
            },

            "navi" : [
                /*
                {"name0" : "href0"},
                {"name1" : "href1"}
                // change to
                ["name0" , "href0"],
                ["name0" , "href0"],

                 */
            ],

            "subsection" : {
                "id" : {
                    "id_next" : 2,
                    "id_notinuse" : []
                },

                "data" : {
                    "0" : {
                        "parent" : "",
                        "id" : "0",
                        "title" : "",
                        "href" : "",
                        "content" : [],
                        "child" : []
                    }
                    ,

                    // "1" : {
                        // "parent" : "0",
                        // "id" : "1",
                        // "title" : "sample",
                        // "href" : "#sample",
                        // "content" : [ {"type" : "text", "value" : "sample"} ],
                        // "child" : []
                    // }

                },

            },

            "href" : {
                "relation" : {},
                "last" : {},
                "history" : {},
            },
        },
    }
}

/// Create a new page under the parent_page
/// It returns an instance of super::Page
/// but its file is not saved.
/// You need to save the file if need it .
///
/// It return Err if a file alraady exists in the child_href,
///
/// Create new navi data taking over parent_page and adding child_title to it
/// converting href based on child_href
///
/// parent_* : some value of parent_page
/// child_* : some value of new navi data created in this function
///
/// child_href: absolute or related location based on parent_page
///
pub fn page_sub_new(
    parent_page: &super::Page,
    child_title: &str,
    child_href: &str,
) -> Result<super::Page, ()> {
    let (child_title, child_href) = child_title_href(child_title, child_href)?;
    let (parent_url, child_url) = parent_child_url(parent_page, child_href)?;

    let child_file_path = parent_page.file_path(child_url.path());
    if let Ok(_) = fs::File::open(&child_file_path) {
        // already the file exists.
        return Err(());
    }

    // let parent_json = match parent_page.json() {
    //     Some(v) => v,
    //     None => return Err(()),
    // };

    // let parent_contents = match parent_page.contents() {
    //     Some(v) => v,
    //     None => return Err(()),
    // };
    let parent_contents = match parent_page.contents_data() {
        Some(v) => v,
        None => return Err(()),
    };

    // Create child_json
    let mut child_json = json_plain();
    // title
    child_json["data"]["page"]["title"] = child_title.into();

    // navi
    // let mut child_navi = child_navi_set(parent_url, &parent_json, &child_url)?;
    let mut child_navi = child_navi_set(parent_url, &parent_contents, &child_url)?;
    // add child page title as last navi element
    let navi_self: Vec<json::JsonValue> = vec![child_title.into(), "".into()];
    let _res = child_navi.push(json::JsonValue::Array(navi_self));
    child_json["data"]["navi"] = child_navi;

    // return Ok(Page)
    page_from_json(parent_page.page_root(), child_url.path(), child_json)
}

// Check child title and href
fn child_title_href<'a>(
    child_title: &'a str,
    child_href: &'a str,
) -> Result<(&'a str, &'a str), ()> {
    let child_title = child_title.trim();
    if child_title.len() == 0 {
        eprintln!("no child title");
        return Err(());
    }

    let child_href = child_href.trim();
    if child_href.starts_with("#") {
        eprintln!("child href starts with #");
        return Err(());
    }
    if child_href.len() == 0 {
        eprintln!("no child href");
        return Err(());
    }

    Ok((child_title, child_href))
}

fn parent_child_url<'a>(
    parent_page: &'a super::Page,
    child_href: &str,
) -> Result<(&'a url::Url, url::Url), ()> {
    let parent_url = match parent_page.url() {
        Some(v) => v,
        None => {
            eprintln!("no parent url");
            return Err(());
        }
    };

    // parent_url;
    let child_url = match parent_url.join(&child_href) {
        Ok(u) => u,
        Err(_) => {
            eprintln!("parent_url.join failed");
            return Err(());
        }
    };

    Ok((parent_url, child_url))
}

fn child_navi_set(
    parent_url: &url::Url,
    parent_json: &json::JsonValue,
    child_url: &url::Url,
) -> Result<json::JsonValue, ()> {
    let parent_navi = match &parent_json["data"]["navi"] {
        json::JsonValue::Array(ref v) => v,
        _ => return Err(()),
    };

    let mut child_navi = json::JsonValue::new_array();

    for navi in parent_navi {
        let title = match navi[0].as_str() {
            Some(v) => v,
            None => "no title",
        };

        // Convert href switching its base on paretn_url to child_url
        let href = match navi[1].as_str() {
            Some(parent_href) => match href_base_switch(&parent_url, parent_href, &child_url) {
                Some(v) => v,
                None => "".to_string(),
            },
            None => "".to_string(),
        };

        let navi2: Vec<json::JsonValue> = vec![title.into(), href.into()];
        let _ = child_navi.push(json::JsonValue::Array(navi2));
    }

    Ok(child_navi)
}

/// Returns url of fm_href switned base from fm_url to to_url.
/// If possible, returns relative url based on to_url.
fn href_base_switch(fm_url: &url::Url, fm_href: &str, to_url: &url::Url) -> Option<String> {
    // join : Get location of fm_href based on fm_url.
    match fm_url.join(&fm_href) {
        Ok(href_url) => {
            // make_relative: Get relative location of href_url based on to_Url.
            match to_url.make_relative(&href_url) {
                Some(v) => Some(v),
                // absolute
                None => Some(href_url.as_str().to_string()),
                // None => Some(href_url.path().to_string()),
            }
        }
        Err(_) => None,
    }
}

///
/// Issue: if a file already exits in the destination path.
///
/// page_moving : page to be move to.
/// dest_url: url where page_moving move to.
/// parent_url: the page page_moving becomes to be the child of.
///
/// Buffer new page and its children to be saved.
/// If an error happens, like a page where moving to already exits with contents,
/// discards the buffer.
/// If no err happens, save all page in the buffer after all pages were handled.
///
/// After move to new location, change the previous pages as discontinued.
///
pub fn page_move(
    page_moving: &super::Page,
    dest_url: url::Url,
    parent_url: Option<url::Url>,
) -> Result<(), ()> {
    // dbg
    println!("page_utility.rs fn page_move start");

    let _href_list = page_move_href_list(page_moving, page_moving.path());
    // dbg
    // for href_url in href_list {
    //     println!("href_url: {}", href_url.as_str());
    // }

    // dbg
    println!("page_utility.rs fn page_move cp0");
    // return Err(());

    // let _page_moving_json = match page_moving.json() {
    //     Some(v) => v,
    //     None => return Err(()),
    // };

    // Save backup of page_moving.
    let _ = page_moving.file_save_rev();

    let page_root = page_moving.page_root();

    // dbg
    println!("page_utility.rs fn page_move cp1");

    // If a page already exists in dest_url.path(), stop moving.
    // let mut dest_page = match dest_page(page_root, dest_url.path()) {
    let mut dest_page = match dest_page(page_moving, dest_url.path()) {
        Ok(v) => v,
        // Return err if dest_page has contents.
        Err(_) => return Err(()),
    };

    // dbg
    println!("page_utility.rs fn page_move cp2");

    // let mut parent_page = match parent_url {
    let parent_page = match parent_url {
        Some(parent_url) => match super::Page::open(page_root, parent_url.path()) {
            Ok(mut page) => {
                // page.json_set();
                page.contents_set();
                page.url_set(parent_url);
                Some(page)
            }
            Err(_) => None,
        },
        None => None,
    };

    // dbg
    println!("page_utility.rs fn page_move cp3");

    // navi
    match page_move_navi(parent_page.as_ref(), page_moving, &mut dest_page) {
        Ok(_) => {}
        Err(_) => return Err(()),
    }

    // dbg
    println!("page_utility.rs fn page_move cp4");

    // subsections
    let _res = page_move_subsections(page_moving, &mut dest_page);

    dest_page.file_save();
    dest_page.file_save_rev();

    // momorize moving info

    // save page_moving

    // ~/projects/wc/wc_html/src/page/page_utility.rs
    // edit the original page_moving
    // set navi

    // temp
    Ok(())
}

/// Returns url::Url list of all external pages that can be found in
/// json_value["data"]["subsection"]["data"][i]["href"].value
/// and those children recursively.
/// base_page is expected parent page of href link,
/// but not have to be the parent,
/// a page of href also works.
/// Because it is used to get inherited page.
fn page_move_href_list(base_page: &super::Page, page_path: &str) -> HashSet<url::Url> {
    let mut href_list: HashSet<url::Url> = HashSet::new();

    let mut page = base_page.inherited(page_path);
    page.contents_set();
    // page.json_set();

    let contents = match page.contents_data() {
        Some(v) => v,
        None => return href_list,
    };
    // let page_json = match page.json() {
    //     Some(v) => v,
    //     None => return href_list,
    // };

    // json_value["data"]["subsection"]["data"][i]["href"].value
    // let subsections = &page_json["data"]["subsection"]["data"];
    let subsections = &contents["data"]["subsection"]["data"];
    if let json::JsonValue::Object(v) = subsections {
        let page_url = match page.url() {
            Some(v) => v,
            None => return href_list,
        };

        for (_i, subsection) in v.iter() {
            let href = match subsection["href"].as_str() {
                Some(v) => v,
                None => continue,
            };

            let href_url = match page_url.join(href) {
                Ok(v) => v,
                Err(_) => continue,
            };

            // Avoides endless loop.
            // if href is #some,
            // calling page_move_href_list() causes loop forever.
            if page_url.path() == href_url.path() {
                continue;
            }

            // Find grandchild href.
            let href_list_sub = page_move_href_list(&page, &href_url.path());
            // Append children.
            for href_sub_url in href_list_sub {
                href_list.insert(href_sub_url);
            }

            // Append the child href.
            href_list.insert(href_url);
        }
    }

    href_list
}

/// Returns new super::Page instance that has no contents.
/// If the page already exists in the path and contains contents, return Err.
/// If it exists, but not contents, it returns the instance.
fn dest_page(page_moving: &super::Page, path: &str) -> Result<super::Page, ()> {
    let page_root = page_moving.page_root();
    match super::Page::open(page_root, path) {
        // Page exists.
        Ok(mut dest_page) => {
            dest_page.contents_set();
            if let Some(contents) = dest_page.contents_data() {
                // No contents, pahaps empty page or discontinued.
                // contents["data"]["subsection"]["data"][0] is not real content.
                if contents["data"]["subsection"]["data"].len() <= 1 {
                    return Ok(dest_page);
                }
            }
        }
        // Page does not exists.
        // Create a new empty instance later.
        Err(_) => (),
    }

    info!("fn dest_page creage new page");

    // Create a new super::Page instace
    let mut dest_page = match super::Page::new(page_root, path) {
        Ok(v) => v,
        Err(_) => return Err(()),
    };
    // dest_page.json_plain_set();
    info!("fn dest_page calling data_plain_set");
    // let contents = dest_page.contents();
    // info!("fn dest_page contents_mut: {:?}", contents);

    // dbg
    // match dest_page.contents_mut() {
    //     Some(_) => info!("fn dest_page contents exists"),
    //     _ => info!("fn dest_page contents Not exists"),
    // };

    dest_page
        .contents_mut()
        .map(|contents| contents.data_plain_set());

    // dest_page.contents.replace(contents::Contents::new());
    dest_page.contents_plain_set();

    Ok(dest_page)
}

// fn dest_page_(page_root: &str, path: &str) -> Result<super::Page, ()> {
//     match super::Page::open(page_root, path) {
//         // Page exists.
//         Ok(mut dest_page) => {
//             dest_page.contents_set();
//             // dest_page.json_set();

//             // if let Some(json) = dest_page.json() {
//             //     // No contents, pahaps empty page or discontinued.
//             //     if json["data"]["subsection"]["data"].len() == 0 {
//             //         return Ok(dest_page);
//             //     }
//             // }
//             if let Some(contents) = dest_page.contents_data() {
//                 // No contents, pahaps empty page or discontinued.
//                 if contents["data"]["subsection"]["data"].len() == 0 {
//                     return Ok(dest_page);
//                 }
//             }
//         }
//         // Page does not exists.
//         // Create a new empty instance later.
//         Err(_) => (),
//     }

//     // Create a new super::Page instace
//     let mut dest_page = match super::Page::new(page_root, path) {
//         Ok(v) => v,
//         Err(_) => return Err(()),
//     };
//     // dest_page.json_plain_set();
//     let _ = &dest_page
//         .contents_mut()
//         .map(|contents| contents.data_plain_set());

//     Ok(dest_page)
// }

fn page_move_navi(
    parent_page: Option<&super::Page>,
    page_moving: &super::Page,
    dest_page: &mut super::Page,
) -> Result<(), ()> {
    // span!(Level::TRACE, "fn_page_move_navi");

    info!("fn page_move_navi cp");

    // let page_moving_json = match page_moving.json() {
    //     Some(v) => v,
    //     None => return Err(()),
    // };
    let fm_contents = match dest_page.contents_data() {
        Some(v) => v,
        None => return Err(()),
    };

    info!("fn page_move_navi cp1");

    // let page_moving_navi = match &page_moving_json["data"]["navi"] {
    let page_moving_navi = match &fm_contents["data"]["navi"] {
        json::JsonValue::Array(ref v) => v,
        _ => return Err(()),
    };

    info!("fn page_move_navi cp2");

    // If page_moving has only one navi element,
    // it means the navi element is the top navi item.
    // So it does not inherit parent navi.
    //
    // If parent_page is None, no parent navi is inherited.
    //
    let mut navi = if 1 < page_moving_navi.len() && parent_page.is_some() {
        // Inherite parent page navi.
        let parent_page = parent_page.as_ref().unwrap();
        page_move_navi_inherit(parent_page, dest_page)
    } else {
        json::JsonValue::new_array()
    };

    // Push page_moving's title
    // navi_dest: the last element of navi list that is for destination page.
    //
    // let fm_json = match page_moving.json() {
    //     Some(v) => v,
    //     None => return Err(()),
    // };
    let fm_contents = match page_moving.contents_data() {
        Some(v) => v,
        None => return Err(()),
    };

    info!("fn page_move_navi cp3");

    // let title = match fm_json["data"]["page"]["title"].as_str() {
    let title = match fm_contents["data"]["page"]["title"].as_str() {
        Some(s) => s,
        None => "no title",
    };
    let navi_dest: Vec<json::JsonValue> = vec![title.into(), "".into()];
    let _ = navi.push(json::JsonValue::Array(navi_dest));

    // Put navi_dest to dest_page.
    //    page_dest.

    //
    // let dest_page_json = match dest_page.json_mut() {
    //     Some(v) => v,
    //     None => return Err(()),
    // };

    let dest_page_contents = match dest_page.contents_data_mut() {
        Some(v) => v,
        None => return Err(()),
    };

    info!("fn page_move_navi cp4");

    // dest_page_json["data"]["navi"] = navi;
    dest_page_contents["data"]["navi"] = navi;

    Ok(())
}

/// Create a navi list inheriting parent_page.
fn page_move_navi_inherit(
    parent_page: &super::Page,
    child_page: &super::Page,
    // ) -> Result<json::JsonValue, ()> {
) -> json::JsonValue {
    let mut child_navi = json::JsonValue::new_array();

    // let parent_page_json = match parent_page.json() {
    //     Some(v) => v,
    //     None => return child_navi,
    // };
    let parent_page_contents = match parent_page.contents_data() {
        Some(v) => v,
        None => return child_navi,
    };

    // let parent_navi = match &parent_page_json["data"]["navi"] {
    let parent_navi = match &parent_page_contents["data"]["navi"] {
        json::JsonValue::Array(ref v) => v,
        _ => return child_navi,
    };

    //let parent_page_url = match parent_page.url().as_ref() {
    let parent_page_url = match parent_page.url() {
        Some(v) => v,
        None => return child_navi,
    };

    // if child_page.url().is_none() { return child_navi; }
    let child_page_url = match child_page.url() {
        Some(v) => v,
        None => return child_navi,
    };

    for navi in parent_navi {
        let title = match navi[0].as_str() {
            Some(v) => v,
            None => "no title",
        };

        let href = match navi[1].as_str() {
            Some(parent_href) => {
                // Convert href that based on parent pate to based on child_page.
                match href_base_switch(&parent_page_url, parent_href, &child_page_url) {
                    Some(v) => v,
                    None => "".to_string(),
                }
            }
            None => "".to_string(),
        };

        let vec: Vec<json::JsonValue> = vec![title.into(), href.into()];
        let _res = child_navi.push(json::JsonValue::Array(vec));
    }

    child_navi
}

fn page_move_subsections(page_moving: &super::Page, dest_page: &mut super::Page) -> Result<(), ()> {
    // let dest_page_json = match dest_page.json_mut() {
    //     Some(v) => v,
    //     None => return Err(()),
    // };
    let dest_page_contents = match dest_page.contents_data() {
        Some(v) => v,
        None => return Err(()),
    };
    // let dest_subsections = match &dest_page_json["data"]["subsection"]["data"] {
    let dest_subsections = match &dest_page_contents["data"]["subsection"]["data"] {
        json::JsonValue::Object(v) => v,
        _ => return Err(()),
    };

    // ??
    let mut _dest_subsections = dest_subsections.clone();

    // let page_moving_json = match page_moving.json() {
    //     Some(v) => v,
    //     None => return Err(()),
    // };
    let page_moving_contents = match page_moving.contents_data() {
        Some(v) => v,
        None => return Err(()),
    };
    let fm_subsections = &page_moving_contents["data"]["subsection"]["data"];
    if let json::JsonValue::Object(_) = &page_moving_contents["data"]["subsection"]["data"] {
    } else {
        return Err(());
    };

    // entries: returns iterator of JsonValue::Object.
    for (_id, fm_subsection) in fm_subsections.entries() {
        let _res = page_move_subsection(page_moving, fm_subsection, dest_page);
    }

    //temp
    Ok(())
}

fn page_move_subsection(
    page_moving: &super::Page,
    fm_subsection: &json::JsonValue,
    dest_page: &mut super::Page,
) -> Result<(), ()> {
    let mut dest_subsection = json::JsonValue::new_object();

    // href
    dest_subsection["href"] = match page_move_subsection_href(page_moving, fm_subsection, dest_page)
    {
        Ok(v) => v.into(),
        Err(_) => return Err(()),
    };

    // content
    dest_subsection["content"] =
        match page_move_subsection_contents(page_moving, fm_subsection, dest_page) {
            Ok(v) => v,
            Err(_) => return Err(()),
        };

    // temp
    Err(())
}

fn page_move_href_convert() {}

fn page_move_subsection_href(
    page_moving: &super::Page,
    fm_subsection: &json::JsonValue,
    dest_page: &mut super::Page,
) -> Result<String, ()> {
    let fm_href = match fm_subsection["href"].as_str() {
        Some(v) => v,
        None => return Err(()),
    };

    let page_moving_url = match page_moving.url() {
        Some(v) => v,
        None => return Err(()),
    };

    let fm_href_url = match page_moving_url.join(fm_href) {
        Ok(v) => v,
        Err(_) => return Err(()),
    };

    // Get fm_href_relative to know if fm_href is a relation to page_moving,
    // or its children.
    let fm_href_relative = match page_moving_url.make_relative(&fm_href_url) {
        Some(v) => v,
        None => return Err(()),
    };

    let dest_page_url = match dest_page.url() {
        Some(v) => v,
        None => return Err(()),
    };

    // If href starts with "../", it relates not to page_moving or its children.
    // In this case, convert the relation based on page_moving_url to dest_page_url.
    let dest_href = if fm_href_relative.starts_with("../") {
        match href_base_switch(&page_moving_url, fm_href, &dest_page_url) {
            Some(v) => v,
            None => return Err(()),
        }
    } else {
        // fm_href_url is a link to page_moving or its children.
        // Relative href in page_moving can work at dest_page as well.
        fm_href_relative
    };

    Ok(dest_href)
}

fn page_move_subsection_contents(
    page_moving: &super::Page,
    fm_subsection: &json::JsonValue,
    dest_page: &super::Page,
) -> Result<json::JsonValue, ()> {
    //
    let fm_contents = match &fm_subsection["content"] {
        json::JsonValue::Array(v) => v,
        _ => return Err(()),
    };

    let page_moving_url = match page_moving.url() {
        Some(v) => v,
        None => return Err(()),
    };

    let dest_page_url = match dest_page.url() {
        Some(v) => v,
        None => return Err(()),
    };

    let dest_contents = json::JsonValue::new_array();

    for fm_content in fm_contents {
        let fm_value = match fm_content["value"].as_str() {
            Some(v) => v,
            None => return Err(()),
        };

        let mut dest_content = json::JsonValue::new_object();

        dest_content["value"] =
            // page_move_subsection_content_href_convert(page_moving, fm_value, dest_page).into();
            page_move_subsection_content_href_convert(fm_value, page_moving_url, dest_page_url).into();
    }

    Ok(dest_contents)
}

fn page_move_subsection_content_href_convert(
    fm_content: &str,
    page_moving_url: &url::Url,
    dest_page_url: &url::Url,
) -> String {
    // fn page_move_subsection_content_href_convert(
    //     page_moving: &super::Page,
    //     fm_content: &str,
    //     dest_page: &super::Page,
    // ) -> String {
    //
    let mut content = String::from(fm_content);

    // let page_moving_url = match page_moving.url() {
    //     Some(v) => v,
    //     None => return content,
    // };

    // let dest_page_url = match dest_page.url() {
    //     Some(v) => v,
    //     None => return content,
    // };

    // inside of content["value"]
    let mut index: usize = 0;
    loop {
        if content.len() <= index {
            break;
        }

        // Get index of href value position in content
        // href="value1"
        // Search from &str[index..]
        let (start, end) = match href_pos(&content, index) {
            Some(v) => v,
            // href value not found.
            None => break,
        };

        // next loop may start from where after the current href value position
        index = end;

        let href_value = &content[start..end];

        // Case #abc, it is local link, so nothing to do about href_value,
        // leave as it is and do further on conversion on the rest.
        if href_value.starts_with("#") {
            // index = end;
            continue;
        }

        // Convert href to based on dest_page
        match href_base_switch(&page_moving_url, href_value, &dest_page_url) {
            Some(href_converted) => {
                // let href_len = href_value.len();

                // index = end - href_value.len() + href_converted.len();

                // Compensate href value change on index.
                // It should be href_value.len() < index
                // index = index - href_value.len() + href_converted.len();
                index -= href_value.len();
                index = match index.checked_add(href_converted.len()) {
                    Some(v) => v,
                    None => break,
                };

                // replace href_value
                // start should be greater than 0,
                // because it is index after " of <a href="href_value">
                // So checked_add_singed is not required.
                // content = content[0..start - 1].to_string() + &href_converted + &content[end..];
                content = content[0..start].to_string() + &href_converted + &content[end..];

                //
            }
            // Failed to convert href
            None => {
                // ignore the href
                // index = end;
                continue;
            }
        }

        // Convert href to based on dest_page
        // let res = href_base_switch(&page_moving_url, href_value, &dest_page_url);
        // If failed to convert the href, ignore it and go on further loop.
        // if let None = res {
        //     // index = match index.checked_add(end) {
        //     //     Some(v) => v,
        //     //     None => break,
        //     // };
        //     continue;
        // }

        // res is Some
        // let href_value_converted = res.unwrap();
        // let href_len = href_value.len();
        // let href_converted_len = href_value_converted.len();

        // replace href_value
        // start should be greater than 0,
        // because it is index after " of <a href="href_value">
        // So checked_add_singed is not required.
        // content = content[0..start - 1].to_string() + &href_value_converted + &content[end..];

        // safely index = index - href_value.len() + href_value_converted.len();
        // index = index - href_len;
        // index = match index.checked_add(href_value_converted.len()) {
        //     Some(v) => v,
        //     None => break,
        // };

        // index = index + start + href_value.len();

        // match href_base_switch(&page_moving_url, href_value, &dest_page_url) {
        //     Some(v) => {
        //         //
        //         v;
        //     }
        //     None => {
        //         // use checked_add()
        //         index += end;
        //     }
        // };

        // Convert href to based on dest_page
        // let href_value = match href_base_switch(&page_moving_url, href_value, &dest_page_url) {
        //     Some(s) => s,
        //     // if not use href not converted
        //     None => href_value.to_string(),
        // };

        // get content_str that href is replaced with what based on dest_page
        // content = content[0..start - 1].to_string() + &href_value + &content[end..];
    }

    content
}

/// Find ptn from &str[search_start..] .
/// search_start: start position of str to find
/// ptn: pattern to find
/// It does not match with \ptn; escaped by \
fn find_not_escaped(str: &str, search_start: usize, ptn: &str) -> Option<(usize, usize)> {
    // Regular expression to find ptn
    let re_ptn = regex::Regex::new(&ptn).unwrap();
    // Regular expression to find backslash `\` continuing more than two.
    let re_esc = regex::Regex::new(r"(\\+)$").unwrap();

    let mut index_start = search_start;

    loop {
        if str.len() <= index_start {
            // ptn was not found
            break;
        }

        // Search ptn
        let mat = match re_ptn.find(&str[index_start..]) {
            Some(v) => v,
            // ptn was not found
            None => break,
        };

        // index position of ptn starts.
        let ptn_start = match index_start.checked_add(mat.start()) {
            Some(v) => v,
            None => break,
        };

        // Check if the ptn is escaped.
        // To do that, count number of \ before ptn.
        //
        // \\\\ptn
        // if \ exists just befor ptn
        // it might be an escape code for ptn (\ptn)
        // or just `\` charactor (\\ptn)
        //
        // In case of `\` charactor,
        // it should be escape code \ before `\` charactor
        // so \\ is a caractor `\` with escaped code.
        //
        // If number of continuous \ is odd, the last \ is escape code for ptn.
        // eg "\\ \\ \\ \\ \ptn" (The parrern is escaped by \.)
        // (consider as spaces are not exists, those are only for easy to see.)
        //
        // If number of continuous \ is even, those are some couple of
        // escape code and `\` charactor.
        // eg "\\ \\ \\ \\ ptn" (The parrern is not escaped by \.)
        //
        // If make some couple of \ (\\) and still remains one \
        // it means ptn is escaped.
        // In case of html, it is not an element.
        // eg: \<a\>
        //
        // Find escap charactor befor ptn position.
        // &str[index_start..ptn_start]: str before ptn
        // Find \ at the end of &str[index_start..ptn_start]
        if let Some(cap) = re_esc.captures(&str[index_start..ptn_start]) {
            // escape charactor found
            if &cap[1].len() % 2 == 1 {
                // Number of charactor in cap is odd (ex: \\ \).
                // ptn is escaped.
                // skip escaped ptn and go further on.
                // Add mat.end() to index_start.
                match index_start.checked_add(mat.end()) {
                    Some(v) => {
                        index_start = v;
                        continue;
                    }
                    None => break,
                };
            }
        }

        let ptn_end = match index_start.checked_add(mat.end()) {
            Some(v) => v,
            None => break,
        };

        // Return ptn position that is not escaped.
        return Some((ptn_start, ptn_end));
    }

    // Reached at the end of str and ptn not escaped was not found.
    None
}

/// Find ptn from str_hole.
/// find_start: start position to find
/// ptn: pattern to find
fn _find_not_escaped_alt1(str_hole: &str, find_start: usize, ptn: &str) -> Option<(usize, usize)> {
    // find_start is more than str_hole.
    if str_hole.len() <= find_start {
        return None;
    }

    let str = &str_hole[find_start..];

    // Regular expression to find ptn
    let re_ptn = regex::Regex::new(&ptn).unwrap();
    // Regular expression to find backslash `\` continuing more than two.
    // at the end of str.
    let re_esc = regex::Regex::new(r"(\\+)$").unwrap();

    // index_current: index position handling currently.
    let mut index_current: usize = 0;

    loop {
        if str.len() <= index_current {
            // ptn was not found
            break;
        }

        let str_crt = &str[index_current..];

        // find ptn
        let mat = match re_ptn.find(&str_crt) {
            Some(v) => v,
            // ptn was not found
            None => break,
        };

        // index position of ptn .
        let ptn_index = mat.start();

        // Check if the ptn is escaped.
        // To do that, count number of \ before ptn.
        //
        // \ptn
        // if \ exists just befor ptn
        // it might be an escape code for ptn (\ptn)
        // or just `\` charactor (\\ptn)
        //
        // In case of `\` charactor,
        // it should be escape code \ before `\` charactor
        // so \\ is a caractor `\` with escaped code.
        //
        // If number of continuous \ is odd, the last \ is escape code.
        // eg "\\ \\ \\ \\ \ptn" (The parrern is escaped by \.)
        // (consider as spaces are not exists, those are only for easy to see.)
        //
        // If number of continuous \ is even, those are some couple of
        // escape code and `\` charactor.
        // eg "\\ \\ \\ \\ ptn" (The parrern is not escaped by \.)
        //
        // If make some couple of \ (\\) and still remains one \
        // it means ptn is escaped, that meas ptn is not an html element
        //
        // &str_crt[..ptn_index]: str before ptn
        // Find \ at the end of &str_crt[..ptn_index].
        //

        // let cap_op = re_esc.captures(&str_crt[..ptn_index]);
        // if let Some(cap) = cap_op {
        //     // escaped
        //     if &cap[1].len() % 2 == 1 {
        //         // skip escaped ptn and go further on.
        //         index_current += mat.end();
        //         continue;
        //     }
        // }

        // Find escap charactor befor ptn position.
        if let Some(cap) = re_esc.captures(&str_crt[..ptn_index]) {
            // escape charactor found
            if &cap[1].len() % 2 == 1 {
                // Number of charactor in cap is odd (ex: \\ \).
                // ptn is escaped.
                // skip escaped ptn and go further on.

                // index_current += mat.end();

                // Add mat.end() to index_current.
                match index_current.checked_add(mat.end()) {
                    Some(v) => {
                        index_current = v;
                        continue;
                    }
                    // index_current reaches usize::MAX.
                    // Can not work further more.
                    None => break,
                }
            }
        }

        // match re_esc.captures(&str_crt[..ptn_index]) {
        //     Some(cap) => {
        //         if &cap[1].len() % 2 == 1 {
        //             // Number of charactor in cap is odd.
        //             // skip escaped ptn and go further on.
        //             index_current += mat.end();
        //             continue;
        //         }
        //     }
        //     // not escaped
        //     None => (),
        // }

        // ptn was found and it is not escaped
        return Some((
            find_start + index_current + mat.start(),
            find_start + index_current + mat.end(),
        ));
    }

    // Reached at the end of str and str not escaped was not found.
    None
}

/// Return href value position of <a href="href_value"> element
/// in &str as Some((start, end)).
/// It searches on &str[find_start..], &str[..find_start] is ignored.
/// Start end end position are counted as &str[0] is 0.
/// If href value is not found, or any err, returns None.
fn href_pos(str: &str, find_start: usize) -> Option<(usize, usize)> {
    // let mut index = 0;
    // href=value
    // let re_href = regex::Regex::new(r#"(?i)\s*href\s*=\s*["']"#).unwrap();

    // Search <a, but not \<a: escaped
    let (_a_start, a_end) = match find_not_escaped(&str, find_start, "<a") {
        Some(v) => v,
        // <a not found
        None => return None,
    };

    // index += a_end;

    // position end of <a
    // let mut index = a_end;

    // Find href="value or href='value
    let re_href = regex::Regex::new(r#"(?i)\s*href\s*=\s*["']"#).unwrap();
    let href_mat = match re_href.find(&str[a_end..]) {
        Some(v) => v,
        // href="value not found
        None => return None,
    };

    // first quote`"` position of href="value"
    // let q1_start = index + href_mat.end() - 1;
    // q1_start = a_end + href_mat.end() - 1;
    let q1_end = match a_end.checked_add(href_mat.end()) {
        Some(v) => v,
        None => return None,
    };
    let q1_start = match q1_end.checked_add_signed(-1) {
        Some(v) => v,
        None => return None,
    };

    // Get charactor of the quote, it is " or '.
    // let quote = &str[q1_start..q1_start + 1];
    let quote = &str[q1_start..q1_end];

    // Set index at end of the first quote
    // let index += href_mat.end();
    // let q1_end = a_end + href_mat.end();

    // Search second " (or ') from abc" of href="abc".
    // let (q2_start, _q2_end) = match find_not_escaped(&str, index, quote) {
    let (q2_start, _q2_end) = match find_not_escaped(&str, q1_end, quote) {
        Some(v) => v,
        // second quote not found.
        None => return None,
    };

    // abc of href="abc"
    Some((q1_end, q2_start))
}

#[cfg(test)]
mod tests {

    #[test]
    fn find_not_escaped() {
        // cargo test -- --nocapture
        // println!("page_urility mod tests fn find_not_escaped");

        // Case does not match.
        let str = "abc";
        let res = super::find_not_escaped(str, 0, "def");
        assert_eq!(res, None);

        // Case match whole.
        let str = "abc";
        let res = super::find_not_escaped(str, 0, "abc");
        assert!(res.is_some());
        if let Some(v) = res {
            let (start, end) = v;
            assert_eq!(start, 0);
            assert_eq!(end, 3);
        };

        // Case match some part
        let str = "abcdef";
        let res = super::find_not_escaped(str, 0, "cde");
        assert!(res.is_some());
        if let Some(v) = res {
            let (start, end) = v;
            assert_eq!(start, 2);
            assert_eq!(end, 5);
        };

        // Case match with escaped backslash
        let str = r"ab\\cdef";
        let res = super::find_not_escaped(str, 0, "cde");
        assert!(res.is_some());
        if let Some(v) = res {
            let (start, end) = v;
            assert_eq!(start, 4);
            assert_eq!(end, 7);
        };

        // Case does not match escaped ptn
        let str = r"ab\cdef";
        let res = super::find_not_escaped(str, 0, "cde");
        assert!(res.is_none());

        // assert!(true);
    }

    #[test]
    fn href_pos() {
        let str = r#"abc<a href="a_b_c">abc</a>def<a href="a_b_c">abc</a>ghi"#;
        //           01234567891123456789212345678931234567894123456789512345
        let op = super::href_pos(str, 27);
        assert!(op.is_some());
        if let Some(v) = op {
            let (start, end) = v;
            assert_eq!(start, 38);
            assert_eq!(end, 43);
        }
        // match op {
        //     Some(v) => {}
        //     None => (),
        // }
    }

    #[test]
    fn page_move_subsection_content_href_convert() {
        // Case a page moves to different url.
        // The href is link to nether fm_url nor dest_url, nor thorse children.
        // Moving the page should not change href value.

        let fm_url = "http://abc/abc.html";
        let dest_url = "http://def/def.html";
        let href1 = "http://page/data.html#d1";
        let href2 = "http://page/data.html#d1";
        let content1 = format!("ha ha ha <a href=\"{}\">data</a> he", href1);
        let content2 = format!("ha ha ha <a href=\"{}\">data</a> he", href2);
        page_move_subsection_content_href_convert_test(fm_url, dest_url, &content1, &content2);

        if false {}

        // Case href is link to fm_url's child.
        let fm_url = "http://abc/abc.html";
        let dest_url = "http://def/def.html";
        let href1 = "http://abc/abc.html#d1";
        let href2 = "#d1";
        let content1 = format!("ha ha ha <a href=\"{}\">data</a> he", href1);
        let content2 = format!("ha ha ha <a href=\"{}\">data</a> he", href2);
        page_move_subsection_content_href_convert_test(fm_url, dest_url, &content1, &content2);

        // println!("{}", content2);
        // assert_eq!(str, &content);
        // assert!(false);
    }

    fn page_move_subsection_content_href_convert_test(
        fm_url: &str,
        dest_url: &str,
        content: &str,
        content2: &str,
    ) {
        let fm_url = url::Url::parse(fm_url).unwrap();
        let dest_url = url::Url::parse(dest_url).unwrap();
        let content_converted =
            super::page_move_subsection_content_href_convert(content, &fm_url, &dest_url);
        assert_eq!(content_converted, content2);
    }
}

// work note
