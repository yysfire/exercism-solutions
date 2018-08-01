#!/bin/bash

if [ -z $1 ]; then
  echo 'Please give an exercise name'
  exit 1
fi

git add $1/
git diff --name-only --cached | tr '\n' ' ' | xargs exercism submit
git commit -m "solve $1"
