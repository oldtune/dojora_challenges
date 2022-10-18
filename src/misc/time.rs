use chrono::prelude::*;
pub fn get_unix_timestamp(time: chrono::DateTime<Utc>) -> i64 {
    time.timestamp()
}
