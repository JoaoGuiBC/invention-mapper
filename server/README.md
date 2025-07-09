# Invention Mapper

Sistema de mapeamento de invenções desenvolvido em Rust com API REST, integração com IA e armazenamento em PostgreSQL.

## 🛠️ Tecnologias Utilizadas

### Backend
- **Rust** - Linguagem principal
- **Actix Web** - Framework web para APIs REST
- **SeaORM** - ORM para PostgreSQL
- **Tokio** - Runtime assíncrono
- **Ollama** - Integração com modelos de IA locais
- **Cloudinary** - Armazenamento de imagens
- **Swagger/OpenAPI** - Documentação da API

### Banco de Dados
- **PostgreSQL** - Banco de dados principal
- **SeaORM Migration** - Sistema de migrações

### Infraestrutura
- **Docker Compose** - Containerização do banco de dados

## 📋 Pré-requisitos

- Rust (edição 2024)
- Docker e Docker Compose
- PostgreSQL (via Docker)
- Ollama (para modelos de IA locais)

## 🚀 Configuração e Setup

### 1. Clone o repositório
```bash
git clone <repository-url>
cd invention-mapper/server
```

### 2. Configure as variáveis de ambiente
Crie um arquivo `.env` na raiz do projeto:

```env
# Database
DATABASE_URL=postgresql://docker:docker@localhost:5432/database

# Server
SERVER_HOST=localhost
SERVER_PORT=3333

# Ollama AI
OLLAMA_PROMPT=your_ai_prompt_here
OLLAMA_MODEL=your_model_name

# Cloudinary
CLOUDINARY_API_SECRET=your_cloudinary_secret
CLOUDINARY_API_KEY=your_cloudinary_key
CLOUDINARY_CLOUD_NAME=your_cloud_name
CLOUDINARY_IMAGES_FOLDER=images
```

### 3. Inicie o banco de dados
```bash
docker-compose up -d
```

### 4. Execute as migrações
```bash
cd migration
cargo run
cd ..
```

### 5. Execute o servidor
```bash
cargo run
```

O servidor estará disponível em `http://localhost:3333`

## 📚 Documentação da API

A documentação Swagger está disponível em:
- **OpenAPI JSON**: `http://localhost:3333/openapi.json`

### 🔧 Endpoints Principais

#### Criação de Invenções

**1. Inserção Manual** - `POST /api/invention`
- O usuário fornece todos os dados da invenção
- Campos obrigatórios: `file` (imagem), `name`, `text`, `external_link`, `lat`, `lon`
- Formato: `multipart/form-data`

**2. Inserção via Wikipedia** - `POST /api/invention/wiki`
- O usuário fornece apenas o link da Wikipedia e coordenadas
- A IA extrai automaticamente os dados da página
- Campos obrigatórios: `wikipedia_link`, `lat`, `lon`
- Formato: `multipart/form-data`

#### Outros Endpoints
- `GET /api/invention` - Lista todas as invenções
- `PUT /api/invention` - Atualiza uma invenção
- `DELETE /api/invention/{id}` - Remove uma invenção

## 🏗️ Estrutura do Projeto

```
server/
├── src/
│   ├── handlers/          # Handlers das requisições HTTP
│   ├── models/           # Modelos de dados
│   ├── routes/           # Definição das rotas
│   ├── env.rs            # Configuração de variáveis de ambiente
│   └── main.rs           # Ponto de entrada da aplicação
├── entity/               # Entidades do banco de dados
├── migration/            # Migrações do banco de dados
├── Cargo.toml           # Dependências do projeto
└── docker-compose.yml   # Configuração do banco de dados
```

## 🔧 Padrões de Projeto

- **Arquitetura em Camadas**: Separação clara entre handlers, models e routes
- **Injeção de Dependência**: Banco de dados injetado via `web::Data`
- **CORS Configurado**: API acessível de diferentes origens
- **Documentação Automática**: Swagger/OpenAPI integrado
- **Migrações Automatizadas**: Sistema de versionamento do banco de dados

## 📝 Licença

Este projeto está sob a licença [inserir licença]. 