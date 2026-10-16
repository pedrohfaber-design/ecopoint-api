# ♻️ EcoPoint

Sistema web para gerenciamento de descarte responsável de resíduos eletrônicos, conectando usuários, administradores e pontos de coleta em uma única plataforma.

O EcoPoint foi desenvolvido com foco em **logística reversa, sustentabilidade e gestão de solicitações de descarte eletrônico**, permitindo que usuários registrem descartes, acompanhem solicitações e mantenham comunicação com administradores.

---

## 📌 Sobre o projeto

O descarte inadequado de equipamentos eletrônicos pode causar impactos ambientais e riscos relacionados à contaminação do solo e da água.

O EcoPoint busca facilitar o processo de descarte correto por meio de uma plataforma que centraliza:

* Cadastro e autenticação de usuários
* Solicitações de descarte
* Pontos de coleta
* Tipos de resíduos eletrônicos
* Acompanhamento de solicitações
* Comunicação entre usuário e administrador
* Gerenciamento administrativo
* Recuperação de senha por e-mail

O sistema possui duas áreas principais:

* **Área do Usuário**
* **Área Administrativa**

---

## 🚀 Funcionalidades

### 👤 Usuário

* Cadastro de usuário
* Login
* Logout
* Recuperação de senha
* Redefinição de senha por e-mail
* Dashboard pessoal
* Consulta de ecopontos
* Criação de solicitações de descarte
* Consulta das próprias solicitações
* Visualização dos detalhes de uma solicitação
* Acompanhamento do status da solicitação
* Chat com o administrador
* Visualização dos tipos de resíduos

### 🛠️ Administrador

* Dashboard administrativo
* Visualização das solicitações
* Visualização dos detalhes das solicitações
* Atualização do status das solicitações
* Gerenciamento de ecopontos
* Gerenciamento de tipos de resíduos
* Comunicação com usuários através do chat
* Controle de acesso baseado em função

### 🔐 Segurança

* Autenticação baseada em JWT
* Senhas armazenadas utilizando hash com Argon2
* Controle de acesso por função
* Proteção de rotas administrativas
* Tokens para recuperação de senha
* Expiração dos tokens de recuperação
* Variáveis sensíveis armazenadas através de `.env`
* Comunicação de recuperação de senha através de SMTP

---

## 🏗️ Arquitetura

O projeto é dividido em duas aplicações:

```text
EcoPoint
│
├── ecopoint
│   ├── Backend
│   ├── API REST
│   ├── Banco de dados
│   └── Autenticação
│
└── ecopoint-web
    ├── Frontend
    ├── Interface do usuário
    └── Interface administrativa
```

### Backend

O backend foi desenvolvido seguindo uma organização baseada em:

```text
src/
├── app_state/
├── config/
├── database/
├── dto/
├── handlers/
├── models/
├── repositories/
├── routes/
├── services/
└── utils/
```

A aplicação utiliza uma separação entre:

* Handlers
* Services
* Repositories
* Models
* DTOs
* Routes

Essa estrutura facilita a manutenção e evolução do sistema.

---

## 🧰 Tecnologias

### Backend

* Rust
* Axum
* Tokio
* SQLx
* PostgreSQL
* Serde
* JSON
* JWT
* Argon2
* Lettre
* Chrono
* UUID
* dotenvy
* Tracing

### Frontend

* React
* TypeScript
* Vite
* React Router
* Tailwind CSS
* Lucide React

### Banco de dados

* PostgreSQL

---

## 🗄️ Banco de dados

O sistema utiliza PostgreSQL para armazenamento das principais entidades da aplicação.

Entre as estruturas utilizadas estão:

* Usuários
* Pontos de coleta
* Tipos de resíduos
* Relação entre pontos de coleta e resíduos
* Solicitações de descarte
* Itens de descarte
* Mensagens do chat
* Tokens de recuperação de senha

As alterações do banco são controladas através de migrations utilizando SQLx.

---

## 🔑 Autenticação

O EcoPoint utiliza JWT para autenticação dos usuários.

Após o login, o backend gera um token contendo as informações necessárias para identificação do usuário e sua função no sistema.

Existem dois perfis:

```text
User
Admin
```

As rotas administrativas possuem proteção específica para impedir que usuários comuns tenham acesso às funcionalidades administrativas.

---

## ♻️ Fluxo de descarte

O fluxo principal do sistema funciona da seguinte maneira:

```text
Usuário
   │
   ▼
Seleciona um ponto de coleta
   │
   ▼
Seleciona os resíduos
   │
   ▼
Cria solicitação
   │
   ▼
Administrador recebe solicitação
   │
   ▼
Administrador analisa
   │
   ├── Aprova
   │
   ├── Cancela
   │
   └── Atualiza
   │
   ▼
Usuário acompanha o status
   │
   ▼
Usuário e administrador
podem conversar pelo chat
```

