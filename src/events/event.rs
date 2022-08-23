use serde::Serialize;

#[derive(Serialize)]
pub struct Event<T: Serialize> {
    event_type: String,
    id: String,
    metadata: EventMetaData,
    data: T,
}

impl<T: Serialize> Event<T> {
    pub fn new(event_type: String, metadata: EventMetaData, id: String, data: T) -> Self {
        Self {
            event_type,
            id,
            metadata,
            data,
        }
    }
}

#[derive(Serialize)]
pub struct EventMetaData {
    trace_id: String,
    user_id: Option<String>,
}

impl EventMetaData {
    pub fn new(trace_id: String, user_id: Option<String>) -> Self {
        Self { trace_id, user_id }
    }
}
