use anyhow::anyhow;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};

use crate::db::Error;
use crate::models::taxpayer::{InsertableTaxpayer, QueryableTaxpayer, UpdatableTaxpayer};
use crate::schema::fiscalidade_taxpayers as taxpayers;
use crate::utils;

pub fn list(conn: &PgConnection) -> Result<Vec<QueryableTaxpayer>, Error> {
    Ok(taxpayers::table.load(conn)?)
}

pub fn create(
    conn: &PgConnection,
    taxpayer: &mut InsertableTaxpayer,
) -> Result<QueryableTaxpayer, Error> {
    taxpayer.token = utils::generate_token();
    Ok(diesel::insert_into(taxpayers::table)
        .values(&*taxpayer)
        .get_result(conn)?)
}

pub fn update(
    conn: &PgConnection,
    id: i64,
    taxpayer: &UpdatableTaxpayer,
) -> Result<QueryableTaxpayer, Error> {
    Ok(diesel::update(taxpayers::table.find(id))
        .set(taxpayer)
        .get_result(conn)?)
}

pub fn delete(conn: &PgConnection, id: i64) -> Result<QueryableTaxpayer, Error> {
    if id == 1 {
        return Err(anyhow!("Não é possível excluir o administrador padrão do servidor").into());
    }
    Ok(diesel::delete(taxpayers::table.find(id)).get_result(conn)?)
}

pub fn by_token(conn: &PgConnection, token: &str) -> Result<QueryableTaxpayer, Error> {
    Ok(taxpayers::table
        .filter(taxpayers::token.eq(token))
        .get_result(conn)?)
}
