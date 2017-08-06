#!/bin/bash

set -ev

cargo build
./INSTALL
python2 battle-royale.py ./punter ./punter-*
