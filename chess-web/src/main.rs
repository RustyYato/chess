use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use axum::routing::{self, post};
use tracing_subscriber::prelude::*;

#[derive(clap::Parser)]
pub struct Args {
    #[clap(long, env)]
    server_addr: SocketAddr,
}

#[tokio::main]
pub async fn main() -> eyre::Result<()> {
    dotenvy::dotenv()?;
    let args: Args = clap::Parser::parse();

    colorz::mode::set_coloring_mode_from_env();
    colorz_eyre::install()?;

    tracing_subscriber::fmt()
        .event_format(colorz_tracing::BasicEventFormat)
        .fmt_fields(colorz_tracing::BasicFieldFormatter)
        .with_writer(std::io::stdout)
        .finish()
        .with(tracing_enabled::GlobalEnable)
        .init();

    let (send_shutdown, recv_shutdown) = tokio::sync::oneshot::channel::<()>();

    let send_shutdown = Arc::new(Mutex::new(Some(send_shutdown)));

    axum::Server::bind(&args.server_addr)
        .serve(
            axum::Router::new()
                .route(
                    "/shutdown",
                    routing::get(|| async move {
                        let send_shutdown = send_shutdown.lock().unwrap().take();
                        if let Some(send_shutdown) = send_shutdown {
                            let _ = send_shutdown.send(());
                        }
                    }),
                )
                .into_make_service(),
        )
        .with_graceful_shutdown(async move {
            let _ = recv_shutdown.await;
        })
        .await?;

    Ok(())
}
