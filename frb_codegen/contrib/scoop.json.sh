#!/bin/sh
# Used to bootstrap a Scoop package.
# If updating, prefer using the CI workflow to this script.
set -e pipefail
REPO="https://github.com/fzyzcjy/flutter_rust_bridge"
MANIFEST=$(cargo read-manifest)
VERSION=$(echo "$MANIFEST" | jq -r ".version")
DESCRIPTION=$(echo "$MANIFEST" | jq -r ".description")
NAME=$(echo "$MANIFEST" | jq -r ".name")
LICENSE=$(echo "$MANIFEST" | jq -r ".license")
TAG="v$VERSION"
URL64="$REPO/releases/download/$TAG/$NAME-x86_64-pc-windows-msvc-$TAG.zip"
URL32="$REPO/releases/download/$TAG/$NAME-i686-pc-windows-msvc-$TAG.zip"
HASH64=$(curl -L "$URL64.sha256")
HASH32=$(curl -L "$URL32.sha256")

cat <<EOF
{
    "version": "$VERSION",
    "description": "$DESCRIPTION",
    "homepage": "https://fzyzcjy.github.io/flutter_rust_bridge/",
    "license": "$LICENSE",
    "architecture": {
        "64bit": {
            "url": "$URL64",
            "hash": "$HASH64"
        },
        "32bit": {
            "url": "$URL32",
            "hash": "$HASH32"
        }
    },
    "bin": "$NAME.exe",
    "checkver": {
        "github": "$REPO"
    },
    "autoupdate": {
        "architecture": {
            "64bit": {
                "url": "$REPO/releases/download/v\$version/$NAME-x86_64-pc-windows-msvc-v\$version.zip",
                "hash": {
                    "url": "\$baseurl/$NAME-x86_64-pc-windows-msvc-v\$version.zip.sha256"
                }
            },
            "32bit": {
                "url": "$REPO/releases/download/v\$version/$NAME-i686-pc-windows-msvc-v\$version.zip",
                "hash": {
                    "url": "\$baseurl/$NAME-i686-pc-windows-msvc-v\$version.zip.sha256"
                }
            }
        }
    }
}
EOF