use chrono::NaiveDateTime;
use diesel::{dsl, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};

use crate::db::{schema::fiscalidade_taxpayers_services_view as taxpayers_services_view, Error};
use crate::models::taxpayer_service::{
    InsertableTaxpayerService, QueryableTaxpayerService, TaxpayerServiceStatus,
    ViewableTaxpayerService,
};
use crate::schema::fiscalidade_taxpayers_services as taxpayers_services;

pub fn create(
    conn: &PgConnection,
    taxpayer_service: &InsertableTaxpayerService,
) -> Result<QueryableTaxpayerService, Error> {
    Ok(diesel::insert_into(taxpayers_services::table)
        .values(taxpayer_service)
        .get_result(conn)?)
}

pub fn by_taxpayer_and_service(
    conn: &PgConnection,
    taxpayer_id: i64,
    service_id: i64,
) -> Result<QueryableTaxpayerService, Error> {
    Ok(taxpayers_services::table
        .filter(taxpayers_services::taxpayer_id.eq(taxpayer_id))
        .filter(taxpayers_services::service_id.eq(service_id))
        .get_result(conn)?)
}

pub fn list(
    conn: &PgConnection,
    status: Option<TaxpayerServiceStatus>,
) -> Result<Vec<ViewableTaxpayerService>, Error> {
    Ok(match status {
        Some(TaxpayerServiceStatus::Authorized) => taxpayers_services_view::table
            .filter(taxpayers_services_view::allowed_at.is_not_null())
            .load(conn),
        Some(TaxpayerServiceStatus::Unauthorized) => taxpayers_services_view::table
            .filter(taxpayers_services_view::allowed_at.is_null())
            .load(conn),
        None => taxpayers_services_view::table.load(conn),
    }?)
}

pub fn authorize(conn: &PgConnection, id: i64) -> Result<Option<QueryableTaxpayerService>, Error> {
    let taxpayer_service =
        diesel::update(taxpayers_services::table.filter(taxpayers_services::id.eq(id)))
            .set(taxpayers_services::allowed_at.eq(dsl::now))
            .get_result(conn)?;
    Ok(Some(taxpayer_service))
}

pub fn unauthorize(
    conn: &PgConnection,
    id: i64,
) -> Result<Option<QueryableTaxpayerService>, Error> {
    let taxpayer_service =
        diesel::update(taxpayers_services::table.filter(taxpayers_services::id.eq(id)))
            .set(taxpayers_services::allowed_at.eq(None as Option<NaiveDateTime>))
            .get_result(conn)?;
    Ok(Some(taxpayer_service))
}
