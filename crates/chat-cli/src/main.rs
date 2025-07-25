pub mod app;
pub mod config;
pub mod event;
pub mod ui;

use tracing_error::ErrorLayer;
use tracing_subscriber::{self, Layer, layer::SubscriberExt, util::SubscriberInitExt};

use crate::app::App;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let project_config = config::ProjectConfig::new()?;
    initialize_logging(&project_config)?;

    let terminal = ratatui::init();
    let result = App::new()?.run(terminal).await;
    ratatui::restore();

    result
}

fn initialize_logging(proj_config: &config::ProjectConfig) -> color_eyre::Result<()> {
    let data_dir = proj_config.data_dir();
    let log_file = std::fs::File::create(data_dir.join("streuen_chat.log"))?;
    let env_filter = tracing_subscriber::EnvFilter::builder()
        .with_default_directive(tracing_subscriber::filter::LevelFilter::INFO.into())
        .from_env_lossy();
    let file_subscriber = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_writer(log_file)
        .with_target(false)
        .with_ansi(false)
        .with_filter(env_filter);
    tracing_subscriber::registry()
        .with(file_subscriber)
        .with(ErrorLayer::default())
        .init();
    Ok(())
}
