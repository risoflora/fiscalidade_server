use diesel::{ExpressionMethods, OptionalExtension, PgConnection, QueryDsl, RunQueryDsl};

use crate::db::Error;
use crate::models::cache::{InsertableCache, QueryableCache};
use crate::schema::fiscalidade_caches as caches;

pub fn set(conn: &PgConnection, cache: &InsertableCache<'_>) -> Result<QueryableCache, Error> {
    Ok(diesel::insert_into(caches::table)
        .values(cache)
        .on_conflict(caches::key)
        .do_update()
        .set(caches::value.eq(&cache.value))
        .get_result(conn)?)
}

pub fn set_value(conn: &PgConnection, key: &str, value: &Vec<u8>) -> Result<Vec<u8>, Error> {
    let cache = self::set(conn, &InsertableCache { key, value })?;
    Ok(cache.value)
}

pub fn lookup(conn: &PgConnection, key: &str) -> Result<Option<QueryableCache>, Error> {
    Ok(caches::table
        .filter(caches::key.eq(key))
        .get_result(conn)
        .optional()?)
}

pub fn lookup_value(conn: &PgConnection, key: &str) -> Result<Option<Vec<u8>>, Error> {
    Ok(self::lookup(conn, key)?.map(|cache| cache.value))
}

pub fn list(conn: &PgConnection) -> Result<Vec<QueryableCache>, Error> {
    Ok(caches::table.load(conn)?)
}
