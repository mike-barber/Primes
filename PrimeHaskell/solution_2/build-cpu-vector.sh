#!/bin/bash

# get cpu list with 
#   llc --march=x86-64 --mcpu=help

ARCH="x86-64"
TRIPLE="x86_64-pc-linux-gnu"
CPU="$1"
echo "CPU is $CPU"

rm -rf target tmp
mkdir -p target tmp
ghc -v -o target/Primes -outputdir target \
    -keep-tmp-files -tmpdir ./tmp \
    -O2 -fllvm \
    -pgmlo /usr/lib/llvm-12/bin/opt \
    -pgmlc /usr/lib/llvm-12/bin/llc \
    -optlc -O3 -optlc -mtriple=$TRIPLE -optlc -march=$ARCH -optlc -mcpu=$CPU -optlc -mattr=+avx,+avx2 -optlc --vectorize-slp \
    -optlo -O3 -optlo -mtriple=$TRIPLE -optlo -march=$ARCH -optlo -mcpu=$CPU -optlo -mattr=+avx,+avx2 -optlo --slp-vectorizer -optlo --vector-combine \
    Primes.hs 2>&1
