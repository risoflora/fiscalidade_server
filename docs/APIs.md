# Tabelas de APIs e tipos de dados

Lista completa de APIs disponíveis no servidor e seus respectivos tipos de dados manipuláveis via `GET`, `POST`, `PUT` e `DELETE`.

## Retornos

Exemplos de retorno com sucesso:

```json
{
  "status": "ok",
  "result": {
    "id": 1,
    "taxpayer_id": 2,
    "service_id": 1,
    "allowed_at": "2020-02-19T18:42:17.851178",
    "created_at": "2020-02-19T17:07:10.825510"
  }
}
```

ou:

```json
{
  "status": "ok",
  "result": "<?xml version='1.0' encoding='utf-8'?><soapenv:Envelope xmlns:soapenv=\"http://www.w3.org/2003/05/soap-envelope\"><soapenv:Body><nfeResultMsg xmlns=\"http://www.portalfiscal.inf.br/nfe/wsdl/NFeStatusServico4\"><retConsStatServ xmlns=\"http://www.portalfiscal.inf.br/nfe\" versao=\"4.00\"><tpAmb>1</tpAmb><verAplic>MT_A2RL-4.00</verAplic><cStat>107</cStat><xMotivo>Servico em Operacao</xMotivo><cUF>51</cUF><dhRecbto>2020-02-19T14:42:43-04:00</dhRecbto><tMed>3</tMed></retConsStatServ></nfeResultMsg></soapenv:Body></soapenv:Envelope>"
}
```

Exemplos de retorno com erros:

```json
{ "status": "error", "reason": "Not found" }
```

ou:

```json
{
  "status": "error",
  "reason": "duplicate key value violates unique constraint \"fiscalidade_taxpayers_name_key\""
}
```

dentre outros.

## Contribuintes (Taxpayers)

APIs:

|  Nº | Método   | Caminho (path)    | Tipo           | Utilidade                 |
| --: | -------- | ----------------- | -------------- | ------------------------- |
|   1 | `POST`   | `/taxpayers`      | Publica        | Cria um novo contribuinte |
|   2 | `PUT`    | `/taxpayers/<id>` | Administrativa | Atualiza um contribuinte  |
|   3 | `DELETE` | `/taxpayers/<id>` | Administrativa | Exclui um contribuinte    |
|   4 | `GET`    | `/taxpayers`      | Administrativa | Lista contribuintes       |

Dados:

| Campo                  | Tipo            | 1 2 3 4 | Descrição             |
| ---------------------- | --------------- | ------- | --------------------- |
| `id`                   | `i64`           | N S S S | Identificador         |
| `name`                 | `String`        | S S S N | Nome                  |
| `business_name`        | `String`        | S S S N | Razão Social          |
| `registry`             | `String`        | S S S N | CNPJ                  |
| `email`                | `String`        | S S S N | E-mail                |
| `certificate`          | `String`        | S S S N | Certificado em base64 |
| `certificate_password` | `String`        | S S S N | Senha do certificado  |
| `token`                | `String`        | N S S N | Token para login      |
| `manager`              | `bool`          | N S S N | Administrador         |
| `active`               | `bool`          | N S S N | Ativo                 |
| `created_at`           | `NaiveDateTime` | N S N N | Data de cadastro      |

## Serviços (Services)

APIs:

| Método | Caminho (path) | Tipo    | Utilidade      |
| ------ | -------------- | ------- | -------------- |
| `GET`  | `/services`    | Pública | Lista serviços |

Dados:

| Campo         | Tipo            | Descrição                    |
| ------------- | --------------- | ---------------------------- |
| `id`          | `i64`           | Identificador                |
| `description` | `String`        | Descrição (NF-e, NFC-e etc.) |
| `slug`        | `String`        | Serviço (nfe, nfce etc.)     |
| `active`      | `bool`          | Ativo                        |
| `created_at`  | `NaiveDateTime` | Data de cadastro             |

## Contribuintes/Serviços

APIs:

|  Nº | Método | Caminho (path)                         | Tipo           | Utilidade                                                   |
| --: | ------ | -------------------------------------- | -------------- | ----------------------------------------------------------- |
|   1 | `POST` | `/taxpayers/services`                  | Privada        | Cria solicitação de atribuição de serviço para contribuinte |
|   2 | `GET`  | `/taxpayers/services/unauthorized`     | Administrativa | Lista serviços ainda não autorizados para contribuintes     |
|   3 | `POST` | `/taxpayers/services/authorize`        | Privada        | Autoriza serviço para contribuinte                          |
|   4 | `POST` | `/taxpayers/services/unauthorize/<id>` | Privada        | Desautoriza serviço de contribuinte                         |

Dados:

| Campo         | Tipo            | 1 2 3 4 | Descrição          |
| ------------- | --------------- | ------- | ------------------ |
| `id`          | `i64`           | N S S N | Identificador      |
| `taxpayer_id` | `i64`           | S S S S | ID do contribuinte |
| `service_id`  | `i64`           | S S S S | ID do serviço      |
| `allowed_at`  | `NaiveDateTime` | N S S S | Data de liberação  |
| `created_at`  | `NaiveDateTime` | N S S S | Data de cadastro   |

## NF-e

APIs:

| Método | Caminho (path)                                                     | Tipo    | Utilidade                                      |
| ------ | ------------------------------------------------------------------ | ------- | ---------------------------------------------- |
| `GET`  | `/status-servico/<uf>/<ambiente>`                                  | Serviço | Consulta Status do Serviço de webservice SEFAZ |
| `GET`  | `/consultar-cadastro/<uf>/<ambiente>/<tipo_documento>/<documento>` | Serviço | Consulta Cadastro na SEFAZ por CPF, CNPJ ou IE |
| `POST` | `/consultar-xml/<uf>/<ambiente>/<chave>`                           | Serviço | Consulta XML por chave da nota                 |
