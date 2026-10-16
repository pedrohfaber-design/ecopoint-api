-- ============================================================
-- EcoPoint
-- Migration: 001_create_users.sql
-- Descrição: Criação da tabela de usuários
-- ============================================================

-- Habilita a extensão para geração de UUIDs
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    name VARCHAR(100) NOT NULL,

    email VARCHAR(255) NOT NULL UNIQUE,

    password_hash TEXT NOT NULL,

    phone VARCHAR(20),

    city VARCHAR(100) NOT NULL,

    state CHAR(2) NOT NULL,

    role VARCHAR(20) NOT NULL DEFAULT 'User'
        CHECK (role IN ('Admin', 'User')),

   created_at TIMESTAMP NOT NULL DEFAULT NOW(),

updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Índices para melhorar pesquisas
CREATE INDEX IF NOT EXISTS idx_users_email
ON users(email);

CREATE INDEX IF NOT EXISTS idx_users_role
ON users(role);

CREATE INDEX IF NOT EXISTS idx_users_city
ON users(city);