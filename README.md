# `fiscalidade_server`

[WIP] Servidor stand-alone com cache e APIs REST para envio e consulta de XMLs de Documentos Fiscais da SEFAZ.

## Linha de comando

A aplicação é compatível com Windows e Linux oferecendo um menu como a seguir:

```
$ cargo run --quiet --release -- --help
Fiscalidade Server v0.6.7 (linux-x86_64)

Copyright (c) Silvio Clécio <silvioprog@gmail.com>

Uso: target/release/fiscalidade_server [opções]

Opções:
    -h, --help          Imprime este menu de ajuda
    -v, --version       Imprime versão da aplicação
    -p, --port 8000     Porta do servidor
    -d, --database postgres://postgres:postgres@localhost/postgres
                        Banco de dados
    -s, --silent        Desativa informações de log
```

## Recursos

Geral:

- Autenticação via token
- Controle acesso público, privado, administrativo ou de serviço
- Criação e atualização automática de DB
- Cadastro e listagem de Contribuintes
- Listagem de Serviços (no momento, apenas NF-e)
- Cache de XML consultado
- Listagem de cache
- Log de erros e acessos
- Configuração de webservices

NF-e / consultas:

- Status do Serviço
- Consulta Cadastro por CPF, CNPJ, ou IE
- Consulta XML por chave da nota

## APIs disponíveis

O servidor possui APIs com acesso pública, privado, administrativo ou por serviço. Toda comunicação é feita via REST, garantindo uma maior compatibilidade entre diversos tipos de clients HTTP.

Uma API é composta do seguinte formato:

`<servidor>/<aplicação>/<versão>/<path>[/outro-path]`

Exemplo:

`http://localhost:8080/fiscalidade/v1/taxpayers/services`

Tipos de autenticação:

- pública: não requer autenticação;
- privada: requer autenticação via token;
- administrativa: requer autenticação via token e direitos de administrativos;
- serviço: requer autenticação via token e autorização para uso de determinado serviço.

A lista completa com as APIs bem como suas respectivas utilidades pode ser encontrada em [docs/APIs.md](docs/APIs.md).

## Uso

Siga as instruções iniciais de uso em [docs/Uso.md](docs/Uso.md). Para instruções sobre instalação, siga os passos em [docs/Instalação.md](docs/Instalação.md).

## ~~Download~~

~~Acesse a [área de releases](https://github.com/risoflora/fiscalidade_server/releases) para download de binários compatíveis com Windows ou Linux.~~

## Wishlist

- [x] criar AppImage (prioridade alta)
- [ ] embutir arquivo de serviço no executável (prioridade alta)
- [x] embutir arquivo de webservice no executável
- [x] configuração via arquivo
- [ ] paginação de dados (prioridade baixa)
- [ ] mais serviços como envio de lote, consulta de recibo, inutilização, distribuição de DFe, etc. (prioridade alta)
- [ ] compressão de dados (prioridade baixa)
- [ ] testes (prioridade alta)
- [ ] documentação (prioridade alta)
- [x] exemplos de uso com cURL
- [ ] imagem Docker (prioridade baixa)
- [ ] migrar web core para Warp (prioridade baixa)

## Contribuições

Pull Requests e Issues são sempre bem-vindos! =)

## Licença

`fiscalidade_server` é distribuído sob qualquer uma das seguintes licenças:

- Apache License 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
