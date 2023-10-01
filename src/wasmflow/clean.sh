#!/bin/sh

cd hex
cargo clean
cd ..

cd register
cargo clean
cd ..

cd locator  
cargo clean
cd ..

cd locatorverifier
cargo clean
cd ..

cd manifest
cargo clean
cd ..  

cd manifestverifier
cargo clean 
cd ..

cd resource
cargo clean
cd ..  

cd resourceverifier
cargo clean
cd ..

cd storage
cargo clean
cd ..

cd rsstreamer 
cargo clean
cd ..