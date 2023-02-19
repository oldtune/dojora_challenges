use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::routes::auth_route::UserViewModel;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
}

pub async fn save_user(user: &User, db: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"insert into appuser values($1, $2, $3)"#,
        uuid::Uuid::new_v4(),
        user.username,
        user.password,
    )
    .execute(db)
    .await?;
    Ok(())
}

pub async fn check_user_password(
    user: &UserViewModel,
    db: &PgPool,
) -> Result<Option<User>, sqlx::Error> {
    let result = sqlx::query_as!(
        User,
        r#"select * from appuser where username like $1 and password like $2"#,
        user.username,
        user.password
    )
    .fetch_one(db)
    .await?;

    Ok(Some(result))
}

pub async fn user_exist(username: &str, db: &PgPool) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!(
        r#"select count(*) as usercount from appuser where username like $1"#,
        username
    )
    .fetch_one(db)
    .await?;

    Ok(result.usercount.unwrap() > 0)
}
