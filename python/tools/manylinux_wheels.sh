#!/bin/bash

# This script builds manylinux{1, 2010} wheels in target/python/dist/
# Usage: ./manylinux_wheels.sh

set -e -x

dir="$(dirname "${BASH_SOURCE[0]}")"
cwd=`eval "cd $dir;pwd;cd - > /dev/null"`
dir="$cwd/../.."
project=`eval "cd $dir;pwd;cd - > /dev/null"`

# Make sure target dir is empty
target="$project/target"
rm -rf $target

docker build -t dqcsim-py-manylinux:1 -f "$cwd/Dockerfile" "$cwd"
docker run --rm -v "$project":/io dqcsim-py-manylinux:1

docker build --build-arg MANYLINUX=2010 -t dqcsim-py-manylinux:2010 -f "$cwd/Dockerfile" "$cwd"
docker run --rm -v "$project":/io dqcsim-py-manylinux:2010
