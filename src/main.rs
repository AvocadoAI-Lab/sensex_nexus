use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

use sensex_backend::routes::create_router;

#[tokio::main]
async fn main() {
    let app = create_router()
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any)
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("Server running on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
