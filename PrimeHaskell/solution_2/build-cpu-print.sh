#!/bin/bash

# get cpu list with 
#   llc --march=x86-64 --mcpu=help

ARCH="x86-64"
CPU="$1"
echo "CPU is $CPU"

rm -rf target
mkdir -p target
ghc -o target/Primes -outputdir target \
    -O2 -fllvm \
    -v \
    -optlc -O3 -optlc --march=$ARCH -optlc --mcpu=$CPU \
    -optlo -O3 -optlo --march=$ARCH -optlo --mcpu=$CPU \
    -optlo -print-after-all -optlo -filter-print-funcs='g2vGj_info$def' \
    Primes.hs 2>&1
