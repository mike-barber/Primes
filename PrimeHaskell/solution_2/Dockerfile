FROM debian:11

RUN apt-get -y update && apt-get -y upgrade && \
    apt-get -y install wget xz-utils build-essential libgmp-dev llvm libtinfo5 && \
    mkdir /ghc-haskell && cd /ghc-haskell && \
# install GHC 8.10.6 from binary distobuiton...
    wget -q https://downloads.haskell.org/~ghc/8.10.7/ghc-8.10.7-x86_64-deb9-linux.tar.xz && \
    tar -axf ghc-8.10.7-x86_64-deb9-linux.tar.xz && \
# the following for aarm64:
#   wget https://downloads.haskell.org/~ghc/8.10.7/ghc-8.10.7-aarch64-deb10-linux.tar.xz && \
#   tar -axf ghc-8.10.7-aarch64-deb10-linux.tar.xz && \
    cd ghc-8.10.7 && ./configure && make install && cd / && rm -rdf /ghc-haskell

# all installations into Debioan 10 finished, buld the app...
WORKDIR /opt/sieve

COPY Primes*.hs ./

RUN ghc Primes.hs

# to run it...
ENTRYPOINT [ "/opt/sieve/Primes" ]
