use rmcp::{Json, schemars};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::config::Config;

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DailyMetricsRequest {
    #[schemars(description = "Email address for the authorized Ultrahuman user")]
    pub email: String,
    #[schemars(description = "Date to fetch in YYYY-MM-DD format")]
    pub date: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DefaultDailyMetricsRequest {
    #[schemars(description = "Date to fetch in YYYY-MM-DD format")]
    pub date: String,
}

#[derive(Debug, Serialize, schemars::JsonSchema)]
pub struct DailyMetricsResponse {
    #[schemars(description = "Email address used for this Ultrahuman Daily API request")]
    pub email: String,
    #[schemars(description = "Date used for this Ultrahuman Daily API request")]
    pub date: String,
    #[schemars(description = "Raw JSON response returned by the Ultrahuman Daily API")]
    pub metrics: Value,
}

#[derive(Debug, Clone)]
pub struct UltrahumanClient {
    http: reqwest::Client,
    authorization: String,
    base_url: String,
    default_email: Option<String>,
}

impl UltrahumanClient {
    pub fn new(config: Config) -> Self {
        Self {
            http: reqwest::Client::new(),
            authorization: config.authorization,
            base_url: config.base_url.trim_end_matches('/').to_string(),
            default_email: config.default_email,
        }
    }

    pub fn default_email(&self) -> Result<&str, String> {
        self.default_email.as_deref().ok_or_else(|| {
            "ULTRAHUMAN_DEFAULT_EMAIL is not configured for default-user tools".to_string()
        })
    }

    pub async fn daily_metrics(
        &self,
        email: &str,
        date: &str,
    ) -> Result<Json<DailyMetricsResponse>, String> {
        validate_daily_date(date)?;

        let url = reqwest::Url::parse_with_params(
            &format!("{}/metrics", self.base_url),
            &[("email", email), ("date", date)],
        )
        .map_err(|error| format!("failed to build Ultrahuman Daily API URL: {error}"))?;

        tracing::debug!(%email, %date, "fetching Ultrahuman daily metrics");

        let response = self
            .http
            .get(url)
            .header(reqwest::header::AUTHORIZATION, &self.authorization)
            .send()
            .await
            .map_err(|error| format!("failed to call Ultrahuman Daily API: {error}"))?;

        let status = response.status();
        let body = response
            .text()
            .await
            .map_err(|error| format!("failed to read Ultrahuman Daily API response: {error}"))?;

        if !status.is_success() {
            return Err(format!(
                "Ultrahuman Daily API returned HTTP {status}: {}",
                truncate_for_error(&body)
            ));
        }

        serde_json::from_str(&body)
            .map(|metrics| {
                Json(DailyMetricsResponse {
                    email: email.to_string(),
                    date: date.to_string(),
                    metrics,
                })
            })
            .map_err(|error| format!("Ultrahuman Daily API returned invalid JSON: {error}"))
    }
}

fn validate_daily_date(date: &str) -> Result<(), String> {
    chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d")
        .map(|_| ())
        .map_err(|_| format!("invalid date '{date}'; expected YYYY-MM-DD"))
}

fn truncate_for_error(body: &str) -> String {
    const MAX_ERROR_BODY_BYTES: usize = 1_000;

    if body.len() <= MAX_ERROR_BODY_BYTES {
        return body.to_string();
    }

    let mut truncated = body.to_string();
    truncated.truncate(MAX_ERROR_BODY_BYTES);
    format!("{truncated}...")
}
