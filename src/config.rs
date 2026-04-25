use anyhow::{Context, Result};

const DEFAULT_BASE_URL: &str = "https://partner.ultrahuman.com/api/v1";

#[derive(Debug, Clone)]
pub struct Config {
    pub authorization: String,
    pub base_url: String,
    pub default_email: Option<String>,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let authorization = read_first_env(&[
            "ULTRAHUMAN_AUTHORIZATION",
            "ULTRAHUMAN_API_TOKEN",
            "ULTRAHUMAN_AUTH_KEY",
        ])
        .context(
            "missing Ultrahuman credentials; set ULTRAHUMAN_AUTHORIZATION, \
             ULTRAHUMAN_API_TOKEN, or ULTRAHUMAN_AUTH_KEY",
        )?;

        let base_url = std::env::var("ULTRAHUMAN_BASE_URL")
            .ok()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| DEFAULT_BASE_URL.to_string());

        let default_email = std::env::var("ULTRAHUMAN_DEFAULT_EMAIL")
            .ok()
            .filter(|value| !value.trim().is_empty());

        Ok(Self {
            authorization,
            base_url,
            default_email,
        })
    }
}

fn read_first_env(names: &[&str]) -> Option<String> {
    names.iter().find_map(|name| {
        std::env::var(name)
            .ok()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty())
    })
}
