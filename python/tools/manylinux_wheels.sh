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

for manylinux in 1 2010 2014; do
  docker build --pull --build-arg MANYLINUX=$manylinux -t dqcsim-py-manylinux:$manylinux -f "$cwd/Dockerfile" "$cwd"
  docker run --rm -v "$project":/io dqcsim-py-manylinux:$manylinux
  rm -rf $target/release
done
