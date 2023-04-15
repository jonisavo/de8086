#!/bin/sh

FILE=$1

[ -z "$FILE" ] && FILE=./test/kitchen_sink
DE_FILE=${FILE}_de

cargo run --release $FILE > ${DE_FILE}.asm
nasm ${DE_FILE}.asm

xxd $DE_FILE > de_file_xxd.txt
xxd $FILE > file_xxd.txt

diff -y de_file_xxd.txt file_xxd.txt
echo $?

rm de_file_xxd.txt file_xxd.txt ${DE_FILE}.asm ${DE_FILE}
