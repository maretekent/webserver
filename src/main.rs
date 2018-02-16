extern crate webserver;
extern crate flexi_logger;
#[macro_use]
extern crate log;
use flexi_logger::{Logger,opt_format};
extern crate clap;

use std::process;
use clap::{Arg, App};
use std::path::{PathBuf};
use webserver::Config;
use webserver::server::Server;

fn main() {
    Logger::with_str("warn, webserver=debug, webserver=debug")
        .log_to_file()
        .directory("log_files")
        .format(opt_format)
        .start()
        .unwrap_or_else(|e|{panic!("Logger initialization failed with {}",e)});

    let matches = App::new("Weltraumschaf's Webserver")
        .version("1.0.0")
        .author("Sven Strittmatter <ich@weltraumschaf.de>")
        .about("A minimalistic HTTP server.")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .takes_value(true)
            .help("Location of configuration file in TOML format.")
            .required(true))
        .get_matches();

    let config_file = matches.value_of("config").expect("No config file given!");
    let config_file = PathBuf::from(config_file);

    let config = Config::from_file(&config_file).unwrap_or_else(|err| {
        println!("Problem reading config file {:?}: {}", config_file, err);
        process::exit(1);
    });

    info!("Starting web server ...");
    let server = Server::new(config);
    server.bind().unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(2);
    });
}

