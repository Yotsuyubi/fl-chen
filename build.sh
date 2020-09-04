#!/bin/bash

cd view/
npm run build
cd ../
cargo build --release
sh osx_vst_bundler.sh FL-Chen target/release/libbasicvst.dylib
