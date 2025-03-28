use std::sync::Arc;
use axum::{Router, routing::get};
use async_graphql_axum::GraphQL;

use kids_backend::util::constant::CFG;
use kids_backend::gql::{build_schema, giql};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let schema = build_schema().await;
    let app_state = Arc::new(AppState {});
    let app_router = Router::new()
        .route(
            CFG.get("GQL_URI").unwrap(),
            get(giql).post_service(GraphQL::new(schema)),
        )
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(format!(
        "{}:{}",
        CFG.get("ADDR").unwrap(),
        CFG.get("PORT").unwrap()
    ))
    .await
    .unwrap();
    tracing::info!(
        "odds gql-server: http://{}{}",
        listener.local_addr().unwrap(),
        CFG.get("GQL_URI").unwrap()
    );
    axum::serve(listener, app_router).await.unwrap()
}

struct AppState {}
