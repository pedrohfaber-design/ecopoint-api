use anyhow::{anyhow, Result};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use chrono::{Duration, Utc};
use rand::Rng;
use sqlx::PgPool;

use crate::{
    repositories::{
        password_reset_repository::PasswordResetRepository,
        user_repository::UserRepository,
    },
    services::email_service::EmailService,
};

pub struct PasswordResetService;

impl PasswordResetService {
    /// Cria um token de recuperação e envia o e-mail
    pub async fn create_reset_token(
        pool: &PgPool,
        email: &str,
    ) -> Result<Option<String>> {
        // Procura o usuário pelo e-mail
        let user = match UserRepository::find_by_email(pool, email).await? {
            Some(user) => user,
            None => {
                // Não revela se o e-mail existe ou não
                return Ok(None);
            }
        };

        // Gera token aleatório
        let token = {
            let mut rng = rand::rng();

let bytes: [u8; 32] = rng.random();

            bytes
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<String>()
        };

        // Token válido por 30 minutos
        let expires_at = Utc::now() + Duration::minutes(30);

        // Salva token no banco
        PasswordResetRepository::create(
            pool,
            user.id,
            &token,
            expires_at,
        )
        .await?;

        // Monta link de recuperação
        let reset_url = format!(
            "http://localhost:5173/reset-password?token={}",
            token
        );

        let body = format!(
            "Olá, {}!\n\n\
             Recebemos uma solicitação para redefinir sua senha no EcoPoint.\n\n\
             Para criar uma nova senha, acesse o link abaixo:\n\n\
             {}\n\n\
             Este link é válido por 30 minutos.\n\n\
             Se você não solicitou a redefinição de senha, ignore este e-mail.\n\n\
             Atenciosamente,\n\
             Equipe EcoPoint",
            user.name,
            reset_url
        );

        // Envia e-mail
        EmailService::send(
            &user.email,
            "Recuperação de senha - EcoPoint",
            &body,
        )
        .await?;

        Ok(Some(token))
    }

    /// Redefine a senha utilizando o token
    pub async fn reset_password(
        pool: &PgPool,
        token: &str,
        password: &str,
        password_confirmation: &str,
    ) -> Result<()> {
        // Confirmação da senha
        if password != password_confirmation {
            return Err(anyhow!("As senhas não coincidem."));
        }

        // Validação básica
        if password.len() < 6 {
            return Err(anyhow!(
                "A senha deve possuir pelo menos 6 caracteres."
            ));
        }

        // Procura token válido
        let reset_data =
            PasswordResetRepository::find_valid_token(pool, token)
                .await?
                .ok_or_else(|| {
                    anyhow!(
                        "Token inválido, expirado ou já utilizado."
                    )
                })?;

        let (_token_id, user_id) = reset_data;

        // Gera novo hash da senha
        let salt = SaltString::generate(&mut OsRng);

        let password_hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|err| {
                anyhow!("Erro ao gerar hash da senha: {}", err)
            })?
            .to_string();

        // Atualiza senha
        UserRepository::update_password(
            pool,
            user_id,
            &password_hash,
        )
        .await?;

        // Marca token como utilizado
        PasswordResetRepository::mark_as_used(
            pool,
            token,
        )
        .await?;

        Ok(())
    }
}