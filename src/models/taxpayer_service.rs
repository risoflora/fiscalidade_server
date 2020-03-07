use chrono::NaiveDateTime;
use rocket::request::FromFormValue;
use serde::{Deserialize, Serialize};

use crate::db::schema::fiscalidade_taxpayers_services_view as taxpayers_services_view;
use crate::models::taxpayer::QueryableTaxpayer;
use crate::schema::fiscalidade_taxpayers_services as taxpayers_services;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, FromFormValue)]
pub enum TaxpayerServiceStatus {
    Authorized,
    Unauthorized,
}

#[derive(Serialize, Queryable)]
pub struct QueryableTaxpayerService {
    pub id: i64,
    pub taxpayer_id: i64,
    pub service_id: i64,
    pub allowed_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
}

#[derive(Deserialize, Insertable)]
#[table_name = "taxpayers_services"]
pub struct InsertableTaxpayerService {
    pub taxpayer_id: i64,
    pub service_id: i64,
}

#[derive(Serialize, Queryable, Identifiable, Associations)]
#[table_name = "taxpayers_services_view"]
#[primary_key(taxpayer_id, service_id)]
#[belongs_to(QueryableTaxpayer, foreign_key = "taxpayer_id")]
pub struct ViewableTaxpayerService {
    pub id: i64,
    pub taxpayer_id: i64,
    pub taxpayer_name: String,
    pub service_id: i64,
    pub service_description: String,
    pub allowed_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
}
