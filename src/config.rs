use std::{fs, env};
use std::path::Path;
use log::{debug, warn};
use serde_derive::*;
use toml;
use lazy_static::lazy_static;

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
    pub mongo: MongoConfig,
    pub common: Common,
    pub mq: MQConfig,
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


#[derive(Clone, Debug, Deserialize)]
#[serde(default)]
pub struct Common {
    pub wsuri: String,
    pub user_name: String,
    pub password: String,
    pub broker: String,
}

impl Default for Common {
    fn default() -> Self {
        Self {
            wsuri: "ws://192.168.2.22:7988".to_string(),
            user_name: "".to_string(),
            password: "".to_string(),
            broker: "simnow".to_string(),
        }
    }
}


pub fn new_config() -> Config {
    let _args: Vec<String> = env::args().collect();
    let cfg: Config = parse_config_from_cli_args(&parse_cli_args());
    cfg
}


lazy_static! {
    pub static ref CONFIG: Config = new_config();
}
