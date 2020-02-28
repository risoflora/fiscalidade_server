use diesel::PgConnection;
use rocket_contrib::database;
use thiserror::Error;

pub mod cache;
pub mod schema;
pub mod service;
pub mod taxpayer;
pub mod taxpayer_service;

#[database("postgres_pool")]
pub struct Conn(PgConnection);

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Diesel(#[from] diesel::result::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
