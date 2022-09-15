use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::query_builder::{Query, QueryFragment};
use diesel::query_dsl::LoadQuery;
use diesel::sql_types::BigInt;

pub trait Paginate: Sized {
    fn paginate(self, page: i64, per_page: i64) -> Paginated<Self>;
}

impl<T> Paginate for T
where
    T: QueryDsl,
{
    fn paginate(self, page: i64, per_page: i64) -> Paginated<Self> {
        Paginated {
            query: self,
            offset: page * per_page,
            per_page,
        }
    }
}

#[derive(Debug, Clone, Copy, QueryId)]
pub struct Paginated<T> {
    query: T,
    per_page: i64,
    offset: i64,
}

impl<T> Paginated<T> {
    pub fn load_paging<'a, U>(self, conn: &mut PgConnection) -> QueryResult<Vec<U>>
    where
        Self: LoadQuery<'a, PgConnection, U>,
    {
        self.load::<U>(conn)
    }
}

impl<T: Query> Query for Paginated<T> {
    type SqlType = T::SqlType;
}

impl<T> RunQueryDsl<PgConnection> for Paginated<T> {}

impl<T> QueryFragment<Pg> for Paginated<T>
where
    T: QueryFragment<Pg>,
{
    fn walk_ast<'b>(
        &'b self,
        mut out: diesel::query_builder::AstPass<'_, 'b, Pg>,
    ) -> diesel::QueryResult<()> {
        out.push_sql("SELECT * FROM (");
        self.query.walk_ast(out.reborrow())?;
        out.push_sql(") AS T ");
        out.push_sql(" LIMIT ");
        out.push_bind_param::<BigInt, _>(&self.per_page)?;
        out.push_sql(" OFFSET ");
        out.push_bind_param::<BigInt, _>(&self.offset)?;
        Ok(())
    }
}
