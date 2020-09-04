#!/bin/bash

cd view/
npm run build
cd ../

cargo build --release
sh osx_vst_bundler.sh FL-Chen target/release/libbasicvst.dylib
mv FL-Chen.vst dest/FL-Chen.vst

cargo build --target x86_64-pc-windows-gnu --release
mv target/x86_64-pc-windows-gnu/release/basicvst.dll dest/FL-Chen.dll
