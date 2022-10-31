#![forbid(unsafe_code)]
#![warn(clippy::nursery, clippy::pedantic)]

use axum::{routing::get, Json, Router};
use axum_extra::routing::SpaRouter;
use clap::Parser;
use std::net::SocketAddr;

/// Command line arguments for server.
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// Sets the port of the embedded webserver.
    #[clap(value_parser = clap::value_parser!(u16))]
    port: u16,
}

#[tokio::main]
async fn main() {
    // Command line parsing.
    let args = Args::parse();

    // API routes
    let api_routes = Router::new().route("/", get(move || async { "API route" }));

    // HTTP route
    let routes = Router::new()
        // Single page application server. Serves static files
        // and catches all routes not in "/assets" and serves
        // the "index.html" file instead.
        .merge(SpaRouter::new("/assets", "public/"))
        // Health check route for service managers.
        .route("/health", get(|| async { Json("OK") }))
        // Route for getting the server version as a json string.
        .route(
            "/version",
            get(|| async { Json(std::env!("CARGO_PKG_VERSION")) }),
        )
        // API routes under the `/api` path
        .nest("/api", api_routes);

    // Binding and running the HTTP server
    let socket = SocketAddr::from(([0, 0, 0, 0], args.port));
    axum::Server::bind(&socket)
        .serve(routes.into_make_service())
        .await
        .unwrap();
}
