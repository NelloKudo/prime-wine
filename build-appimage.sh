#!/usr/bin/env bash
# attempt at packaging this into an appimage
# this took WAY too long to write
set -e
cd "$(dirname "$0")"

cargo build --release

# static cabextract so users do not need it installed, built once and cached
CABEXTRACT_VERSION=1.11
if [ ! -f build-cache/cabextract ]; then
    mkdir -p build-cache
    curl -L -o build-cache/cabextract.tar.gz "https://www.cabextract.org.uk/cabextract-$CABEXTRACT_VERSION.tar.gz"
    tar -xzf build-cache/cabextract.tar.gz -C build-cache
    (cd "build-cache/cabextract-$CABEXTRACT_VERSION" && ./configure LDFLAGS="-static" >/dev/null && make >/dev/null)
    cp "build-cache/cabextract-$CABEXTRACT_VERSION/cabextract" build-cache/cabextract
fi

rm -rf AppDir
mkdir -p AppDir/usr/bin
cp target/release/prime-wine AppDir/usr/bin/
cp build-cache/cabextract AppDir/usr/bin/
cp assets/icon.png AppDir/prime-wine.png
ln -sf prime-wine.png AppDir/.DirIcon
ln -sf usr/bin/prime-wine AppDir/AppRun

cat > AppDir/prime-wine.desktop <<'EOF'
[Desktop Entry]
Type=Application
Name=Prime Video
GenericName=Video Streaming
Comment=Prime Video client alternative for Linux!
Exec=prime-wine
Icon=prime-wine
Terminal=false
Categories=AudioVideo;Video;Player;Network;
EOF

if [ ! -f appimagetool ]; then
    curl -L -o appimagetool "https://github.com/AppImage/appimagetool/releases/download/continuous/appimagetool-x86_64.AppImage"
    chmod +x appimagetool
fi

# unlink first so a running appimage does not block the build
rm -f PrimeWine-x86_64.AppImage
ARCH=x86_64 ./appimagetool AppDir PrimeWine-x86_64.AppImage
echo "done: PrimeWine-x86_64.AppImage"
