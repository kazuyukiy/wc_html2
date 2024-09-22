use std::str::FromStr;
use tracing::info;

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

    // pub fn data(&self) -> Option<&json::JsonValue> {
    pub fn value(&self) -> Option<&json::JsonValue> {
        self.data.as_ref()
        // match self.data.as_ref() {
        //     Some(v) => Some(v),
        //     None => {
        //         eprintln!("Failed to get data in json");
        //         None
        //     }
        // }
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

    pub fn rev(&self) -> Option<u32> {
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
                match u32::from_str(rev) {
                    Ok(v) => return Some(v),
                    Err(_) => {
                        eprintln!("Failed to get rev");
                        return None;
                    }
                }
            }
            _ => None::<u32>,
        };

        None
    }

    // rev counted up from current rev
    pub fn rev_uped(&self) -> Option<u32> {
        let rev = self.rev()?;
        Some(rev + 1)
    }

    // data["data"]["navi"] {
    // pub fn navi(&self) -> Option<&json::object::Object> {
    // pub fn navi(&self) -> Option<&json::JsonValue> {
    // pub fn navi(&self) -> Option<&json::JsonValue::Array> {
    pub fn navi(&self) -> Option<&json::Array> {
        // DBG
        // info!("fn navi");

        // let data = self.data()?;
        let value = self.value()?;

        // DBG
        // info!("fn navi cp1");

        if value["data"]["navi"].is_empty() {
            return None;
        }

        // DBG
        // info!("fn navi cp2 navi: {}", value["data"]["navi"]);

        match value["data"]["navi"] {
            json::JsonValue::Array(ref vec) => Some(vec),
            _ => None,
        }

        // match value["data"]["navi"] {
        //     json::JsonValue::Null => None,
        //     _ => Some(&value["data"]["navi"]),
        // }

        // match value["data"]["navi"] {
        //     json::JsonValue::Object(ref object) => Some(object),
        //     _ => {
        //         // DBG
        //         info!(
        //             "fn navi cp2 navi: {} -- But not Object, return None.",
        //             value["data"]["navi"]
        //         );

        //         None
        //     }
        // }
    }

    // pub fn navi(&self) -> Option<&json::JsonValue> {
    //     let data = self.data()?;
    //     if value["data"]["navi"].is_empty() {
    //         None
    //     } else {
    //         Some(&value["data"]["navi"])
    //     }
    // }

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

    pub fn moved_to(&self) -> Option<String> {
        let value = self.value()?;
        let moveto = &value["data"]["page"]["moved_to"];
        if moveto.is_empty() {
            return None;
        }

        // DBG (rmove the line below for production)
        info!("DBG pass fn moved_to");
        return None;

        moveto.as_str().and_then(|v| Some(v.to_string()))
    }
}

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
