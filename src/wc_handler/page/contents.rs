pub struct Contents {
    data: Option<json::JsonValue>,
}

impl Contents {
    pub fn from(data: json::JsonValue) -> Contents {
        Contents { data: Some(data) }
    }

    /// Set page_json plain.
    /// If self.data is not None, do nothing.
    pub fn data_plain_set(&mut self) {
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
    pub fn rev(&self) -> Option<u32> {
        match self.data()?["data"]["page"]["rev"].as_u32() {
            Some(v) => Some(v),
            None => {
                eprintln!("Failed to get rev");
                None
            }
        }
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
