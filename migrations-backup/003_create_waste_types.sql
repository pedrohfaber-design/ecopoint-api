-- ============================================================
-- EcoPoint
-- Migration: 003_create_waste_types.sql
-- ============================================================

CREATE TABLE IF NOT EXISTS waste_types (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    name VARCHAR(100) NOT NULL UNIQUE,

    description TEXT,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

INSERT INTO waste_types (name, description)
VALUES
('Computador', 'Desktop completo'),
('Notebook', 'Notebook usado'),
('Celular', 'Smartphone'),
('Tablet', 'Tablet'),
('Monitor', 'Monitor LCD ou LED'),
('Televisão', 'TV'),
('Impressora', 'Impressora'),
('Teclado', 'Teclado'),
('Mouse', 'Mouse'),
('Cabos', 'Cabos diversos'),
('Bateria', 'Baterias'),
('Pilha', 'Pilhas')
ON CONFLICT (name) DO NOTHING;