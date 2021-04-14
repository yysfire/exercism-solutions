#!/bin/bash -e

if [ -z $1 ]; then
  echo 'Please give an exercise name'
  exit 1
fi

if [ -z "$2" ]; then
  msg="solve $1 in rust track"
else
  msg=$2
fi

git add $1/
#git diff --name-only --cached | tr '\n' ' ' | xargs exercism submit
git diff --name-status --cached | grep -v D | sed 's|[^/]\+/||' | tr '\n' ' ' | xargs exercism submit
git commit -m "${msg}"
