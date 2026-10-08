#!/bin/sh
# Compila snappec-verify y lo deja en public/snappec-verify/ para que el workflow de Pages lo publique.
#
#   scripts/publish-snappec-verify.sh <checkout de snappec-testing-harness> [directorio de salida]
#
# Escribe versions/<versión>/ (archivos con versión, como goreleaser) y reemplaza latest/ (sin versión,
# para links estables). Cada carpeta lleva checksums.txt y manifest.json, que lee la página.
#
# Variables:
#   VERSION      versión a estampar (default: git describe --tags --always del checkout)
#   ALLOW_DIRTY  1 para compilar con cambios sin commitear (solo para pruebas locales)
set -eu

usage() { echo "uso: $0 <checkout de snappec-testing-harness> [salida]" >&2; exit 2; }
fail() { echo "publish: $*" >&2; exit 1; }

[ $# -ge 1 ] || usage
src="$(cd "$1" && pwd)"
site="$(cd "$(dirname "$0")/.." && pwd)"
out="${2:-$site/public/snappec-verify}"
module="gitlab.snappler.com/ai-lab/ai-kit/snappec-verify"

[ -f "$src/cmd/snappec-verify/main.go" ] || fail "$src no parece un checkout de snappec-verify"
if [ -n "$(git -C "$src" status --porcelain --untracked-files=no)" ] && [ "${ALLOW_DIRTY:-0}" != 1 ]; then
    fail "$src tiene cambios sin commitear; commitealos o corré con ALLOW_DIRTY=1"
fi

version="${VERSION:-$(git -C "$src" describe --tags --always)}"
version="${version#v}"
commit="$(git -C "$src" rev-parse --short HEAD)"
date="$(git -C "$src" show -s --format=%cs HEAD)"

if command -v sha256sum >/dev/null 2>&1; then
    sha256() { sha256sum "$1" | cut -d' ' -f1; }
else
    sha256() { shasum -a 256 "$1" | cut -d' ' -f1; }
fi
filesize() { wc -c <"$1" | tr -d ' '; }

stage="$(mktemp -d)"
trap 'rm -rf "$stage"' EXIT INT TERM
versioned="$stage/versions/$version"
latest="$stage/latest"
mkdir -p "$versioned" "$latest"

manifest_entries=""
for os in linux darwin; do
    for arch in amd64 arm64; do
        echo "compilando $version $os/$arch"
        build="$stage/build-$os-$arch"
        mkdir -p "$build"
        (cd "$src" && CGO_ENABLED=0 GOOS=$os GOARCH=$arch go build -trimpath \
            -ldflags "-s -w -X $module/internal/cli.Version=$version" \
            -o "$build/snappec-verify" ./cmd/snappec-verify)
        cp "$src/README.md" "$build/README.md"

        vfile="snappec-verify_${version}_${os}_${arch}.tar.gz"
        lfile="snappec-verify_${os}_${arch}.tar.gz"
        # Fechas y dueños fijos: el mismo commit da el mismo tarball, y git no guarda dos blobs.
        tar -C "$build" --owner=0 --group=0 --numeric-owner --mtime="$date" -cf - snappec-verify README.md \
            | gzip -n >"$versioned/$vfile"
        cp "$versioned/$vfile" "$latest/$lfile"

        sum="$(sha256 "$versioned/$vfile")"
        size="$(filesize "$versioned/$vfile")"
        echo "$sum  $vfile" >>"$versioned/checksums.txt"
        echo "$sum  $lfile" >>"$latest/checksums.txt"
        manifest_entries="$manifest_entries${manifest_entries:+,}
    {\"os\": \"$os\", \"arch\": \"$arch\", \"file\": \"%FILE_${os}_${arch}%\", \"size\": $size, \"sha256\": \"$sum\"}"
    done
done

write_manifest() { # $1 = carpeta, $2 = "versioned" | "latest"
    entries="$manifest_entries"
    for os in linux darwin; do
        for arch in amd64 arm64; do
            if [ "$2" = versioned ]; then name="snappec-verify_${version}_${os}_${arch}.tar.gz"
            else name="snappec-verify_${os}_${arch}.tar.gz"; fi
            entries="$(printf '%s' "$entries" | sed "s|%FILE_${os}_${arch}%|$name|")"
        done
    done
    printf '{\n  "version": "%s",\n  "commit": "%s",\n  "date": "%s",\n  "assets": [%s\n  ]\n}\n' \
        "$version" "$commit" "$date" "$entries" >"$1/manifest.json"
}
write_manifest "$versioned" versioned
write_manifest "$latest" latest

mkdir -p "$out/versions"
rm -rf "$out/versions/$version" "$out/latest"
cp -R "$versioned" "$out/versions/$version"
cp -R "$latest" "$out/latest"

echo "listo: $out/latest ($version, commit $commit)"
