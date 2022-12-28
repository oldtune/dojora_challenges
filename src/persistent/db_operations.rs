fn find<T: DbObject>(query: &str) -> Result<Option<T>, sqlx::Error> {}

fn find_all<T: DbObject>(query: &str) -> Result<Vec<T>, sqlx::Error> {}

fn insert<T: DbObject>(query: &str, item: T) -> Result<(), sqlx::Error> {}

fn update<T: DbObject>(query: &str, item: T) -> Result<(), sqlx::Error> {}

fn delete<T: DbObject>(query: &str, item: T) -> Result<(), sqlx::Error> {}
