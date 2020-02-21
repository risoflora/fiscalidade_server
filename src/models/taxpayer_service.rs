use chrono::NaiveDateTime;

use crate::schema::fiscalidade_taxpayers_services as taxpayers_services;

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

#[derive(Deserialize, AsChangeset)]
#[table_name = "taxpayers_services"]
pub struct UpdatableTaxpayerService {
    pub taxpayer_id: i64,
    pub service_id: i64,
    #[serde(skip_deserializing)]
    pub allowed_at: Option<NaiveDateTime>,
}
