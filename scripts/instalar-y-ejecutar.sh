#!/usr/bin/env bash
# Instala dependencias y ejecuta el simulador en Linux.
# Distribuciones soportadas: Debian/Ubuntu, Fedora y Arch (y derivadas).
set -euo pipefail
cd "$(dirname "$0")/.."

if command -v apt-get >/dev/null 2>&1; then
    sudo apt-get update
    sudo apt-get install -y libx11-dev libxi-dev libgl1-mesa-dev \
        libasound2-dev libxkbcommon-dev pkg-config curl
elif command -v dnf >/dev/null 2>&1; then
    sudo dnf install -y libX11-devel libXi-devel mesa-libGL-devel \
        alsa-lib-devel libxkbcommon-devel pkgconf-pkg-config curl
elif command -v pacman >/dev/null 2>&1; then
    sudo pacman -S --needed libx11 libxi mesa alsa-lib libxkbcommon pkgconf curl
else
    echo "No reconozco tu gestor de paquetes (apt/dnf/pacman)." >&2
    echo "Instala a mano: X11 + OpenGL + ALSA + libxkbcommon + Rust." >&2
    exit 1
fi

if ! command -v cargo >/dev/null 2>&1; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    # shellcheck disable=SC1091
    source "$HOME/.cargo/env"
fi

cargo run --release
