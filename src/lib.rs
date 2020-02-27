#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use std::{collections::HashMap, env, fs::File, path::Path};

use anyhow::anyhow;
use diesel::{Connection, PgConnection};
use fiscalidade::WebServices;
use log::error;
use rocket::{
    catch, catchers,
    config::{Config as RocketConfig, Environment, Limits, LoggingLevel, Value},
    routes,
};
use rocket_contrib::json::JsonValue;
use simplelog::{Config as LogConfig, LevelFilter, WriteLogger};

#[macro_use]
mod utils;

mod args;
mod config;
mod daemon;
mod db;
mod models;
mod options;
mod routes;
mod schema;

use crate::args::Args;
use crate::config::Config;
use crate::db::Conn;
use crate::options::Options;
use crate::routes::{cache, nfe, services, taxpayer, taxpayer_service};

#[derive(Clone)]
pub struct AppData {
    pub webservices: WebServices,
}

pub struct AppProps {
    pub port: u16,
    pub database: String,
    #[cfg(not(feature = "embed_webservices"))]
    pub webservices: String,
    pub install: bool,
    pub uninstall: bool,
    pub silent: bool,
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
    let args = Args::new();
    let opts: AppProps = if args.len() > 1 {
        Options::from_args(args)?.into()
    } else {
        Config::from_file(Config::filename())?.into()
    };
    if !opts.silent {
        WriteLogger::init(
            LevelFilter::Warn,
            LogConfig::default(),
            File::create(
                Path::new(&env::current_exe()?.display().to_string()).with_extension("log"),
            )?,
        )?;
    }
    if opts.install {
        daemon::install()
    } else if opts.uninstall {
        daemon::uninstall()
    }
    let database = opts.database;
    let conn = match PgConnection::establish(&database) {
        Ok(conn) => conn,
        Err(error) => {
            error!("{}", error);
            return Err(error.into());
        }
    };
    if let Err(error) = embedded_migrations::run(&conn) {
        error!("{}", error);
        return Err(error.into());
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
    let config = RocketConfig::build(Environment::Production)
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
