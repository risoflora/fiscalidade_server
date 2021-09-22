use std::{env, result::Result};

use getopts::{Fail, Matches, Options as OptsOptions};
use thiserror::Error;

use crate::{config::config_dir, consts, options::Options};

#[derive(Debug, Error)]
pub enum ArgsError {
    #[error(transparent)]
    GetOpts(#[from] Fail),
}

pub struct Args(Options);

pub type ArgsResult = Result<Options, ArgsError>;

impl Args {
    #[inline]
    fn new(config_file: &str) -> OptsOptions {
        let mut opts = OptsOptions::new();
        opts.optopt("c", "config", "Configuration file", config_file)
            .optflag("v", "version", "Print program version")
            .optflag("h", "help", "Print this help menu");
        opts
    }

    #[inline]
    fn parse_config_file(config_file: String, matches: &Matches) -> Option<String> {
        let option = matches.opt_get("c").unwrap_or_default();
        Some(option.unwrap_or(config_file))
    }

    #[inline]
    fn parse_version(matches: &Matches) -> Option<String> {
        if matches.opt_present("v") {
            Some(consts::APP_VERSION.to_owned())
        } else {
            None
        }
    }

    #[inline]
    fn parse_help(opts: &OptsOptions, matches: &Matches) -> Option<String> {
        if matches.opt_present("h") {
            Some(opts.usage(&format!("Usage: {}", consts::APP_NAME)))
        } else {
            None
        }
    }

    pub fn parse(args: &[String]) -> ArgsResult {
        let config_file = config_dir().unwrap_or_default().display().to_string();
        let opts = Self::new(&config_file);
        let matches = opts.parse(args)?;
        let args = Self {
            0: Options {
                config_file: Self::parse_config_file(config_file, &matches),
                version: Self::parse_version(&matches),
                help: Self::parse_help(&opts, &matches),
            },
        };
        Ok(args.0)
    }

    pub fn parse_from_env() -> ArgsResult {
        let args: Vec<String> = env::args().collect();
        Self::parse(&args[1..])
    }
}
