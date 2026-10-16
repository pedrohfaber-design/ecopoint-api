use std::env;

#[derive(Clone)]
pub struct Settings {
    pub database_url: String,
    pub host: String,
    pub port: u16,
    pub jwt_secret: String,
}

impl Settings {
    pub fn new() -> Self {
        dotenvy::dotenv().ok();

        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL não encontrada"),

            host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),

            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .expect("PORT inválida"),

            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET não encontrada"),
        }
    }
}
