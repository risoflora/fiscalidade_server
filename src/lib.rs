#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

extern crate anyhow;
extern crate chrono;
extern crate fiscalidade;
extern crate getopts;
extern crate nanoid;
extern crate serde;
extern crate serde_json;
extern crate thiserror;

use std::{collections::HashMap, io::stdout};

use anyhow::anyhow;
use diesel::{Connection, PgConnection};
use fiscalidade::WebServices;
use rocket::config::{Config, Environment, Limits, LoggingLevel, Value};
use rocket_contrib::json::JsonValue;

#[macro_use]
mod utils;

mod args;
mod db;
mod models;
mod options;
mod routes;
mod schema;

use crate::db::Conn;
use crate::options::Options;
use crate::routes::{cache, nfe, services, taxpayer, taxpayer_service};

#[derive(Clone)]
pub struct AppData {
    pub webservices: WebServices,
}

#[catch(400)]
fn bad_request() -> JsonValue {
    json_error!("Bad request")
}

#[catch(401)]
fn unauthorized() -> JsonValue {
    json_error!("Unauthorized")
}

#[catch(403)]
fn forbidden() -> JsonValue {
    json_error!("Forbidden")
}

#[catch(404)]
fn not_found() -> JsonValue {
    json_error!("Not found")
}

#[catch(422)]
fn unprocessable_entity() -> JsonValue {
    json_error!("Unprocessable entity")
}

#[catch(424)]
fn failed_dependency() -> JsonValue {
    json_error!("Failed dependency")
}

#[catch(500)]
fn internal_error() -> JsonValue {
    json_error!("Internal error")
}

#[catch(503)]
fn service_unavailable() -> JsonValue {
    json_error!("Service unavailable")
}

embed_migrations!();

pub fn rocket() -> anyhow::Result<rocket::Rocket> {
    let opts = Options::from_args()?;
    let database = opts.database;
    if opts.migrations {
        let conn = PgConnection::establish(&database)?;
        if opts.silent {
            embedded_migrations::run(&conn)?;
        } else {
            embedded_migrations::run_with_output(&conn, &mut stdout())?;
        }
    }
    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();
    database_config.insert("url", Value::from(database));
    databases.insert("db::conn", Value::from(database_config));
    #[cfg(feature = "embed_webservices")]
    let webservices = WebServices::from_embedded();
    #[cfg(not(feature = "embed_webservices"))]
    let webservices = WebServices::from_file(&opts.webservices);
    let webservices = match webservices {
        Ok(webservices) => webservices,
        Err(err) => return Err(anyhow!("Failed to load webservices file: {}", err)),
    };
    let limits = Limits::new().limit("forms", 512 * 1024);
    let config = Config::build(Environment::Production)
        .extra("databases", databases)
        .port(opts.port)
        .limits(limits)
        .keep_alive(16)
        .secret_key("dcvE9tKmPfmHIGkh8b2AUalwaNYnObZUyDWYjbiPQeo=")
        .log_level(if opts.silent {
            LoggingLevel::Off
        } else {
            LoggingLevel::Critical
        })
        .finalize()?;
    Ok(rocket::custom(config)
        .mount(
            mount_path!(),
            routes![
                routes::version,
                routes::info,
                cache::list,
                taxpayer::create_manager,
                taxpayer::create,
                taxpayer::update,
                taxpayer::delete,
                taxpayer::list,
                services::list,
                taxpayer_service::create,
                taxpayer_service::unauthorized,
                taxpayer_service::authorize,
                taxpayer_service::unauthorize
            ],
        )
        .mount(
            mount_path!("/nfe"),
            routes![
                nfe::status_servico,
                nfe::consultar_cadastro,
                nfe::consultar_xml
            ],
        )
        .attach(Conn::fairing())
        .manage(AppData {
            webservices: webservices,
        })
        .register(catchers![
            bad_request,
            unauthorized,
            forbidden,
            not_found,
            unprocessable_entity,
            failed_dependency,
            internal_error,
            service_unavailable
        ]))
}
