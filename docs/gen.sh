#!/bin/bash

set -e

if [ ! -d '.git' ]; then
    echo 'Please run this script from root directory of the repository' 2>&1
fi

ronn git-brws.1.ronn
sed -i '' -E "s/([^'])(https:\\/\\/[^[:space:]<]*)/\\1<a href=\"\\2\" target=\"_blank\" rel=\"noopener\">\\2<\\/a>/" git-brws.1.html
mv git-brws.1.html docs/index.html
