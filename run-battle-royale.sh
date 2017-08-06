#!/bin/bash

set -ev

cargo build
cargo build --release
cp target/release/punter target/release/punter-* .
rm -f *.d
python2 battle-royale.py ./punter ./punter-*
