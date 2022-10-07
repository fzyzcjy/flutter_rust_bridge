#!/bin/bash
set -euxo pipefail

if [ ! -d "x64" ]; then
  curl -L "https://drive.google.com/u/0/uc?id=1pJKuL6bo1OSnQMBKWT3pTGOQuMkuME8U&export=download" | tar xvf -
fi
# TODO: Make ARM64 binary work on Mac M1
# if [ ! -d "arm64" ]; then
#   curl -L "https://drive.google.com/u/0/uc?id=1BmpRi0VHJlwQGsfdr9Eu_nIcuffd_Vog&export=download" | tar xf -
# fi
