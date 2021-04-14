#!/bin/bash -e

if [ -z $1 ]; then
  echo 'Please give an exercise name'
  exit 1
fi

exercism download --exercise=$1 --track=rust
git add $1
git commit -m "download $1 for rust track"
