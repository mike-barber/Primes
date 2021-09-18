#!/bin/bash

rm -rf target
mkdir -p target
ghc -o target/Primes -outputdir target \
    -O2 -fllvm \
    -optlc -O3 \
    -optlo -O3 \
    Primes.hs