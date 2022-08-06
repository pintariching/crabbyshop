use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = crabbyshop::create_app().await;

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
