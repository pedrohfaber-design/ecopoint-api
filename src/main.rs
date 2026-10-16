mod app_state;
mod config;
mod database;
mod dto;
mod errors;
mod extractors;
mod handlers;
mod middleware;
mod models;
mod repositories;
mod routes;
mod services;
mod utils;

use axum::Router;
use tokio::net::TcpListener;
use tracing::info;
use axum::http::{header, HeaderValue, Method};
use tower_http::cors::CorsLayer;

use crate::{
    app_state::AppState,
    config::settings::Settings,
    database::connection::{create_pool, test_connection},
};

#[tokio::main]
async fn main() {
    // Inicializa logs
    tracing_subscriber::fmt::init();

    // Carrega configurações
    let settings = Settings::new();

    // Cria o pool
    let pool = create_pool(&settings)
        .await
        .expect("Erro ao conectar ao PostgreSQL");

    // Testa conexão
    test_connection(&pool)
        .await
        .expect("Falha ao conectar ao PostgreSQL");
    database::migrations::run_migrations(&pool)
        .await
        .expect("Erro ao executar migrations");

    // Estado compartilhado
    let state = AppState::new(pool, settings.clone());

let cors = CorsLayer::new()
    .allow_origin(
        "http://localhost:5173"
            .parse::<HeaderValue>()
            .unwrap(),
    )
    .allow_methods([
        Method::GET,
        Method::POST,
        Method::PUT,
        Method::PATCH,
        Method::DELETE,
        Method::OPTIONS,
    ])
    .allow_headers([
        header::CONTENT_TYPE,
        header::AUTHORIZATION,
    ]);

    // Rotas
    let app = Router::new()
        .merge(routes::public::router())
        .nest("/api/users", routes::user_routes::router())
        .nest(
            "/api/collection-points",
            routes::collection_point_routes::router(),
        )
        .nest(
            "/api/disposal-requests",
            routes::disposal_request_routes::router(),
        )
        .nest(
    "/api/chat/messages",
    routes::chat_message_routes::router(),
)
        .nest("/api/dashboard", routes::dashboard_routes::router())
        .nest("/api/waste-types", routes::waste_type_routes::router())
        .nest(
            "/api/user-dashboard",
            routes::user_dashboard_routes::router(),
        )
         .layer(cors)
        .with_state(state);

    let address = format!("{}:{}", settings.host, settings.port);

    let listener = TcpListener::bind(&address)
        .await
        .expect("Erro ao iniciar servidor");

    info!("🚀 EcoPoint iniciado em http://{}", address);

    axum::serve(listener, app)
        .await
        .expect("Erro ao executar servidor");
}
