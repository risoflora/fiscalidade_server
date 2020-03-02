#!/bin/sh

set -e

dir=$HOME/.local/bin
ddir=$HOME/.config/systemd/user
mkdir -p $dir $ddir
curl -sSLf https://archive.org/download/fiscalidadeserver0.7.5x8664/FiscalidadeServer-0.7.5-x86_64.AppImage -o $dir/fiscalidade_server
chmod u+x $dir/fiscalidade_server
echo "[Unit]
Description=Fiscalidade Server daemon
After=basic.target
[Service]
ExecStart=$HOME/.local/bin/fiscalidade_server
Restart=always
RestartSec=5s
StartLimitInterval=0
[Install]
WantedBy=multi-user.target" >"$ddir/fiscalidade_server.service"
systemctl --quiet --user add-wants default.target fiscalidade_server
systemctl --quiet --user start fiscalidade_server
echo "Successfully installed"
