// use markup5ever_rcdom::{Handle, Node, NodeData, RcDom, SerializableHandle};
// use std::collections::HashMap;
use std::str::FromStr;
// use tracing::info; // {error, event, info, instrument, span, Level, Node}
// use tracing::info;

pub struct PageJson {
    data: Option<json::JsonValue>,
}

impl PageJson {
    pub fn new() -> PageJson {
        PageJson {
            data: Some(page_json_plain()),
        }
    }

    pub fn from(data: json::JsonValue) -> PageJson {
        PageJson { data: Some(data) }
    }

    pub fn value(&self) -> Option<&json::JsonValue> {
        self.data.as_ref()
    }

    pub fn value_mut(&mut self) -> Option<&mut json::JsonValue> {
        match self.data.as_mut() {
            Some(v) => Some(v),
            None => {
                eprintln!("Failed to get data in json as mutable");
                None
            }
        }
    }

    pub fn value_take(&mut self) -> Option<json::JsonValue> {
        self.data.take()
    }

    // pub fn data_mut(&mut self) -> Option<&mut json::JsonValue> {
    //     match self.data.as_mut() {
    //         Some(v) => Some(v),
    //         None => {
    //             eprintln!("Failed to get mutable data in json");
    //             None
    //         }
    //     }
    // }

    pub fn rev(&self) -> Option<usize> {
        // case rev=10: Number(Number { category: 1, exponent: 0, mantissa: 10 })
        if let Ok(rev) = to_usize(&self.value()?["data"]["page"]["rev"]) {
            return Some(rev);
        }

        // case: rev="12" ( with "" )
        match self.value()?["data"]["page"]["rev"] {
            json::JsonValue::Short(short) => {
                let rev = short.as_str();
                match usize::from_str(rev) {
                    // Ok(v) => Some(v),
                    Ok(v) => return Some(v),
                    Err(_) => {
                        eprintln!("Failed to get rev");
                        return None;
                    }
                }
            }
            _ => None::<usize>,
        };

        None
    }

    // rev counted up from current rev
    pub fn rev_uped(&self) -> Option<usize> {
        let rev = self.rev()?;
        Some(rev + 1)
    }

    pub fn subsection_id_next(&self) -> Option<usize> {
        // return to_usize(&self.data.as_ref()?["data"]["subsection"]["id"]["id_next"]).ok();
        to_usize(&self.data.as_ref()?["data"]["subsection"]["id"]["id_next"]).ok()
    }

    /// Return new subsection id getting from ["data"]["subsection"]["id"]["id_next"]
    /// and add one to id_next.
    pub fn subsection_id_new(&mut self) -> Option<usize> {
        let id = self.subsection_id_next()?;
        let id_next = id.checked_add(1)?;

        let data = self.data.as_mut()?;
        data["data"]["subsection"]["id"]["id_next"] = id_next.into();
        Some(id)
    }

    pub fn subsection_new(&mut self, parent_id: &usize) -> Option<Subsection> {
        let parent_id_str = parent_id.to_string();

        // get id hrer before self is borrowed as mutable.
        let id = self.subsection_id_new()?;
        let id_str = id.to_string();

        // subsection for paren_id must exists.
        let page_json = self.data.as_mut()?;
        if page_json["data"]["subsection"]["data"][parent_id_str.as_str()].is_null() {
            return None;
        }

        // already exists
        if !page_json["data"]["subsection"]["data"][id_str.as_str()].is_null() {
            return None;
        }

        page_json["data"]["subsection"]["data"][id_str.as_str()] = json::object! {
            "parent_id" : *parent_id,
            "id":  id,
        };

        // Set new subsection's id to parent subsection
        {
            let parent = &mut page_json["data"]["subsection"]["data"][parent_id_str.as_str()];
            let _ = parent["child"].push(id);
        }

        Some(Subsection {
            page_json: self.data.as_mut().unwrap(),
            id,
        })
    }

    pub fn subsection_by_name(&mut self, href_arg: &str) -> Option<Subsection> {
        let page_json = self.data.as_mut()?;

        // Search subsection that has the href_arg value.
        let mut id_str_match = None;
        if let json::JsonValue::Object(object) = &page_json["data"]["subsection"]["data"] {
            for (id_str, subsection) in object.iter() {
                // `(&str, &JsonValue)`
                if let json::JsonValue::Object(object) = subsection {
                    let href = match object.get("href") {
                        Some(v) => v,
                        None => continue,
                    };

                    if href == href_arg {
                        id_str_match.replace(id_str);
                        break;
                    }
                };
            }
        };

        if let Some(id_str) = id_str_match {
            let id = usize::from_str_radix(id_str, 10).ok()?;
            // info!("id: {}", id);
            return Some(Subsection {
                page_json: self.data.as_mut().unwrap(),
                id,
            });
        }
        None
    }

