use std::net::SocketAddr;

use clap::Parser;
use yxy_httpd::router;

#[tokio::main]
async fn main() {
    // Parse args
    let args = Args::parse();
    let addr = args.bind;

    // Init global logger
    tracing_subscriber::fmt::init();

    let app = router::init();

    tracing::info!("Listening on: {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Parser)]
#[clap(author, version, about)]
struct Args {
    /// HTTPd binding address
    #[clap(short, long, default_value = "127.0.0.1:3000")]
    bind: SocketAddr,
}
