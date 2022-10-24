use std::{error::Error, str::FromStr};

use serde::Serialize;
use sqlx::{database::HasValueRef, Database, Decode, Encode};

#[derive(Serialize)]
pub struct Challenge {
    pub id: uuid::Uuid,
    pub title: ChallengeTitle,
    pub description: String,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
}

impl Challenge {
    pub fn new(
        id: uuid::Uuid,
        title: ChallengeTitle,
        description: String,
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

#[derive(Serialize, Encode)]
#[sqlx(transparent)]
pub struct ChallengeTitle(String);

impl ChallengeTitle {
    pub fn new(str: &str) -> Result<Self, String> {
        if str.len() < 10 {
            return Err("Title must be more than 10 characters".to_string());
        }

        Ok(ChallengeTitle(str.to_string()))
    }
}

impl AsRef<str> for ChallengeTitle {
    fn as_ref(&self) -> &str {
        return &self.0;
    }
}

// DB is the database driver
// `'r` is the lifetime of the `Row` being decoded
impl<'r, DB: Database> Decode<'r, DB> for ChallengeTitle
where
    // we want to delegate some of the work to string decoding so let's make sure strings
    // are supported by the database
    &'r str: Decode<'r, DB>,
{
    fn decode(
        value: <DB as HasValueRef<'r>>::ValueRef,
    ) -> Result<ChallengeTitle, Box<dyn Error + 'static + Send + Sync>> {
        // the interface of ValueRef is largely unstable at the moment
        // so this is not directly implementable

        // however, you can delegate to a type that matches the format of the type you want
        // to decode (such as a UTF-8 string)

        let value = <&str as Decode<DB>>::decode(value)?;

        // now you can parse this into your type (assuming there is a `FromStr`)

        Ok(value.parse()?)
    }
}

impl FromStr for ChallengeTitle {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.into())
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
