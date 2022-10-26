use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Suggestion {
    pub id: uuid::Uuid,
    pub title: SuggestionTitle,
    pub description: SuggestionDescription,
    pub created_at: i64,
}

impl Suggestion {
    pub fn new(
        id: uuid::Uuid,
        title: SuggestionTitle,
        description: SuggestionDescription,
        created_at: i64,
    ) -> Self {
        Suggestion {
            id,
            title,
            description,
            created_at,
        }
    }
}

#[derive(Serialize)]
pub struct SuggestionDescription(String);

impl SuggestionDescription {
    pub fn new<S: Into<String>>(somestring: S) -> Result<Self, String> {
        let string = somestring.into();
        if string.len() < 10 {
            return Err("Description should be at least 10 words".into());
        }

        Ok(Self { 0: string })
    }
}

impl AsRef<String> for SuggestionDescription {
    fn as_ref(&self) -> &String {
        return &self.0;
    }
}

#[derive(Serialize)]
pub struct SuggestionTitle(String);

impl SuggestionTitle {
    pub fn new<S: Into<String>>(some_string: S) -> Result<Self, String> {
        let string = some_string.into();
        if string.len() < 10 {
            return Err("Title should be at least 10 characters".into());
        }

        Ok(Self { 0: string })
    }
}

impl AsRef<String> for SuggestionTitle {
    fn as_ref(&self) -> &String {
        return &self.0;
    }
}
