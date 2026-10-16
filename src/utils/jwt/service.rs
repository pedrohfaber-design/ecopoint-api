use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{anyhow, Result};
use jsonwebtoken::{
    decode,
    encode,
    DecodingKey,
    EncodingKey,
    Header,
    Validation,
};

use crate::{
    config::settings::Settings,
    utils::jwt::claims::Claims,
};

pub struct JwtService;

impl JwtService {
    /// Gera um token válido por 24 horas
    pub fn generate(
        user_id: &str,
        role: &str,
        settings: &Settings,
    ) -> Result<String> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs();

        let claims = Claims {
            sub: user_id.to_string(),
            role: role.to_string(),
            exp: (now + 60 * 60 * 24) as usize,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(settings.jwt_secret.as_bytes()),
        )?;

        Ok(token)
    }

    /// Valida um token
    pub fn verify(
        token: &str,
        settings: &Settings,
    ) -> Result<Claims> {
        let data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(settings.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| anyhow!("Token inválido"))?;

        Ok(data.claims)
    }
}