# API Rust 01

Este é um projeto de exemplo de uma API RESTful construída com a linguagem **Rust**, utilizando o framework **Axum**. O projeto demonstra como estruturar rotas, manipular dados JSON e gerar documentação automática com **Swagger UI** (via `utoipa`).

## 🚀 Tecnologias Utilizadas

- **Rust**: Linguagem de programação focada em performance e segurança.
- **Axum**: Framework web ergonômico e modular para Rust.
- **Tokio**: Runtime assíncrono para Rust.
- **Serde**: Framework para serialização e desserialização de dados.
- **MongoDB**: Banco de dados NoSQL utilizado para persistência dos dados.
- **Tower-HTTP**: Middleware utilizado para configuração de CORS.
- **Utoipa**: Gerador de documentação OpenAPI (Swagger) para Rust.

## 📋 Pré-requisitos

Antes de começar, certifique-se de ter o **Rust** e o **Cargo** instalados em sua máquina.

- Instalação do Rust verifique no site official: https://rust-lang.org

## 🛠️ Como Executar

1. **Navegue até a pasta do projeto**:
   ```bash
   cd api-rust-01
   ```

2. **Compile e execute o projeto**:
   ```bash
   cargo run
   ```

   O Cargo irá baixar as dependências, compilar o código e iniciar o servidor.

3. **Acesse a aplicação**:
   O servidor iniciará escutando no endereço `0.0.0.0:8000`.

## 📚 Documentação da API (Swagger UI)

A documentação interativa da API está disponível através do Swagger UI. Após iniciar o servidor, acesse:

👉 **http://localhost:8000/swagger-ui**

Aqui você poderá visualizar todos os endpoints disponíveis, seus esquemas de dados e testar as requisições diretamente pelo navegador.

## 🔌 Endpoints Disponíveis

### Geral
- `GET /`: Retorna uma mensagem de "Hello, World!".
- `GET /api/hello`: Retorna uma mensagem de boas-vindas da API.

### Usuários (`/api/users`)
- `POST /create_user`: Cria um novo usuário no MongoDB (recebe JSON, retorna JSON).
- `GET /:id`: Retorna detalhes simulados de um usuário.