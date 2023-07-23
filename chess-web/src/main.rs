use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use axum::routing;
use http::Request;
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
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .finish()
        .with(tracing_enabled::GlobalEnable)
        .init();

    tracing::info!("starting web server at: {}", args.server_addr);

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
                .layer(RequestLogging)
                .into_make_service(),
        )
        .with_graceful_shutdown(async move {
            let _ = recv_shutdown.await;
            tracing::info!("shutting down web server at: {}", args.server_addr);
        })
        .await?;

    Ok(())
}

#[derive(Clone, Copy)]
pub struct RequestLogging;
#[derive(Clone, Copy)]
pub struct RequestLoggingService<S>(S);

pin_project_lite::pin_project! {
    pub struct RequestLoggingFuture<F> {
        first: bool,
        uri: String,
        version: http::Version,
        method: String,
        #[pin]
        inner: F,
    }
}

impl<S> tower_layer::Layer<S> for RequestLogging {
    type Service = RequestLoggingService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RequestLoggingService(inner)
    }
}

impl<F: std::future::Future<Output = Result<http::Response<Response>, Error>>, Response, Error>
    std::future::Future for RequestLoggingFuture<F>
{
    type Output = F::Output;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let req = self.project();

        if core::mem::take(req.first) {
            tracing::trace!(
                req.method,
                ?req.version,
                req.uri,
                "recieved request"
            );
        }

        match req.inner.poll(cx) {
            std::task::Poll::Pending => std::task::Poll::Pending,
            std::task::Poll::Ready(Ok(resp)) => {
                tracing::trace!(
                    req.method,
                    ?req.version,
                    req.uri,
                    resp.status=%resp.status(),
                    "served request"
                );

                std::task::Poll::Ready(Ok(resp))
            }
            std::task::Poll::Ready(Err(error)) => {
                tracing::trace!(
                    req.method,
                    ?req.version,
                    req.uri,
                    "failed to serve request"
                );

                std::task::Poll::Ready(Err(error))
            }
        }
    }
}

impl<S, T, U> tower_service::Service<Request<T>> for RequestLoggingService<S>
where
    S: tower_service::Service<Request<T>, Response = http::Response<U>>,
{
    type Response = http::Response<U>;
    type Error = S::Error;
    type Future = RequestLoggingFuture<S::Future>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.0.poll_ready(cx)
    }

    fn call(&mut self, req: Request<T>) -> Self::Future {
        RequestLoggingFuture {
            first: true,
            method: req.method().to_string(),
            version: req.version(),
            uri: req.uri().to_string(),
            inner: self.0.call(req),
        }
    }
}
