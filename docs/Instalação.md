# Instalando o PostgreSQL

O Fiscalidade Server requer o PostgreSQL que pode ser instalado por setup de instalação ou container Docker.

Para instala-lo por setup, baixe o executável distribuído em [postgresql.org/download](https://www.postgresql.org/download) e siga as instruções dele.

Caso opte instalar o PostgreSQL num container Docker, execute o seguinte comando:

```bash
docker run --name postgres -d \
    -p 5432:5432 \
    -e POSTGRES_PASSWORD=postgres \
    -v $HOME/docker/volumes/postgres:/var/lib/postgresql/data \
    --restart unless-stopped postgres
```

O comando acima instala uma versão mínima (suficiente) do PostgreSQL e configura para subir na porta `5432` usando banco, usuário e senha `postgresql`.

Opcionalmente, também é possível instalar a versão web do pgAdmin4 via Docker:

```bash
docker run --name pgadmin4 -d \
    -p 80:80 \
    -e 'PGADMIN_DEFAULT_EMAIL=postgres' \
    -e 'PGADMIN_DEFAULT_PASSWORD=postgres' \
    -v /private/var/lib/pgadmin:$HOME/docker/pgadmin \
    --restart unless-stopped dpage/pgadmin4
```

Agora basta acessar o pgAdmin em `http://localhost` para verificar se tudo ocorreu bem.

## Instalando `fiscalidade_server`

Os passos a seguir instalam o Fiscalidade Server em modo de serviço no Windows e daemon de usuário no Linux a partir de um [AppImage](https://en.wikipedia.org/wiki/AppImage).

## Windows 10 ou superior

**Instalando:**

Para instalar o Fiscalidade Server no Windows, acesse a [área de downloads](https://github.com/risoflora/fiscalidade_server/releases) do projeto e obtenha o setup de instalação da versão mais recente (por exemplo: `FiscalidadeServerSetup-x64-1.0.0.exe`). Feito isto, basta executá-lo como administrador e seguir as informações presentes nas telas do instalador.

**Configurando:**

Se a instalação ocorrer com sucesso, o arquivo de configuração `fiscalidade_server.conf` encontra-se em `C:\Program Files\Fiscalidade Server`. Neste mesmo diretório também encontra-se o arquivo de logs `fiscalidade_server.log` com os erros (caso ocorram) da aplicação.

**Desinstalando:**

Caso deseje desinstala-lo, acesse o **Painel de control** do sistema e remova-o na área **Programas > Programas e Recursos**.

## Linux (Debian, Ubuntu, Fedora, openSUSE etc.)

**Instalando:**

Para instalar o Fiscalidade Server no Linux execute o seguinte comando:

```bash
bash <(curl -s -L https://raw.githubusercontent.com/risoflora/fiscalidade_server/master/scripts/setup.sh)
```

Escolha a opção `1. Install daemon`, confirme com Enter e aguarde até completar a instalação. Observe: este comando **não requer** usuário root.

**Configurando:**

Se a instalação ocorrer com sucesso, o arquivo de configuração encontra-se em `~/.fiscalidade_server.conf` e o de logs `~/.fiscalidade_server.log` contendo os erros (caso ocorram) da aplicação.

**Desinstalando:**

Caso deseje desinstala-lo, execute:

```bash
bash <(curl -s -L https://raw.githubusercontent.com/risoflora/fiscalidade_server/master/scripts/setup.sh)
```

Escolha a opção `2. Uninstall daemon`, confirme com Enter e aguarde a desinstalação.

## Usando

Após a instalação com sucesso, siga os passos de uso disponíveis em [docs/Uso.md](Uso.md).
