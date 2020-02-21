use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};

use crate::db::Error;
use crate::models::service::QueryableService;
use crate::schema::fiscalidade_services as services;

pub fn list(conn: &PgConnection) -> Result<Vec<QueryableService>, Error> {
    Ok(services::table.load(conn)?)
}

pub fn by_slug(conn: &PgConnection, slug: &str) -> Result<QueryableService, Error> {
    Ok(services::table
        .filter(services::slug.eq(slug))
        .get_result(conn)?)
}
