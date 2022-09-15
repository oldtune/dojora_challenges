use diesel::{r2d2::ConnectionManager, PgConnection};

pub type DbPool = diesel::r2d2::Pool<ConnectionManager<PgConnection>>;
