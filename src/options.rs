use std::process;

use anyhow::anyhow;
use getopts::Options as GetOptsOptions;

use crate::args::Args;
use crate::utils::Info;

fn print_help(program: &str, opts: &GetOptsOptions) {
    let info = Info::new();
    let brief = format!(
        "{} v{} ({}-{})\n\nCopyright (c) {}\n\nUso: {} [opções]",
        info.long_name, info.version, info.os, info.arch, info.authors, program
    );
    println!(
        "{}",
        opts.usage_with_format(|opts| {
            format!(
                "{}\n\nOpções:\n{}\n",
                brief,
                opts.collect::<Vec<String>>().join("\n")
            )
        })
    );
    process::exit(0);
}

fn print_version() {
    println!("{}", Info::new());
    process::exit(0);
}

pub struct Options {
    pub port: u16,
    pub database: String,
    #[cfg(not(feature = "embed_webservices"))]
    pub webservices: String,
    pub migrations: bool,
    pub silent: bool,
}

impl Options {
    pub fn from_args() -> anyhow::Result<Options> {
        let args = Args::new();
        let mut opts = GetOptsOptions::new();
        opts.optflag("h", "help", "Imprime este menu de ajuda");
        opts.optflag("v", "version", "Imprime versão da aplicação");
        opts.reqopt("p", "port", "Porta do servidor", "8000");
        opts.reqopt(
            "d",
            "database",
            "Banco de dados",
            "postgres://postgres:postgres@localhost/postgres",
        );
        #[cfg(not(feature = "embed_webservices"))]
        opts.reqopt(
            "w",
            "webservices",
            "Arquivo de webservices",
            "resources/webservices.ini",
        );
        opts.optflag("m", "migrations", "Cria ou atualiza o DB");
        opts.optflag("s", "silent", "Desativa informações de log");
        if args.len() <= 1 {
            print_help(args.program(), &opts);
        }
        if args.has('h', "help") {
            print_help(args.program(), &opts);
        }
        if args.has('v', "version") {
            print_version();
        }
        let args = opts.parse(args.args())?;
        Ok(Options {
            port: match args.opt_get("p") {
                Ok(arg) => arg.unwrap(),
                Err(_) => return Err(anyhow!("Invalid port")),
            },
            database: args.opt_get("d")?.unwrap(),
            #[cfg(not(feature = "embed_webservices"))]
            webservices: args.opt_get("w")?.unwrap(),
            migrations: args.opt_present("m"),
            silent: args.opt_present("s"),
        })
    }
}
