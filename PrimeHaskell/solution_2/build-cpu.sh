#!/bin/bash
ARCH="x86-64"
CPU="$1"
echo "CPU is $CPU"
ghc -optlc --march=$ARCH -optlc --mcpu=$CPU -optlo --march=$ARCH -optlo --mcpu=$CPU Primes.hs