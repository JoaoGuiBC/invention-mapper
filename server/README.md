
## Invention Mapper - server

Primeiramente, para rodar o servidor certifique-se de: 

 - Ter [Rust](https://www.rust-lang.org/pt-BR) instalado na sua máquina
 - Ter um banco de dados Postgres (o repositório contém um arquivo [Docker Compose](https://docs.docker.com/compose/))
 - Ter uma conta na plataforma [Cloudinary](https://cloudinary.com)
 - Caso deseje usar a funcionalidade de coletar dados automaticamente a partir de um link da Wikipédia também será necessário ter [Ollama](https://ollama.com) instalado na sua máquina
 
 Após isso, crie um arquivo .env na raiz do server, copie o conteúdo do arquivo .env.example e preencha as variáveis de ambiente.
 

### Banco de dados
Com o banco rodando e a variáveis de ambiente definidas, chegou a hora de executar as migrations do banco.

Este projeto utiliza [SeaORM](https://www.sea-ql.org/SeaORM/) para trabalhar com o banco de dados, abra o terminal e instale a CLI do ORM através do comando:

    cargo install sea-orm-cli@1.1.0

Agora basta rodar o comando abaixo e o banco de dados estará configurado:

    sea-orm-cli migrate up

### Executando o servidor
Para executar o servidor rode o comando:

    cargo run

Para gerar uma build de produção rode o comando:

    cargo build --release

Caso deseje uma descrição das rotas do servidor, com ele rodando faça uma requisição para */openapi.json* para receber um arquivo de formatação openapi
