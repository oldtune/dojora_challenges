use chrono::prelude::*;
pub fn get_unix_timestamp(time: chrono::DateTime<Utc>) -> i64 {
    time.timestamp()
}

pub trait TimeProvider {
    fn get_timestamp(&self) -> i64;
    fn get_timestamp_from_utc(&self, time: &DateTime<Utc>) -> i64;
}

pub struct AppTimeProvider;

impl AppTimeProvider {
    pub fn new() -> Self {
        Self
    }
}

impl TimeProvider for AppTimeProvider {
    fn get_timestamp(&self) -> i64 {
        self.get_timestamp_from_utc(&chrono::Utc::now())
    }

    fn get_timestamp_from_utc(&self, time: &chrono::DateTime<Utc>) -> i64 {
        DateTime::timestamp(&time)
    }
}
