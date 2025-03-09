watch:
  watchexec just build

build:
  cmake --build build -j 28 -- bitcoind

init:
  cmake -B build

d:
  ./build/src/bitcoind -datadir=datadir

schema:
  ./build/src/bitcoin-cli -datadir=datadir schema | jq > schema.json
