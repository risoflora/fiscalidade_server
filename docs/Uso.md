# Passo a passo para usar o `fiscalidade_server`

Os passos a seguir preparam um ambiente para rodar o `fiscalidade_server` no Windows ou Linux. Serão necessárias as seguintes ferramentas:

- PostgreSQL
- Docker (opcional)
- Executável do Fiscalidade Server

## Instalando o PostgreSQL

O primeiro passo necessário para rodar `fiscalidade_server` é instalar o servidor banco de dados. Para instala-lo, basta usar o [setup oficial distribuído no site do fabricante](https://www.postgresql.org/download) e seguir os passos do arquivo de leia-me deles.

É possível também usar o PostgreSQL a partir de uma imagem Docker. Por exemplo:

```bash
docker run --name postgres -d \
    -p 5432:5432 \
    -e POSTGRES_PASSWORD=postgres \
    -v $HOME/docker/volumes/postgres:/var/lib/postgresql/data \
    --restart unless-stopped postgres
```

O comando acima instala uma versão mínima (suficiente) do PostgreSQL subindo-o na porta padrão, `5432`. Alternativamente, também é possível instalar a versão web do pgAdmin4 via Docker:

```bash
docker run --name pgadmin4 -d \
    -p 80:80 \
    -e 'PGADMIN_DEFAULT_EMAIL=postgres' \
    -e 'PGADMIN_DEFAULT_PASSWORD=postgres' \
    -v /private/var/lib/pgadmin:$HOME/docker/pgadmin \
    --restart unless-stopped dpage/pgadmin4
```

Agora basta acessar `http://localhost` e verificar se tudo ocorreu bem.

## Executando o `fiscalidade_server`

Uma vez com o PostgreSQL instalado, basta executar o Fiscalidade Server e ele se encarregará de criar as tabelas no banco de dados automaticamente. O comando mínimo para executá-lo é:

```bash
cargo run --release -- -m \
    -p 8080 \
    -d postgres://postgres:postgres@172.17.0.1/postgres \
    -w resources/webservices.ini
```

Explicando os parâmetros do comando acima:

- `-p 8080` - Porta do servidor. Onde a API será disponibilizada, ex: `http://localhost:8080/fiscalidade/v1/taxpayers/services`.
- `-d postgres://postgres:postgres@localhost/postgres` - Caminho (path) para o banco de dados. Neste caso, usando o DB padrão, `postgres`.
- `-w resources/webservices.ini` - Arquivo de webservices. Também disponível [aqui](https://github.com/risoflora/fiscalidade/tree/master/resources).
- `-m` - Executa scripts para criação (ou atualização) do banco de dados.

Se tudo ocorrer bem, será exibido o seguinte resultado no terminal:

```
cargo run --release -- -m \
    -p 8080 \
    -d postgres://postgres:postgres@localhost/postgres \
    -w resources/webservices.ini
🔧 Configured for production.
    => address: 0.0.0.0
    => port: 8080
    => log: critical
    => workers: 16
    => secret key: provided
    => limits: forms = 512KiB
    => keep-alive: 16s
    => tls: disabled
    => [extra] databases: { db::conn = { url = "postgres://postgres:postgres@localhost/postgres" } }
🚀 Rocket has launched from http://0.0.0.0:8080
```

Para mais informações, use o menu de ajuda da aplicação: `cargo run --release -- -h`.
