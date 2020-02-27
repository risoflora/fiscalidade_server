# Passo a passo para usar o `fiscalidade_server`

Os passos a seguir preparam um ambiente para rodar o `fiscalidade_server` no Linux e serão necessárias as seguintes ferramentas:

- [PostgreSQL](https://www.postgresql.org)
- [Docker](https://www.docker.com) (opcional)
- [jsonpp](https://crates.io/crates/jsonpp) (opcional)
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

Agora basta acessar o pgAdmin em `http://localhost` para verificar se tudo ocorreu bem.

## Executando o `fiscalidade_server`

Uma vez com o PostgreSQL instalado, basta executar o Fiscalidade Server e ele se encarregará de criar as tabelas no banco de dados automaticamente. O comando mínimo para executá-lo é:

```bash
cargo run --release -- \
    -p 8080 \
    -d postgres://postgres:postgres@172.17.0.1/postgres
```

Explicando os parâmetros do comando acima:

- `-p 8080` - Porta do servidor. Onde a API será disponibilizada, ex: `http://localhost:8080/fiscalidade/v1/taxpayers/services`.
- `-d postgres://postgres:postgres@172.17.0.1/postgres` - Caminho (path) para o banco de dados. Neste caso, usando o DB padrão disponível no Docker, `postgres`.

Para mais informações, use o menu de ajuda da aplicação: `cargo run --release -- -h`.

## Definindo administrador padrão do servidor

Antes de prosseguir com os passos seguintes, verifique se o servidor está online. Para isto, basta consultar sua versão, por exemplo:

```bash
curl -s \
    http://localhost:8080/fiscalidade/v1/version | jsonpp
```

ele deve retornar um JSON com a versão do servidor, exemplo:

```
{
  "status": "ok",
  "result": {
    "major": 0,
    "minor": 6,
    "patch": 2
  }
}
```

feito isso, agora podemos definir um administrador para gerenciamento do servidor:

```bash
curl -s \
    -X POST http://localhost:8080/fiscalidade/v1/taxpayers/manager | jsonpp
```

administrador criado:

```
{
  "status": "ok",
  "result": {
    "id": 1,
    "name": "admin",
    "business_name": "Administrador",
    "registry": "",
    "email": "",
    "certificate": "",
    "certificate_password": "",
    "token": "qoNrF2mZsSUpZCEXUw2Mxx",
    "manager": true,
    "active": true,
    "created_at": "2020-02-24T15:44:37.210486"
  }
}
```

Observe o token gerado: `qoNrF2mZsSUpZCEXUw2Mxx`. **Guarde ele em um local seguro!** Este será o token do administrador padrão do servidor. Usaremos ele nos passos a seguir.

## Cadastrando contribuinte / solicitando uso de serviço

Considerando que o certificado do contribuinte encontra-se em `~/Downloads/certificado.pfx`:

```bash
curl -s \
    -X POST \
    -H 'Content-Type: application/json' \
    -d '{"name":"Fulano","business_name":"Fulano de tal","registry":"123456789","email":"fulano@gmail","certificate":"'$(base64 -w 0 $HOME/Downloads/certificado.pfx)'","certificate_password":"12345678"}' \
    http://localhost:8080/fiscalidade/v1/taxpayers | jsonpp
```

o servidor deve retornar o seguinte JSON:

```
{
  "status": "ok",
  "result": {
    "id": 2,
    "name": "Fulano",
    "business_name": "Fulano de tal",
    "registry": "123456789",
    "email": "fulano@gmail",
    "certificate": "MIIkEAIB...<demais caracteres>=",
    "certificate_password": "12345678",
    "token": "U8pNjWuAdj2PB3AGnai7mT",
    "manager": false,
    "active": true,
    "created_at": "2020-02-24T15:47:03.824520"
  }
}
```

agora, com o contribuinte cadastrado, podemos fazer uma solicitação de uso de serviço. Para consultar a lista de serviços disponíveis, use:

```bash
curl -s \
    http://localhost:8080/fiscalidade/v1/services | jsonpp
```

serviços listados:

```
{
  "status": "ok",
  "result": [
    {
      "id": 1,
      "description": "NF-e",
      "slug": "nfe",
      "active": true,
      "created_at": "2020-02-24T15:44:31.537942"
    }
  ]
}
```

por fim, solicitamos o uso do serviço NF-e para contribuinte cadastrado:

```bash
curl -s \
    -X POST \
    -H 'X-Auth-Token: U8pNjWuAdj2PB3AGnai7mT' \
    -H 'Content-Type: application/json' \
    -d '{"taxpayer_id":2,"service_id":1}' \
    http://localhost:8080/fiscalidade/v1/taxpayers/services | jsonpp
```

solicitação criada:

```
{
  "status": "ok",
  "result": {
    "id": 1,
    "taxpayer_id": 2,
    "service_id": 1,
    "allowed_at": null,
    "created_at": "2020-02-24T15:49:36.359227"
  }
}
```

## Autorizando uso de serviço

A listagem de serviços solicitados pode ser acessada por qualquer usuário administrador. Neste exemplo, usaremos o administrador padrão, que foi cadastrado com o token `qoNrF2mZsSUpZCEXUw2Mxx`.

Listando serviços solicitados:

```bash
curl -s \
    -H 'X-Auth-Token: qoNrF2mZsSUpZCEXUw2Mxx' \
    http://localhost:8080/fiscalidade/v1/taxpayers/services/unauthorized | jsonpp
```

solicitações listadas:

```
{
  "status": "ok",
  "result": [
    {
      "id": 1,
      "taxpayer_id": 2,
      "taxpayer_name": "Fulano",
      "service_id": 1,
      "service_description": "NF-e",
      "allowed_at": null,
      "created_at": "2020-02-24T15:49:36.359227"
    }
  ]
}
```

por fim, basta autorizar uso de serviço "NF-e" para contribuinte "Fulano":

```bash
curl -s \
    -X POST \
    -H 'X-Auth-Token: qoNrF2mZsSUpZCEXUw2Mxx' \
    -H 'Content-Type: application/json' \
    http://localhost:8080/fiscalidade/v1/taxpayers/services/authorize/1 | jsonpp
```

autorização criada:

```
{
  "status": "ok",
  "result": {
    "id": 1,
    "taxpayer_id": 2,
    "service_id": 1,
    "allowed_at": "2020-02-24T15:50:59.319443",
    "created_at": "2020-02-24T15:49:36.359227"
  }
}
```

e finalmente o contribuinte tem permissão para acessar o serviço:

```bash
curl -s \
    -H 'X-Auth-Token: U8pNjWuAdj2PB3AGnai7mT' \
    http://localhost:8080/fiscalidade/v1/nfe/status-servico/mt/p | jsonpp
```

resultado:

```
{
  "status": "ok",
  "result": "<?xml version='1.0' encoding='utf-8'?><soapenv:Envelope xmlns:soapenv=\"http://www.w3.org/2003/05/soap-envelope\"><soapenv:Body><nfeResultMsg xmlns=\"http://www.portalfiscal.inf.br/nfe/wsdl/NFeStatusServico4\"><retConsStatServ xmlns=\"http://www.portalfiscal.inf.br/nfe\" versao=\"4.00\"><tpAmb>1</tpAmb><verAplic>MT_A2RL-4.00</verAplic><cStat>109</cStat><xMotivo>Servico Paralisado sem Previsao</xMotivo><cUF>51</cUF></retConsStatServ></nfeResultMsg></soapenv:Body></soapenv:Envelope>"
}
```

## Desautorizando uso de serviço

Se por alguma razão for necessário remover autorização de uso de serviço para contribuinte, use:

```bash
curl -s \
    -X PUT \
    -H 'X-Auth-Token: qoNrF2mZsSUpZCEXUw2Mxx' \
    -H 'Content-Type: application/json' \
    http://localhost:8080/fiscalidade/v1/taxpayers/services/unauthorize/1 | jsonpp
```

resultado:

```
{
  "status": "ok",
  "result": {
    "id": 1,
    "taxpayer_id": 2,
    "service_id": 1,
    "allowed_at": null,
    "created_at": "2020-02-24T15:49:36.359227"
  }
}
```

e ao tentar acessar o serviço novamente:

```bash
curl -s \
    -H 'X-Auth-Token: U8pNjWuAdj2PB3AGnai7mT' \
    http://localhost:8080/fiscalidade/v1/nfe/status-servico/mt/p | jsonpp
```

o acesso é negado:

```
{
  "status": "error",
  "reason": "Unauthorized"
}
```
