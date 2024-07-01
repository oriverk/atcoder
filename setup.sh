#!/usr/bin/env bash

cargo compete new $1
echo https://atcoder.jp/contests/$1/tasks
echo "- [$1](/contests/$1)" >> README.md
