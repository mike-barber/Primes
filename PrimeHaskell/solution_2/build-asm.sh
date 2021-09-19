#!/bin/bash

# get cpu list with 
#   llc --march=x86-64 --mcpu=help

ARCH="x86-64"
CPU="$1"
echo "CPU is $CPU"

# rm -rf target
# mkdir -p target

# read: https://downloads.haskell.org/~ghc/latest/docs/html/users_guide/debugging.html#llvm-code-generator

FLAGS="-outputdir target \
    -O2 -fllvm \
    -optlc -O3 -optlc --march=$ARCH -optlc --mcpu=$CPU \
    -optlo -O3 -optlo --march=$ARCH -optlo --mcpu=$CPU \
    "

ghc -v PrimesTH.hs $FLAGS -S -o PrimesTH.hs
ghc -v PrimesNoLSR.hs $FLAGS -S -o PrimesNoLSR.s
ghc -v Primes.hs $FLAGS -S -o Primes.s

# ghc -o target/Primes -outputdir target \
#     -O2 -fllvm \
#     -optlc -O3 -optlc --march=$ARCH -optlc --mcpu=$CPU \
#     -optlo -O3 -optlo --march=$ARCH -optlo --mcpu=$CPU \
#     Primes.hs
