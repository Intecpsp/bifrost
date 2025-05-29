use thiserror::Error;

#[derive(Error, Debug)]
pub enum WledError {
    #[error("No MAC address in WLED response")]
    MissingMac,
}
