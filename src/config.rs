use std::{fs, env};
use std::path::Path;
use log::{debug, warn};
use serde_derive::*;
use toml;
use lazy_static::lazy_static;

extern crate clap;

use clap::{Arg, App, SubCommand};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Defines and parses CLI argument for this server.
pub fn parse_cli_args<'a>() -> clap::ArgMatches<'a> {
    clap::App::new("qaruntime-rs")
        .version(VERSION)
        .arg(
            clap::Arg::with_name("config")
                .required(false)
                .help("Path to configuration file")
                .index(1),
        )
        .get_matches()
}

/// Parses CLI arguments, finds location of config file, and parses config file into a struct.
pub fn parse_config_from_cli_args(matches: &clap::ArgMatches) -> Config {
    let conf = match matches.value_of("config") {
        Some(config_path) => match Config::from_file(config_path) {
            Ok(config) => config,
            Err(msg) => {
                eprintln!("Failed to parse config file {}: {}", config_path, msg);
                std::process::exit(1);
            }
        },
        None => {
            warn!("No config file specified, use default");
            Config::default()
        }
    };
    conf
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Config {
    pub common: Common,
}

impl Config {
    /// Read configuration from a file into a new Config struct.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let path = path.as_ref();
        debug!("Reading configuration from {}", path.display());

        let data = match fs::read_to_string(path) {
            Ok(data) => data,
            Err(err) => return Err(err.to_string()),
        };

        let conf: Config = match toml::from_str(&data) {
            Ok(conf) => conf,
            Err(err) => return Err(err.to_string()),
        };

        Ok(conf)
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(default)]
pub struct MongoConfig {
    pub uri: String,
    pub db: String,
}

impl Default for MongoConfig {
    fn default() -> Self {
        Self {
            uri: "mongodb://localhost:27017".to_owned(),
            db: "".to_owned(),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(default)]
pub struct MQConfig {
    pub uri: String,
    pub exchange: String,
    pub routing_key: String,
}

impl Default for MQConfig {
    fn default() -> Self {
        Self {
            uri: "amqp://admin:admin@localhost:5672/".to_owned(),
            exchange: "".to_owned(),
            routing_key: "".to_owned(),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(default)]
pub struct RedisConfig {
    pub uri: String,
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            uri: "localhost:6379".to_owned(),
        }
    }
}


#[derive(Clone, Debug, Deserialize, Default)]
#[serde(default)]
pub struct Common {
    pub log_level: String,
    pub account: String,
    pub password: String,
    pub broker: String,
    pub wsuri: String,
    pub eventmq_ip: String,
    pub database_ip: String,
    pub ping_gap: i32,
    pub taskid: String,
    pub portfolio: String,
    pub bank_password: String,
    pub capital_password: String,
    pub appid: String,
}


pub fn new_config() -> Config {
    let matches = App::new("QATrader")
        .version("1.0")
        .author("junefar")
        .about("Does awesome things")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("conf\\boot.toml")
            .help("toml文件获取配置")
            .takes_value(true))
        .arg(Arg::with_name("account")
            .long("account")
            .value_name("")
            .help("Set account name")
            .takes_value(true))
        .arg(Arg::with_name("password")
            .long("password")
            .value_name("")
            .help("Set password")
            .takes_value(true))
        .arg(Arg::with_name("wsuri")
            .long("wsuri")
            .value_name("ws://localhost:7988")
            .help("Set websocket uri")
            .takes_value(true))
        .arg(Arg::with_name("broker")
            .long("broker")
            .value_name("simnow")
            .help("Set broker")
            .takes_value(true))
        .arg(Arg::with_name("eventmq_ip")
            .long("eventmq_ip")
            .value_name("amqp://admin:admin@192.168.2.125:5672/")
            .help("接收发单MQ")
            .takes_value(true))
        .arg(Arg::with_name("database_ip")
            .long("database_ip")
            .value_name("mongodb://localhost:27017")
            .help("QIFI 数据库")
            .takes_value(true))
        .arg(Arg::with_name("ping_gap")
            .long("ping_gap")
            .value_name("5")
            .help("ping 间隔")
            .takes_value(true))
        .arg(Arg::with_name("taskid")
            .long("taskid")
            .value_name("")
            .help("Set taskid")
            .takes_value(true))
        .arg(Arg::with_name("portfolio")
            .long("portfolio")
            .value_name("default")
            .help("Set portfolio")
            .takes_value(true))
        .arg(Arg::with_name("bank_password")
            .long("bank_password")
            .value_name("")
            .help("银行密码")
            .takes_value(true))
        .arg(Arg::with_name("capital_password")
            .long("capital_password")
            .value_name("")
            .help("资金密码")
            .takes_value(true))
        .arg(Arg::with_name("appid")
            .long("appid")
            .value_name("")
            .help("Set app id")
            .takes_value(true))
        .arg(Arg::with_name("log_level")
            .long("log_level")
            .value_name("info")
            .help("日志等级[ debug / info / warn / error]")
            .takes_value(true))
        .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    if let Some(config_path) = matches.value_of("config") {
        match Config::from_file(config_path) {
            Ok(config) => config,
            Err(msg) => {
                eprintln!("Failed to parse config file {}: {}", config_path, msg);
                std::process::exit(1);
            }
        }
    } else {
        let account = matches.value_of("account").unwrap_or("").to_string();
        let password = matches.value_of("password").unwrap_or("").to_string();
        let wsuri = matches.value_of("wsuri").unwrap_or("ws://localhost:7988").to_string();
        let broker = matches.value_of("broker").unwrap_or("simnow").to_string();
        let eventmq_ip = matches.value_of("eventmq_ip").unwrap_or("").to_string();
        let database_ip = matches.value_of("database_ip").unwrap_or("").to_string();
        let ping_gap = matches.value_of("ping_gap").unwrap_or("5").parse::<i32>().unwrap();
        let taskid = matches.value_of("taskid").unwrap_or("").to_string();
        let portfolio = matches.value_of("portfolio").unwrap_or("default").to_string();
        let bank_password = matches.value_of("bank_password").unwrap_or("").to_string();
        let capital_password = matches.value_of("capital_password").unwrap_or("").to_string();
        let appid = matches.value_of("appid").unwrap_or("").to_string();
        let log_level = matches.value_of("log_level").unwrap_or("info").to_string();
        Config {
            common: Common {
                account,
                password,
                broker,
                wsuri,
                eventmq_ip,
                database_ip,
                ping_gap,
                taskid,
                portfolio,
                bank_password,
                capital_password,
                appid,
                log_level,
            }
        }
    }
}

lazy_static! {
    pub static ref CONFIG: Config = new_config();
}
