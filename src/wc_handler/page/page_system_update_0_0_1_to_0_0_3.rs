// use crate::page5;

//pub fn update(page: &mut page5::Page) -> bool {
pub fn update(page: &mut super::Page) -> bool {
    // let page_json = &mut page.page_json.value_mut();
    // let page_json = &mut page.page_json.value();
    let page_json = match page.page_json.as_mut() {
        Some(v) => v.value_mut(),
        None => return false,
    };

    if page_json.is_null() {
        return false;
    }

    // Since wrong spell "syttem" was used
    let version_op = page_json["syttem"]["version"].as_str();
    if version_op.is_none() {
        return false;
    }
    if version_op.unwrap() != "0.0.1" {
        return false;
    }

    // confirmed  "syttem" : "0.0.1"

    // remove "syttem" : "0.0.1" (wring spell)
    _ = page_json.remove("syttem");

    // set new data with colect spell
    page_json["system"] = json::object! {"version" : "0.0.3"};

    // Set href data space.
    if page_json["data"]["href"].is_null() {
        page_json["data"]["href"] = json::object! {
            "relation" : {}, "last" : {}, "history" : {}
        };
    }

    println!("page_system_update3_0_0_1_to_0_0_3");

    true
} // end of fn update

// // page_system_update_0_0_1_to_0_0_3.rs

// // use crate::page::Page;
// // use crate::json_to_dom;
// use super::json_to_dom;

// pub fn update(page: &mut super::Page) -> bool {
//     // check ["syttem"]["version"] : "0.0.1"
//     // Since wrong spell "syttem" was used
//     let page_json_value = page.page_json().page_json();
//     let version_op = page_json_value["syttem"]["version"].as_str();
//     if version_op.is_none() {
//         return false;
//     }
//     if version_op.unwrap() != "0.0.1" {
//         return false;
//     }

//     // confirmed  "syttem" : "0.0.1"

//     // remove "syttem" : "0.0.1" (wring spell)
//     _ = page_json_value.remove("syttem");
//     // set new data with colect spell
//     page_json_value["system"] = json::object! {"version" : "0.0.3"};

//     // Set href data space.
//     if page_json_value["data"]["href"].is_null() {
//         page_json_value["data"]["href"] = json::object! {
//             "relation" : {}, "last" : {}, "history" : {}
//         };
//     }

//     // Create a new page_dom and replace with it.
//     let page_dom = json_to_dom::json_to_dom(&page_json_value);
//     // page.page_dom().replace(&page_dom);
//     page.page_dom_replace(page_dom);

//     println!("page_system_update_0_0_1_to_0_0_3");

//     // update was done
//     true
// } // end of fn update
