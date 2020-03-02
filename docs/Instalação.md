# Instalando `fiscalidade_server`

Os passos a seguir instalam o Fiscalidade Server em modo de serviço no Windows e um daemon de usuário no Linux a partir de um [AppImage](https://en.wikipedia.org/wiki/AppImage).

## Windows 10 ou superior

**Instalando:**

Para instalar o Fiscalidade Server no Windows, acesse a [área de downloads](https://github.com/risoflora/fiscalidade_server/releases) do projeto e obtenha o setup de instalação da versão mais recente (por exemplo: `FiscalidadeServerSetup-x64-1.0.0.exe`). Feito isto, basta executá-lo como administrador e seguir as informações presentes nas telas do instalador.

**Configurando:**

Se a instalação ocorrer com sucesso, o arquivo de configuração `fiscalidade_server.conf` encontra-se em `C:\Program Files\Fiscalidade Server`. Neste mesmo diretório também encontra-se o arquivo de logs `fiscalidade_server.log` com os erros (caso ocorram) da aplicação.

**Desinstalando:**

Caso deseje desinstala-lo, acesse o **Painel de control** do sistema e remova-o na área **Programas > Programas e Recursos**.

## Linux (Debian, Ubuntu, Fedora, openSUSE etc.)

**Instalando:**

Para instalar o Fiscalidade Server no Linux, acesse a [área de downloads](https://github.com/risoflora/fiscalidade_server/releases) do projeto e obtenha o [AppImage](https://en.wikipedia.org/wiki/AppImage) da versão mais recente (por exemplo: `FiscalidadeServer-1.0.0-x86_64.AppImage`). Feito isto, execute o seguinte comando:

```bash
curl -sSf https://raw.githubusercontent.com/risoflora/fiscalidade_server/master/scripts/setup.sh | sh
```

Escolha a opção "`1. Install daemon`" e aguarde até completar a instalação. Observe: este comando **não requer** usuário root.

**Configurando:**

Se a instalação ocorrer com sucesso, o arquivo de configuração encontra-se em `~/.fiscalidade_server.conf` e o de logs `~/.fiscalidade_server.log` contendo os erros (caso ocorram) da aplicação.

**Desinstalando:**

Caso deseje desinstala-lo, execute:

```bash
curl -sSf https://raw.githubusercontent.com/risoflora/fiscalidade_server/master/scripts/setup.sh | sh
```

Escolha a opção "`2. Uninstall daemon`" e aguarde a desinstalação.

## Usando

Após a instalação com sucesso, siga os passos de uso disponíveis em [docs/Uso.md](Uso.md).
