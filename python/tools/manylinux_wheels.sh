#!/bin/bash

# This script builds manylinux wheels in target/python/dist/
# Usage: ./manylinux_wheels.sh

set -e -x

dir="$(dirname "${BASH_SOURCE[0]}")"
cwd=`eval "cd $dir;pwd;cd - > /dev/null"`
dir="$cwd/../.."
project=`eval "cd $dir;pwd;cd - > /dev/null"`

# Make sure target dir is empty
target="$project/target"
rm -rf $target

docker build -t dqcsim-py-manylinux -f "$cwd/Dockerfile" "$cwd"
docker run --rm -v "$project":/io dqcsim-py-manylinux
