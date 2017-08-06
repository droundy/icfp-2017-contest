#!/bin/bash

set -ev

cargo build
./INSTALL
python2 battle-royale.py --max 10000 ./punter ./punter-*
