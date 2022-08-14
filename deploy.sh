#!/usr/bin/env sh

set -e
cd svelte/dist
git init
git add -A
git commit -m 'deploy'
git push -f git@github.com:benediktwerner/stacklands-combat-simulator.git master:gh-pages

cd -
