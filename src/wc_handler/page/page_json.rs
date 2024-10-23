use std::str::FromStr;
// use tracing::info;

pub struct PageJson {
    data: Option<json::JsonValue>,
}

impl PageJson {
    // pub fn new() -> PageJson {
    //     PageJson {
    //         data: Some(page_json_plain()),
    //     }
    // }

    pub fn from(data: json::JsonValue) -> PageJson {
        PageJson { data: Some(data) }
    }

    pub fn value(&self) -> Option<&json::JsonValue> {
        self.data.as_ref()
    }

    // pub fn value_mut(&mut self) -> Option<&mut json::JsonValue> {
    //     match self.data.as_mut() {
    //         Some(v) => Some(v),
    //         None => {
    //             eprintln!("Failed to get data in json as mutable");
    //             None
    //         }
    //     }
    // }

    // pub fn data_mut(&mut self) -> Option<&mut json::JsonValue> {
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
        // match self.data()?["data"]["page"]["rev"] {
        match self.value()?["data"]["page"]["rev"] {
            // case rev=10: Number(Number { category: 1, exponent: 0, mantissa: 10 })
            json::JsonValue::Number(number) => {
                match number.try_into() {
                    Ok(rev) => return Some(rev),
                    Err(_) => {
                        eprintln!("Failed to get rev");
                        return None;
                    }
                };
            }
            // case: rev="12" ( with "" )
            json::JsonValue::Short(short) => {
                let rev = short.as_str();
                match usize::from_str(rev) {
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

        // DBG (rmove the line below for production)
        // info!("DBG pass fn moved_to");
        // return None;

        moveto.as_str().and_then(|v| Some(v.to_string()))
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
