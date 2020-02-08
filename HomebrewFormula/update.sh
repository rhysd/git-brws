#!/bin/bash

set -e

if [ ! -d '.git' ]; then
    echo 'This script must be run at root of the repository' >&2
    exit 1
fi

if [[ "$1" == "" ]]; then
    echo 'Usage: update.sh {tag name}' >&2
    exit 1
fi

cd ./HomebrewFormula

VERSION="$1"

if [[ ! "$VERSION" =~ ^v\d+\.\d+\.\d+$ ]]; then
    echo 'Version number in first argument must match to ^v\d+\.\d+\.\d+$ like v1.2.3' >&2
    exit 1
fi

echo "Update formula to version ${VERSION}"

MAC_ZIP="git-brws-${VERSION}-x86_64-apple-darwin.zip"
MAC_URL="https://github.com/rhysd/git-brws/releases/download/${VERSION}/${MAC_ZIP}"

echo "Downloading ${MAC_ZIP}..."
curl -LO "$MAC_URL"
MAC_SHA="$(shasum -a 256 "$MAC_ZIP" | cut -f 1 -d ' ')"
echo "Mac sha256: ${MAC_SHA}"
sed -i '' -E "s/    sha256 '[0-9a-f]*' # mac/    sha256 '${MAC_SHA}' # mac/" git-brws.rb


LINUX_ZIP="git-brws-${VERSION}-x86_64-unknown-linux-gnu.zip"
LINUX_URL="https://github.com/rhysd/git-brws/releases/download/${VERSION}/${LINUX_ZIP}"

echo "Downloading ${LINUX_ZIP}..."
curl -LO "$LINUX_URL"
LINUX_SHA="$(shasum -a 256 "$LINUX_ZIP" | cut -f 1 -d ' ')"
echo "Linux sha256: ${LINUX_SHA}"
sed -i '' -E "s/    sha256 '[0-9a-f]*' # linux/    sha256 '${LINUX_SHA}' # linux/" git-brws.rb

echo "Version: ${VERSION}"
sed -i '' -E "s/  version '[0-9][0-9]*\.[0-9][0-9]*\.[0-9][0-9]*'/  version '${VERSION#v}'/" git-brws.rb

echo "Clean up zip files"
rm -rf ./*.zip

echo 'Done.'

