#!/bin/bash

# build
cargo run build
cd target/debug
mv unsafe-finder ../../
cd ../../

# check input file
if [ ! -f input.txt ]; then
  echo "Error: input.txt 파일이 존재하지 않습니다."
  exit 1
fi

if [! -d output]; then
  mkdir output
fi

while IFS= read -r line
do
  crate_name=$(basename -s .git "$line")
  git clone $line
  ./unsafe-finder $crate_name > ./output/"$crate_name.txt"
  rm -rf $crate_name
done < input.txt

# clean up
rm -rf target
rm unsafe-finder
