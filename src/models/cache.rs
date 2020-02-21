use chrono::NaiveDateTime;

use crate::schema::fiscalidade_caches as caches;

#[derive(Serialize, Queryable)]
pub struct QueryableCache {
    pub id: i64,
    pub key: String,
    pub value: Vec<u8>,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "caches"]
pub struct InsertableCache<'a> {
    pub key: &'a str,
    pub value: &'a Vec<u8>,
}
