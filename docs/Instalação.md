# Instalando `fiscalidade_server`

Os passos a seguir instalam o daemon do `fiscalidade_server` no sistema em modo usuário, ou seja, não requer root.

## Windows 10 ou superior

TODO

Para desinstalar:

TODO

## Linux (Debian, Ubuntu, Mint etc.)

Instalar a `libpq` em distribuições Debian, Ubuntu ou derivados:

```bash
sudo apt install libpq5
```

Instalando `fiscalidade_server` como daemon sem usuário root:

```bash
# baixar ou compilar o executável `fiscalidade_server`
cd ~
mkdir -p $HOME/.fiscalidade/bin
mv Downloads/fiscalidade_server $HOME/.fiscalidade/bin/
echo 'export PATH="$HOME/.fiscalidade/bin:$PATH"' >> .profile
source .profile
mv fiscalidade_server.service ~/.config/systemd/user/
systemctl --user add-wants default.target fiscalidade_server
systemctl --user start fiscalidade_server
```

Checando se o daemon está ativo:

```bash
systemctl --user status fiscalidade_server.service
```

Para desinstalar:

```bash
systemctl --user stop fiscalidade_server
systemctl --user disable fiscalidade_server
rm ~/.config/systemd/user/fiscalidade_server.service
```

## Linux (RHEL, Fedora and CentOS etc.)

Instalar a `libpq` em distribuições RHEL, Fedora and CentOS ou derivados:

```bash
sudo dnf install libpq
```

Instalando `fiscalidade_server` como daemon sem usuário root:

```bash
# baixar ou compilar o executável `fiscalidade_server`
cd ~
mkdir -p $HOME/.fiscalidade/bin
mv Downloads/fiscalidade_server $HOME/.fiscalidade/bin/
echo 'export PATH="$HOME/.fiscalidade/bin:$PATH"' >> .profile
source .profile
mv fiscalidade_server.service ~/.config/systemd/user/
systemctl --user add-wants default.target fiscalidade_server
systemctl --user start fiscalidade_server
```

Checando se o daemon está ativo:

```bash
systemctl --user status fiscalidade_server.service
```

Para desinstalar:

Para desinstalar:

```bash
systemctl --user stop fiscalidade_server
systemctl --user disable fiscalidade_server
rm ~/.config/systemd/user/fiscalidade_server.service
```
