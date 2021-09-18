#!/bin/bash

rm -rf target
mkdir -p target
ghc -o target/Primes -outputdir target Primes.hs
