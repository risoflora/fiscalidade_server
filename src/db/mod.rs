use diesel::PgConnection;
use thiserror::Error;

pub mod cache;
pub mod service;
pub mod taxpayer;
pub mod taxpayer_service;

#[database("db::conn")]
pub struct Conn(PgConnection);

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Diesel(#[from] diesel::result::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
