#!/bin/sh

set -e

NAME="fiscalidade_server"
DESCRIPTION="Fiscalidade Server"
APPIMAGE_URL="https://archive.org/download/fiscalidadeserver0.7.5x8664/FiscalidadeServer-0.7.5-x86_64.AppImage"
DATABASE_URL="postgres:postgres@localhost/postgres"
SERVER_PORT="8080"

dir=$HOME/.local/bin
ddir=$HOME/.config/systemd/user
exe=$dir/$NAME
svc=$ddir/$NAME.service

setup_question() {
    read -p "Do you want to $1 $DESCRIPTION? [Y/n] " ans
    [ "$ans" = Y -o "$ans" = y -o -z "$ans" ]
}

install() {
    setup_question "install"
    read -p "Server port [$SERVER_PORT]: " port
    if [ -z $port ]; then
        port=$SERVER_PORT
    fi
    if ! [ -z "${port//[0-9]/}" ]; then
        echo "Invalid port"
        exit 1
    fi
    read -p "Database URL [$DATABASE_URL]: " database
    if [ -z $database ]; then
        database=$DATABASE_URL
    fi
    echo "Installing, please wait ..."
    echo "port=$port
database=postgres://$database
silent=true" >"$HOME/.$NAME.conf"
    mkdir -p $dir $ddir
    curl -sSLf $APPIMAGE_URL -o $exe
    chmod u+x $exe
    echo "[Unit]
Description=$DESCRIPTION daemon
After=basic.target
[Service]
ExecStart=$exe
Restart=always
RestartSec=5s
StartLimitInterval=0
[Install]
WantedBy=multi-user.target" >$svc
    systemctl --quiet --user add-wants default.target $NAME
    systemctl --quiet --user start $NAME
    echo "Successfully installed!"
}

uninstall() {
    setup_question "uninstall"
    systemctl --user stop $NAME
    systemctl --quiet --user disable $NAME
    rm -f $svc $exe
    echo "Successfully uninstalled!"
}

status() {
    systemctl --user status $NAME
    read
}

while true; do
    clear
    cat <<_EOF_
Select an option:

1. Install daemon
2. Uninstall daemon
3. Daemon status
0. Quit

_EOF_
    read -p "Enter selection [0-3] > " opt
    if [[ $opt =~ ^[0-3]$ ]]; then
        case $opt in
        1)
            install
            break
            ;;
        2)
            uninstall
            break
            ;;
        3)
            status
            ;;
        0)
            break
            ;;
        esac
    else
        echo "Invalid option"
        sleep 1
    fi
done
