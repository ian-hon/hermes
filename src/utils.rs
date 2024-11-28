use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_time() -> i64 {
    // epoch unix, in seconds
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards (???)")
        .as_secs() as i64
}