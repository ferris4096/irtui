#![warn(clippy::pedantic)]
#![allow(
    clippy::cast_lossless,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation
)]

use std::{
    env,
    fs::{self, File},
    str::FromStr,
};

use tracing::{Level, info};

use crate::app::App;

pub mod app;
pub mod event;
pub mod pano;
pub mod roadtrip;
pub mod ui;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Init tracing logs
    let log_level = env::var("IRTUI_LOG_LEVEL")
        .ok()
        .and_then(|level| Level::from_str(&level).ok());

    let log_path = cfg_select! {
        windows => r"Local\irtui\logs\irtui.log", // Use raw string for windows path
        _ => ".local/share/irtui/log/irtui.log"
    };

    let log_path = env::home_dir().unwrap().join(log_path);

    if let Some(parent) = log_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let log_file = File::options().create(true).append(true).open(log_path)?;

    let subscriber = tracing_subscriber::fmt()
        .with_max_level(log_level)
        .with_ansi(false)
        .with_writer(log_file)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("Initializing terminal");
    let terminal = ratatui::init();
    info!("Lauching app");
    let result = App::with_default_term()?.run(terminal).await;
    info!("Exiting...");
    ratatui::restore();
    result
}
