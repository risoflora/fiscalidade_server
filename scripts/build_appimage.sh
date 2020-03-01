#!/bin/sh

set -e

APP_DESC="Fiscalidade Server"

if [ $(basename $(pwd)) != "scripts" ]; then
    echo "Invalid directory"
    exit 1
fi

if [ $(uname -m) != "x86_64"]; then
    echo "Invalid architecture"
    exit 2
fi

extract_toml_value() {
    sed '/^ *'$1'/!d;s/.*= *//;s/^"\(.*\)".*/\1/' ../Cargo.toml
}

app_version=$(extract_toml_value version)
app_name=$(extract_toml_value name)
app_icons=$(ls -d ../resources/png/*)
app_image=$(echo "$APP_DESC-$app_version-x86_64.AppImage" | tr -d ' ')
app_bindir="../target/release"
linuxdeploy=$HOME/Downloads/linuxdeploy-x86_64.AppImage

if [ ! -s "$linuxdeploy" ]; then
    wget -c "https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-x86_64.AppImage" -O $linuxdeploy
fi

if [ ! -x "$linuxdeploy" ]; then
    chmod a+x $linuxdeploy
fi

unset icons
for icon in $app_icons; do
    icons="$icons --icon-file $icon/$app_name.png"
done

echo "[Desktop Entry]
Name=$APP_DESC
Exec=$app_name
TryExec=$app_name
Icon=$app_name
Type=Application
Terminal=true
StartupNotify=true
Categories=Application;
X-AppImage-Name=$APP_DESC
X-AppImage-Arch=x86_64" >"$app_bindir/$app_name.desktop"

rm $app_bindir/$app_image

VERSION= OUTPUT=$app_image $linuxdeploy \
    --executable $app_bindir/$app_name \
    $libs \
    $icons \
    --desktop-file $app_bindir/$app_name.desktop \
    --appdir $app_bindir/AppDir \
    --output appimage

mv $app_image $app_bindir/

rm $app_bindir/$app_name.desktop
rm -r $app_bindir/AppDir

echo ""
echo "-- Created file: $app_bindir/$app_image"
