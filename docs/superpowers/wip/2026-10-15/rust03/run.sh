#!/bin/sh
# usage: run.sh name...  -> runs each bin, writes out/<name>.txt with masked paths
cd "$(dirname "$0")/ownership-lab"
mkdir -p ../out
for n in "$@"; do
  cargo run -q --bin "$n" > ../out/$n.txt 2>&1
  echo "exit=$?" >> ../out/$n.txt
  sed -i -e 's#C:\Users\[^ )]*ownership-lab#.../ownership-lab#g' -e 's#C:/Users/[^ )]*ownership-lab#.../ownership-lab#g' ../out/$n.txt
  echo "===== $n"
  cat ../out/$n.txt
done
