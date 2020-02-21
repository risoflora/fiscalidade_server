use diesel::{dsl, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};

use crate::db::{schema::fiscalidade_taxpayers_services_view as taxpayers_services_view, Error};
use crate::models::taxpayer_service::{
    InsertableTaxpayerService, QueryableTaxpayerService, UpdatableTaxpayerService,
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

pub fn unauthorized(conn: &PgConnection) -> Result<Vec<ViewableTaxpayerService>, Error> {
    Ok(taxpayers_services_view::table
        .filter(taxpayers_services_view::allowed_at.is_null())
        .load(conn)?)
}

pub fn authorize(
    conn: &PgConnection,
    taxpayer_service: &InsertableTaxpayerService,
) -> Result<Option<QueryableTaxpayerService>, Error> {
    let taxpayer_service = diesel::update(
        taxpayers_services::table
            .filter(taxpayers_services::taxpayer_id.eq(taxpayer_service.taxpayer_id))
            .filter(taxpayers_services::service_id.eq(taxpayer_service.service_id)),
    )
    .set(taxpayers_services::allowed_at.eq(dsl::now))
    .get_result(conn)?;
    Ok(Some(taxpayer_service))
}

pub fn unauthorize(
    conn: &PgConnection,
    id: i64,
    taxpayer_service: &mut UpdatableTaxpayerService,
) -> Result<Option<QueryableTaxpayerService>, Error> {
    taxpayer_service.allowed_at = None;
    let taxpayer_service =
        diesel::update(taxpayers_services::table.filter(taxpayers_services::id.eq(id)))
            .set(&*taxpayer_service)
            .get_result(conn)?;
    Ok(Some(taxpayer_service))
}
