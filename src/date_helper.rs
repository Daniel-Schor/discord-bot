use chrono::{DateTime, Utc};

pub fn timestamp() -> i64 {
    Utc::now().timestamp() as i64 // Get timestamp as i64
}

pub fn elapsed_time(start_timestamp: i64) -> i64 {
    let current_timestamp = timestamp();
    current_timestamp - start_timestamp
}

pub fn timestamp_string() -> String {
    let naive_datetime = DateTime::from_timestamp(timestamp(), 0)
        .expect("Invalid timestamp");
    naive_datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}