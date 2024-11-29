use std::{collections::HashMap, time::{SystemTime, UNIX_EPOCH}};

pub fn get_time() -> i64 {
    // epoch unix, in seconds
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards (???)")
        .as_secs() as i64
}

pub fn from_query(k: &str, q: &HashMap<String, String>) -> String {
    return urlencoding::decode(q.get(&k.to_string()).unwrap().clone().as_str()).unwrap().to_string()
}
