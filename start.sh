#!/bin/bash

# build
# 변경 사항이 있을 때 마다 바이너리를 삭제해줘야되서 항상 실행되도록 주석 처리해두었습니다.

#if [ ! -f unsafe-finder ]; then
  cargo build
  cd target/debug
  mv unsafe-finder ../../
  cd ../../
  rm -rf target
#fi

# check input file
if [ ! -f ./input.txt ]; then
  echo "Error: input.txt 파일이 존재하지 않습니다."
  exit 1
fi

if [ ! -d output ]; then
  mkdir output
fi

while IFS= read -r line
do
  crate_name=$(basename -s .git "$line")
  git clone $line </dev/null
  ./unsafe-finder $crate_name > ./output/"$crate_name.txt"
  rm -rf $crate_name
done < input.txt
