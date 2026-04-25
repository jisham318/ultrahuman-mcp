//! Ultrahuman MCP server over stdio.

use anyhow::Result;
use rmcp::{ServiceExt, handler::server::wrapper::Parameters, tool, tool_router, transport::stdio};

mod config;
mod ultrahuman;

use config::Config;
use ultrahuman::{
    DailyMetricsRequest, DailyMetricsResponse, DefaultDailyMetricsRequest, UltrahumanClient,
};

#[derive(Debug, Clone)]
struct UltrahumanMcp {
    ultrahuman: UltrahumanClient,
}

impl UltrahumanMcp {
    fn new(config: Config) -> Self {
        Self {
            ultrahuman: UltrahumanClient::new(config),
        }
    }
}

#[tool_router(server_handler)]
impl UltrahumanMcp {
    #[tool(
        name = "get_daily_metrics",
        description = "Get complete Ultrahuman daily metrics for an authorized user email and date"
    )]
    async fn get_daily_metrics(
        &self,
        Parameters(DailyMetricsRequest { email, date }): Parameters<DailyMetricsRequest>,
    ) -> Result<rmcp::Json<DailyMetricsResponse>, String> {
        self.ultrahuman.daily_metrics(&email, &date).await
    }

    #[tool(
        name = "get_default_daily_metrics",
        description = "Get complete Ultrahuman daily metrics for ULTRAHUMAN_DEFAULT_EMAIL and date"
    )]
    async fn get_default_daily_metrics(
        &self,
        Parameters(DefaultDailyMetricsRequest { date }): Parameters<DefaultDailyMetricsRequest>,
    ) -> Result<rmcp::Json<DailyMetricsResponse>, String> {
        let email = self.ultrahuman.default_email()?;
        self.ultrahuman.daily_metrics(email, &date).await
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    tracing::info!("ultrahuman-mcp starting (stdio)");

    let service = UltrahumanMcp::new(Config::from_env()?)
        .serve(stdio())
        .await?;
    service.waiting().await?;

    Ok(())
}
