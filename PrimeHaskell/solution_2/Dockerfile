FROM debian:11 as build

# RUN apt-get -y update && apt-get -y upgrade && \
#     apt-get -y install wget xz-utils build-essential libgmp-dev llvm libtinfo5 libnuma1 && \
#     mkdir /ghc-haskell && cd /ghc-haskell && \
# # install GHC 8.10.6 from binary distobuiton...
# #    wget -q https://downloads.haskell.org/~ghc/8.10.7/ghc-8.10.7-x86_64-deb9-linux.tar.xz && \
# #    tar -axf ghc-8.10.7-x86_64-deb9-linux.tar.xz && \
# # the following for aarm64:
#    wget https://downloads.haskell.org/~ghc/8.10.7/ghc-8.10.7-aarch64-deb10-linux.tar.xz && \
#    tar -axf ghc-8.10.7-aarch64-deb10-linux.tar.xz && \
#     cd ghc-8.10.7 && ./configure && make install && cd / && rm -rdf /ghc-haskell

# install dependencies
RUN apt-get -y update && apt-get install -y ghc llvm 

# build the app
WORKDIR /opt/sieve
COPY Primes*.hs ./
RUN ghc Primes.hs

# runtime container
FROM debian:11 AS runtime
WORKDIR /opt/sieve
COPY --from=build /opt/sieve/Primes Primes
ENTRYPOINT [ "/opt/sieve/Primes" ]
