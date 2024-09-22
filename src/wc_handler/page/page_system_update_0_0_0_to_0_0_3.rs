// use crate::page5;

// pub fn update(page: &mut page5::Page) -> bool {
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
