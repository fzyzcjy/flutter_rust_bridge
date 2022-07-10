#!/bin/sh
set -e pipefail
REPO="https://github.com/fzyzcjy/flutter_rust_bridge"
MANIFEST=`cargo read-manifest`
VERSION=`echo $MANIFEST | jq -r '.version'`
DESCRIPTION=`echo $MANIFEST | jq -r '.description'`
NAME=`echo $MANIFEST | jq -r '.name'`
LICENSE=`echo $MANIFEST | jq -r '.license'`
TAG="v$VERSION"

cat <<EOF
{
    "version": "$VERSION",
    "description": "$DESCRIPTION",
    "homepage": "https://fzyzcjy.github.io/flutter_rust_bridge/",
    "license": "$LICENSE",
    "architecture": {
        "64bit": {
            "url": "$REPO/releases/download/$TAG/$NAME-x86_64-pc-windows-msvc-$TAG.zip"
        },
        "32bit": {
            "url": "$REPO/releases/download/$TAG/$NAME-i686-pc-windows-msvc-$TAG.zip"
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
                    "url": "\$baseurl/$NAME-x86_64-pc-windows-msvc.zip.sha256"
                }
            },
            "32bit": {
                "url": "$REPO/releases/download/v\$version/$NAME-i686-pc-windows-msvc-v\$version.zip",
                "hash": {
                    "url": "\$baseurl/$NAME-i686-pc-windows-msvc.zip.sha256"
                }
            }
        }
    }
}
EOF