    pub fn subsections(&self) -> Option<&json::object::Object> {
        // let data = self.data()?;
        let value = self.value()?;
        if value["data"]["subsection"]["data"].is_empty() {
            return None;
        }
        match value["data"]["subsection"]["data"] {
            json::JsonValue::Object(ref object) => Some(object),
            _ => None,
        }
    }

    pub fn subsections_data_exists(&self) -> bool {
        self.subsections()
            .and_then(|subsections| {
                // value["data"]["subsection"]["data"][0] is not real content.
                // if 1 < subsections["data"].len() {
                if 1 < subsections.len() {
                    Some(subsections) // true for is_some()
                } else {
                    None // false for is_some()
                }
            })
            .is_some()
    }

    /// Return where the page was moved to in some option.
    /// Return None if not muved.
    pub fn moved_to(&self) -> Option<String> {
        let value = self.value()?;
        let moveto = &value["data"]["page"]["moved_to"];
        if moveto.is_empty() {
            return None;
        }

        moveto.as_str().and_then(|v| Some(v.to_string()))
    }
}

fn to_usize(v: &json::JsonValue) -> Result<usize, String> {
    match v {
        json::JsonValue::Number(number) => {
            match <json::number::Number as TryInto<usize>>::try_into(*number) {
                Ok(num) => return Ok(num),
                Err(_) => {
                    return Err("Failed to convert to usize.".to_string());
                }
            };
        }
        _ => {
            return Err("It is not number".to_string());
        }
    }
}

pub struct Subsection<'a> {
    page_json: &'a mut json::JsonValue,
    id: usize,
}

impl Subsection<'_> {
    //     fn from_id(page_json: &PageJson, id: usize) -> Subsection {
    //         // Subsection { page_json, id }
    //     }

    pub fn id(&self) -> usize {
        // index of json_value["data"]["subsection"]["data"] is string, eg "0"
        self.id
    }

    pub fn _title(&self) -> Option<String> {
        let id_str = self.id.to_string();
        self.page_json["data"]["subsection"]["data"][id_str.as_str()]["title"]
            .as_str()
            .and_then(|str| Some(str.to_string()))
    }

    pub fn title_set(&mut self, title: &str) {
        let id_str = self.id.to_string();
        self.page_json["data"]["subsection"]["data"][id_str.as_str()]["title"] =
            json::value!(title.into());
    }

    pub fn _href(&self) -> Option<String> {
        let id_str = self.id.to_string();
        self.page_json["data"]["subsection"]["data"][id_str.as_str()]["href"]
            .as_str()
            .and_then(|str| Some(str.to_string()))
    }

    pub fn href_set(&mut self, href: &str) {
        let id_str = self.id.to_string();
        self.page_json["data"]["subsection"]["data"][id_str.as_str()]["href"] =
            json::value!(href.into());
    }

    pub fn contents_mut(&mut self) -> &mut json::JsonValue {
        let id_str = self.id.to_string();
        let subsection_data = &mut self.page_json["data"]["subsection"]["data"][id_str.as_str()];

        if subsection_data["content"].is_empty() {
            subsection_data["content"] = json::array! {};
        }

        // Some(&mut subsection_data["content"])
        &mut subsection_data["content"]
    }
}

// index of json_value["data"]["subsection"]["data"] is string, eg "0"
// but numbers are used in json_value["data"]["subsection"]["id"],
// json_value["data"]["subsection"]["data"]["0"]["child"] = [1, 4, 5] are numbers.
// So it is better to meke all same useing numbers in futer.
//
pub fn page_json_plain() -> json::JsonValue {
    // ~/projects/wc/wc/src/page_json_utility.rs
    json::object! {
        // "syttem" :
        "system" : {
            // "version" : "0.0.1",
            // "version" : "0.0.2",
            // "version" : "0.0.3",
            "version" : "0.0.4",
        },

        "data" : {
            "page" : {
                "title" : "",
                "rev" : 0,
                "rev_speculation" : 0,
                "group_top" : false,
                "moved_to" : "",
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
                    "id_next" : 1,
                    "id_notinuse" : []
                },

                "data" : {
                    "0" : {
                        // "parent" : "",
                        "parent" : 0,
                        // "id" : "0",
                        "id" : 0,
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

            // "href" : {
            //     "relation" : {},
            //     "last" : {},
            //     "history" : {},
            // },
        },
    }
}
