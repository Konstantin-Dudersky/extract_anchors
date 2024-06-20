//! Компонент для взаимодействия с InfluxDB

mod component;
mod config;
mod error;
mod fn_process;

pub use component::Cmp;
pub use config::{Config, LineProtocolItem, ValueType};
pub use error::Error;

type Result<T> = std::result::Result<T, Error>;

// TODO - после выхода InfluxDB 3.0 (май 2024) пересмотреть
