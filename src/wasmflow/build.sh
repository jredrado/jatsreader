#!/bin/sh

cd hex
make
cd ..

cd register
make
cd ..

cd locator  
make
cd ..

cd locatorverifier
make
cd ..

cd manifest
make
cd ..  

cd manifestverifier
make 
cd ..

cd resource
make
cd ..  

cd resourceverifier
make
cd ..

cd storage
make
cd ..

cd rsstreamer 
cargo build --release
cd ..