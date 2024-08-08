use std::str::FromStr;
use tracing::{event, info, instrument, span, Level};

pub struct Contents {
    data: Option<json::JsonValue>,
}

impl Contents {
    pub fn from(data: json::JsonValue) -> Contents {
        Contents { data: Some(data) }
    }

    pub fn new() -> Contents {
        Contents {
            data: Some(json_plain()),
        }
    }

    /// Set page_json plain.
    /// If self.data is not None, do nothing.
    pub fn data_plain_set(&mut self) {
        info!("fn data_plain_set");

        if self.data.is_some() {
            return;
        }
        self.data.replace(json_plain());
    }

    /// Returns reference of data in json as enum option.
    /// To call self.data(), call self.data_set() previously.
    pub fn data(&self) -> Option<&json::JsonValue> {
        match self.data.as_ref() {
            Some(v) => Some(v),
            None => {
                eprintln!("Failed to get data in json");
                None
            }
        }
    }

    pub fn data_mut(&mut self) -> Option<&mut json::JsonValue> {
        match self.data.as_mut() {
            Some(v) => Some(v),
            None => {
                eprintln!("Failed to get mut data in json");
                None
            }
        }
    }

    // current rev
    // "version":"0.0.3" "rev":10
    pub fn rev(&self) -> Option<u32> {
        // data()?["data"]["page"]["rev"]:
        // may be
        // case rev="138": Short("138")
        // or
        // case rev=10: Number(Number { category: 1, exponent: 0, mantissa: 10 })

        // data()?["data"]["page"]["rev"]: Short("134")
        // println!(
        //     "contents.rs fn rev rev: {:?}",
        //     self.data()?["data"]["page"]["rev"]
        // );

        match self.data()?["data"]["page"]["rev"] {
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
            // case: rev="12"
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

        // let rev = match self.data()?["data"]["page"]["rev"] {
        //     // case: rev="12"
        //     json::short::Short => {
        //         // let rev = self.data()?["data"]["page"]["rev"].as_str()?;
        //         let rev = rev.as_str()?;
        //         match u32::from_str(rev) {
        //             Ok(v) => Some(v),
        //             Err(_) => {
        //                 eprintln!("Failed to get rev");
        //                 None
        //             }
        //         }
        //     }
        //     json::number::Number => {
        //         //		let rev = self.data()?["data"]["page"]["rev"].as_str()?;

        //         let rev = self.data()?["data"]["page"]["rev"];

        //         match u32::from(rev) {
        //             Ok(v) => Some(v),
        //             Err(_) => {
        //                 eprintln!("Failed to get rev");
        //                 None
        //             }
        //         }
        //     }
        //     _ => None,
        // };

        // rev

        // let rev = self.data()?["data"]["page"]["rev"].as_str()?;
        // match u32::from_str(rev) {
        //     Ok(v) => Some(v),
        //     Err(_) => {
        //         eprintln!("Failed to get rev");
        //         None
        //     }
        // }

        // temp
        None
    }

    // rev counted up from current rev
    pub fn rev_uped(&self) -> Option<u32> {
        let rev = self.rev()?;
        Some(rev + 1)
    }
}

fn json_plain() -> json::JsonValue {
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
