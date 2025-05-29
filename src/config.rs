use std::collections::HashMap;

use camino::Utf8Path;
use config::{Config, ConfigError, Value};

pub use bifrost_api::config::*;

pub fn parse(filename: &Utf8Path) -> Result<AppConfig, ConfigError> {
    let settings = Config::builder()
        .set_default("bifrost.state_file", "state.yaml")?
        .set_default("bifrost.cert_file", "cert.pem")?
        .set_default("bridge.http_port", 80)?
        .set_default("bridge.https_port", 443)?
        .set_default("bridge.entm_port", 2100)?
        .set_default("z2m", HashMap::<String, Value>::new())?
        .set_default("wled", HashMap::<String, Value>::new())?
        .add_source(config::File::with_name(filename.as_str()))
        .build()?;

    settings.try_deserialize()
}
