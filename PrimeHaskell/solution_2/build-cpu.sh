#!/bin/bash

# get cpu list with 
#   llc --march=x86-64 --mcpu=help

ARCH="x86-64"
CPU="$1"
echo "CPU is $CPU"

rm -rf target
mkdir -p target
ghc -o target/Primes -outputdir target -optlc --march=$ARCH -optlc --mcpu=$CPU -optlo --march=$ARCH -optlo --mcpu=$CPU Primes.hs
