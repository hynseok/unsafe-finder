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

while IFS= read -r line
do
  folder_name=$(basename -s .git "$line")
  git clone $line
  ./unsafe-finder $folder_name >> output.txt
  rm -rf $folder_name
done < input.txt

# clean up
rm -rf target
rm unsafe-finder
