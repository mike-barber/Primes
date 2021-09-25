#!/bin/bash

rm -rf target tmp
mkdir -p target tmp
ghc -v -o target/Primes -outputdir target \
    -keep-tmp-files -tmpdir ./tmp \
    -O2 -fllvm -v \
    -optlc -O3 \
    -optlo -O3 \
    Primes.hs