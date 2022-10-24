use serde::Serialize;

#[derive(Serialize)]
pub struct Suggestion {
    title: String,
    description: SuggestionDescription,
}

impl Suggestion {
    pub fn new(title: String, description: SuggestionDescription) -> Self {
        Suggestion {
            title: title,
            description: description,
        }
    }
}

#[derive(Serialize)]
pub struct SuggestionDescription(String);

impl SuggestionDescription {
    pub fn new<S: Into<String>>(somestring: S) -> Self {
        Self {
            0: somestring.into(),
        }
    }
}
