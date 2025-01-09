#!/bin/bash
set -e
echo "${1}.png"
plantuml -o ./ $1
