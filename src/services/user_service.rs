use anyhow::{anyhow, Result};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    dto::{create_user::CreateUserDto, login_user::LoginUserDto},
    models::{user::User, user_role::UserRole},
    repositories::user_repository::UserRepository,
};

pub struct UserService;

impl UserService {
    /// Cadastra um novo usuário
    pub async fn register(pool: &PgPool, dto: CreateUserDto) -> Result<User> {
       let email = dto.email.trim().to_lowercase();

// Verifica se já existe um usuário com este e-mail
if UserRepository::find_by_email(pool, &email)
    .await?
    .is_some()
{
    return Err(anyhow!("E-mail já cadastrado."));
}

// Gera hash da senha
let salt = SaltString::generate(&mut OsRng);

let password_hash = Argon2::default()
    .hash_password(dto.password.as_bytes(), &salt)
    .map_err(|e| anyhow!("Erro ao gerar hash da senha: {}", e))?
    .to_string();

let user = User {
    id: Uuid::new_v4(),
    name: dto.name,
    email,
    password_hash,
    phone: None,
    city: dto.city,
    state: dto.state,
    role: UserRole::User,
    created_at: Utc::now(),
    updated_at: Utc::now(),
};
        // Salva no banco
        UserRepository::create(pool, &user).await?;

        Ok(user)
    }

    /// Realiza login
    pub async fn login(pool: &PgPool, dto: LoginUserDto) -> Result<User> {
        let user = UserRepository::find_by_email(pool, &dto.email)
            .await?
            .ok_or_else(|| anyhow!("Usuário não encontrado."))?;

        let parsed_hash =
            PasswordHash::new(&user.password_hash).map_err(|e| anyhow!("Hash inválido: {}", e))?;

        Argon2::default()
            .verify_password(dto.password.as_bytes(), &parsed_hash)
            .map_err(|_| anyhow!("Senha incorreta."))?;

        Ok(user)
    }

    /// Busca usuário pelo ID
    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<User> {
        let user = UserRepository::find_by_id(pool, id)
            .await?
            .ok_or_else(|| anyhow!("Usuário não encontrado."))?;

        Ok(user)
    }

    /// Lista todos os usuários
    pub async fn list(pool: &PgPool) -> Result<Vec<User>> {
        let users = UserRepository::find_all(pool).await?;

        Ok(users)
    }

    /// Exclui usuário
    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<()> {
        UserRepository::delete(pool, id).await?;

        Ok(())
    }
}
