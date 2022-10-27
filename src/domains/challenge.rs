use std::str::FromStr;

use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Challenge {
    pub id: uuid::Uuid,
    pub title: ChallengeTitle,
    pub description: ChallengeDescription,
    pub created_at: i64,
}

impl Challenge {
    pub fn new(
        id: uuid::Uuid,
        title: ChallengeTitle,
        description: ChallengeDescription,
        created_at: i64,
    ) -> Self {
        Self {
            id,
            title,
            description,
            created_at,
        }
    }
}

#[derive(Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct ChallengeTitle(String);

impl ChallengeTitle {
    pub fn new<S: Into<String>>(str: S) -> Result<Self, String> {
        let string = str.into();
        if string.len() < 10 {
            return Err("Title must be more than 10 characters".to_string());
        }

        Ok(ChallengeTitle(string))
    }
}

impl AsRef<str> for ChallengeTitle {
    fn as_ref(&self) -> &str {
        return &self.0;
    }
}

impl FromStr for ChallengeTitle {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

#[derive(Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct ChallengeDescription(String);

impl ChallengeDescription {
    pub fn new<S: Into<String>>(str: S) -> Result<Self, String> {
        let string = str.into();
        if string.len() < 10 {
            return Err("Description should be at least 10 characters".into());
        }

        Ok(Self { 0: string })
    }
}

impl FromStr for ChallengeDescription {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl AsRef<str> for ChallengeDescription {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod test {
    use super::ChallengeTitle;

    #[test]
    fn cannot_create_challenge_title_less_than_10_char() {
        let challenge_title = ChallengeTitle::new("hello");
        assert!(challenge_title.is_err())
    }
}
