use anyhow::anyhow;
use diesel::{dsl, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};

use crate::db::Error;
use crate::models::taxpayer::{InsertableTaxpayer, QueryableTaxpayer, UpdatableTaxpayer};
use crate::schema::fiscalidade_taxpayers as taxpayers;
use crate::utils;

fn exists_admin(conn: &PgConnection) -> bool {
    diesel::select(dsl::exists(taxpayers::table.filter(taxpayers::id.eq(1)))).get_result(conn)
        == Ok(true)
}

pub fn create_admin(conn: &PgConnection) -> Result<QueryableTaxpayer, Error> {
    if exists_admin(conn) {
        return Err(anyhow!("Já existe um administrador padrão para o servidor").into());
    }
    let taxpayer = InsertableTaxpayer {
        name: "admin".into(),
        business_name: "Administrador".into(),
        registry: Default::default(),
        email: Default::default(),
        certificate: Default::default(),
        certificate_password: Default::default(),
        token: utils::generate_token(),
    };
    let taxpayer: QueryableTaxpayer = diesel::insert_into(taxpayers::table)
        .values(taxpayer)
        .get_result(conn)?;
    if taxpayer.id != 1 {
        self::delete(conn, taxpayer.id)?;
        return Err(
            anyhow!("Ocorreu um erro ao cadastrar administrador padrão para o servidor").into(),
        );
    };
    Ok(taxpayer)
}

pub fn create(
    conn: &PgConnection,
    taxpayer: &mut InsertableTaxpayer,
) -> Result<QueryableTaxpayer, Error> {
    if !exists_admin(conn) {
        return Err(anyhow!("Não existe um administrador padrão para o servidor").into());
    }
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
    if id == 1 {
        return Err(anyhow!("Não é possível alterar o administrador padrão do servidor").into());
    }
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

pub fn list(conn: &PgConnection) -> Result<Vec<QueryableTaxpayer>, Error> {
    Ok(taxpayers::table.load(conn)?)
}

pub fn by_token(conn: &PgConnection, token: &str) -> Result<QueryableTaxpayer, Error> {
    Ok(taxpayers::table
        .filter(taxpayers::token.eq(token))
        .get_result(conn)?)
}
