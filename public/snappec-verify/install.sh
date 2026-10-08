#!/bin/sh
# Instala snappec-verify desde https://egonik-unlp.github.io/snappec-verify
#
#   curl -fsSL https://egonik-unlp.github.io/snappec-verify/install.sh | sh
#
# Variables:
#   BINDIR                  destino del binario (default: ~/.local/bin)
#   SNAPPEC_VERIFY_VERSION  versión puntual (default: la última publicada)
#   SNAPPEC_VERIFY_BASE     URL base de la página (default: la de arriba)
set -eu

BASE="${SNAPPEC_VERIFY_BASE:-https://egonik-unlp.github.io/snappec-verify}"
VERSION="${SNAPPEC_VERIFY_VERSION:-latest}"
BINDIR="${BINDIR:-$HOME/.local/bin}"

fail() { echo "snappec-verify: $*" >&2; exit 1; }

case "$(uname -s)" in
    Linux) os=linux ;;
    Darwin) os=darwin ;;
    *) fail "sistema no soportado: $(uname -s) (hay binarios para Linux y macOS)" ;;
esac
case "$(uname -m)" in
    x86_64 | amd64) arch=amd64 ;;
    aarch64 | arm64) arch=arm64 ;;
    *) fail "arquitectura no soportada: $(uname -m) (hay binarios para x86-64 y ARM64)" ;;
esac

if [ "$VERSION" = latest ]; then
    dir="latest"
    file="snappec-verify_${os}_${arch}.tar.gz"
else
    dir="versions/$VERSION"
    file="snappec-verify_${VERSION}_${os}_${arch}.tar.gz"
fi

if command -v curl >/dev/null 2>&1; then
    fetch() { curl -fsSL -o "$2" "$1"; }
elif command -v wget >/dev/null 2>&1; then
    fetch() { wget -qO "$2" "$1"; }
else
    fail "hace falta curl o wget"
fi

if command -v sha256sum >/dev/null 2>&1; then
    sha256() { sha256sum "$1" | cut -d' ' -f1; }
elif command -v shasum >/dev/null 2>&1; then
    sha256() { shasum -a 256 "$1" | cut -d' ' -f1; }
else
    fail "hace falta sha256sum o shasum para verificar la descarga"
fi

tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT INT TERM

echo "descargando $file"
fetch "$BASE/$dir/$file" "$tmp/$file" || fail "no se pudo bajar $BASE/$dir/$file"
fetch "$BASE/$dir/checksums.txt" "$tmp/checksums.txt" || fail "no se pudo bajar checksums.txt"

expected="$(awk -v f="$file" '$2 == f { print $1 }' "$tmp/checksums.txt")"
[ -n "$expected" ] || fail "$file no figura en checksums.txt"
actual="$(sha256 "$tmp/$file")"
[ "$expected" = "$actual" ] || fail "checksum distinto para $file (esperado $expected, obtenido $actual)"

tar -xzf "$tmp/$file" -C "$tmp" snappec-verify || fail "no se pudo descomprimir $file"
mkdir -p "$BINDIR"
install -m 0755 "$tmp/snappec-verify" "$BINDIR/snappec-verify"

echo "instalado en $BINDIR/snappec-verify"
"$BINDIR/snappec-verify" --version

case ":$PATH:" in
    *":$BINDIR:"*) ;;
    *) echo "aviso: $BINDIR no está en el PATH; agregalo a tu shell (export PATH=\"$BINDIR:\$PATH\")" ;;
esac
