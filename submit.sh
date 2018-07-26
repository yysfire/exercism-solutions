#!/bin/bash

if [ -z $1 ]; then
  echo 'Please give an exercise name'
  exit 1
fi

exercism submit $1/src/lib.rs
git add $1/.
git commit -m "solve $1"
