use serde_json;

use crate::structs::entry::Entry;

pub fn to_json(unprocessed: String) -> Entry {
    let processed: serde_json::Value = serde_json::from_str(&unprocessed).unwrap();
    let data = &processed["data"];

    let json: Entry = serde_json::from_value(data.clone()).unwrap();

    json
}