---

## 💬 Sistema de comunicação

O EcoPoint possui um sistema de mensagens vinculado às solicitações de descarte.

Isso permite que:

* O usuário envie dúvidas ao administrador
* O administrador responda à solicitação
* A comunicação fique associada ao descarte
* Usuário e administrador acompanhem o histórico da conversa

---

## 📧 Recuperação de senha

O sistema possui fluxo completo de recuperação de senha:

```text
Usuário informa o e-mail
        │
        ▼
Sistema verifica o usuário
        │
        ▼
Gera token de recuperação
        │
        ▼
Token possui validade
        │
        ▼
E-mail é enviado
        │
        ▼
Usuário acessa o link
        │
        ▼
Define nova senha
        │
        ▼
Senha atualizada
```

O envio de e-mails é realizado através de SMTP.

---

## ⚙️ Como executar o projeto

### Pré-requisitos

Antes de executar o projeto, instale:

* Rust
* Cargo
* PostgreSQL
* Node.js
* npm

Também é necessário possuir um banco PostgreSQL disponível.

---

## 🦀 Executando o Backend

Clone o repositório:

```bash
git clone SEU_REPOSITORIO
```

Entre na pasta:

```bash
cd ecopoint
```

Configure o arquivo:

```text
.env
```

Exemplo:

```env
DATABASE_URL=postgres://postgres:SUA_SENHA@localhost:5432/ecopoint

HOST=127.0.0.1
PORT=3000

JWT_SECRET=SUA_CHAVE_JWT

SMTP_HOST=smtp.gmail.com
SMTP_PORT=587
SMTP_USERNAME=seu_email@gmail.com
SMTP_PASSWORD=SUA_SENHA_DE_APP
SMTP_FROM=seu_email@gmail.com
```

Crie o banco:

```sql
CREATE DATABASE ecopoint;
```

Execute as migrations:

```bash
sqlx migrate run
```

Verifique o estado:

```bash
sqlx migrate info
```

Execute o backend:

```bash
cargo run
```

O backend será iniciado em:

```text
http://127.0.0.1:3000
```

---

## ⚛️ Executando o Frontend

Entre na pasta:

```bash
cd ecopoint-web
```

Instale as dependências:

```bash
npm install
```

Execute o projeto:

```bash
npm run dev
```

O frontend será disponibilizado pelo Vite, normalmente em:

```text
http://localhost:5173
```

---

## 🧪 Build de produção

### Backend

```bash
cargo build --release
```

### Frontend

```bash
npm run build
```

---

## 📁 Estrutura do projeto

```text
EcoPoint/
│
├── ecopoint/
│   │
│   ├── migrations/
│   │
│   ├── src/
│   │   ├── app_state/
│   │   ├── config/
│   │   ├── database/
│   │   ├── dto/
│   │   ├── handlers/
│   │   ├── models/
│   │   ├── repositories/
│   │   ├── routes/
│   │   ├── services/
│   │   └── utils/
│   │
│   ├── Cargo.toml
│   ├── Cargo.lock
│   └── .env.example
│
└── ecopoint-web/
    │
    ├── src/
    │   ├── components/
    │   ├── context/
    │   ├── layouts/
    │   ├── pages/
    │   ├── routes/
    │   └── services/
    │
    ├── package.json
    ├── package-lock.json
    └── .env.example
```

---

## 🌱 Objetivo

O EcoPoint tem como objetivo contribuir para a conscientização e facilitar o processo de descarte correto de resíduos eletrônicos, aproximando usuários dos pontos de coleta e proporcionando uma ferramenta de gerenciamento para os responsáveis pela operação.

---

## 📚 Contexto acadêmico

O projeto também foi desenvolvido como aplicação prática de conceitos relacionados a:

* Desenvolvimento de sistemas
* Engenharia de software
* Desenvolvimento Web
* Banco de dados
* APIs REST
* Segurança da informação
* Autenticação
* Arquitetura de software
* Sustentabilidade
* Logística reversa

---

## 👨‍💻 Autor

**Pedro Henrique Faber**

Desenvolvedor em formação com interesse em desenvolvimento de software, backend, sistemas e segurança da informação.

### Contato

* LinkedIn: [Pedro Faber](https://www.linkedin.com/in/pedro-faber-a52a25252)
* GitHub: [pedrohfaber-design](https://github.com/pedrohfaber-design)
* Instagram: [@faberzx](https://www.instagram.com/faberzx/)

---

## 📄 Licença

Este projeto foi desenvolvido para fins acadêmicos e de portfólio.

Caso o projeto seja disponibilizado publicamente, consulte as condições de uso antes de reutilizar seu código.
