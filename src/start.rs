use std::{fs, path::PathBuf};

use axum::{middleware, Extension};

use crate::{delay_response, routing::config::Config};

pub async fn start(config: PathBuf) {
    let config = fs::read_to_string(config).expect("Unable to read config");
    let config: Config = serde_yaml::from_str(&config).unwrap();
    let options = config.clone().options;
    let router = config
        .router()
        .layer(middleware::from_fn_with_state(
            options.clone(),
            delay_response,
        ))
        .layer(Extension(options.clone()));

    match tokio::net::TcpListener::bind(format!(
        "{}:{}",
        options.clone().address,
        options.clone().port
    ))
    .await
    {
        Ok(listener) => {
            log::info!(
                "Starting server on: {}:{}",
                options.clone().address,
                options.clone().port
            );

            if let Err(e) = axum::serve(listener, router).await {
                log::error!("Failed to start server - {e}");
            }
        }
        Err(e) => {
            log::error!(
                "Failed to bind to: {}:{} - {e}",
                options.clone().address,
                options.clone().port
            );
            std::process::exit(1);
        }
    }
}
