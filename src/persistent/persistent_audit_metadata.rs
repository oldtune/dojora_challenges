use serde::Serialize;
use sqlx::{postgres::PgValueRef, Decode, Postgres};

pub trait AuditMetadata {
    fn get_created_at() -> i64;
    fn get_upated_at() -> i64;
    fn get_created_by() -> Option<Author>;
    fn get_updated_by() -> Option<Author>;
}

pub enum Author {
    System,
    Admin,
    ManuallyInserted,
    User(String),
}

impl Serialize for Author {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Admin => serializer.serialize_str("admin"),
            Self::ManuallyInserted => serializer.serialize_str("manually"),
            Self::System => serializer.serialize_str("system"),
            Self::User(some_user) => serializer.serialize_str(some_user as &str),
        }
    }
}

impl Decode<'_, Postgres> for Author {
    fn decode(value: PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        Ok(Author::from(value.as_str().unwrap()))
    }
}

impl From<&str> for Author {
    fn from(value: &str) -> Self {
        if value == "admin" {
            return Self::Admin;
        }
        if value == "system" {
            return Self::System;
        }
        if value == "manually" {
            return Self::ManuallyInserted;
        }

        Self::User(value.to_string())
    }
}

impl AsRef<str> for Author {
    fn as_ref(&self) -> &str {
        match &self {
            Author::Admin => "admin",
            Author::ManuallyInserted => "manually",
            Author::System => "system",
            Author::User(user) => &user,
        }
    }
}
