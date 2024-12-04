use serde::Deserialize;
use std::sync::Arc;

/// Represent the environment mode of the application
#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EnvMode {
    // Development mode
    Dev,
    // Production mode
    Prd,
    // Staging mode
    Stg,
}

impl From<&Arc<str>> for EnvMode {
    fn from(value: &Arc<str>) -> Self {
        match value.as_ref() {
            "dev" => EnvMode::Dev,
            "prd" => EnvMode::Prd,
            "stg" => EnvMode::Stg,
            _ => unreachable!("invalid environment mode"),
        }
    }
}

impl EnvMode {
    // Check if the mode is development
    pub fn is_dev(mode: &str) -> bool {
        mode == "dev"
    }

    // Check if the mode is production
    pub fn is_prd(mode: &str) -> bool {
        mode == "prd"
    }

    // Check if the mode is staging
    pub fn is_stg(mode: &str) -> bool {
        mode == "stg"
    }

    // Check if the mode is valid
    pub fn is_valid(mode: &str) -> bool {
        Self::is_dev(mode) || Self::is_prd(mode) || Self::is_stg(mode)
    }
}
