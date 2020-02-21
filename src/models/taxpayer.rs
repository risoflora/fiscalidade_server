use chrono::NaiveDateTime;

use crate::schema::fiscalidade_taxpayers as taxpayers;

#[derive(Serialize, Queryable)]
pub struct QueryableTaxpayer {
    pub id: i64,
    pub name: String,
    pub business_name: String,
    pub registry: String,
    pub email: String,
    pub certificate: String,
    pub certificate_password: String,
    pub token: String,
    pub manager: bool,
    pub active: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Deserialize, Insertable)]
#[table_name = "taxpayers"]
pub struct InsertableTaxpayer {
    pub name: String,
    pub business_name: String,
    pub registry: String,
    pub email: String,
    pub certificate: String,
    pub certificate_password: String,
    #[serde(skip_deserializing)]
    pub token: String,
}

#[derive(Deserialize, AsChangeset)]
#[table_name = "taxpayers"]
pub struct UpdatableTaxpayer {
    pub name: String,
    pub business_name: String,
    pub registry: String,
    pub email: String,
    pub certificate: String,
    pub certificate_password: String,
    pub manager: bool,
    pub active: bool,
}
