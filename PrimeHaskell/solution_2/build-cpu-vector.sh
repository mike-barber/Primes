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
    -pgmlo /usr/lib/llvm-12/bin/opt \
    -pgmlc /usr/lib/llvm-12/bin/llc \
    -optlc -O3 -optlc --march=$ARCH -optlc --mcpu=$CPU -optlc --vectorize-slp \
    -optlo -O3 -optlo --march=$ARCH -optlo --mcpu=$CPU -optlo --slp-vectorizer -optlo --vector-combine \
    Primes.hs
