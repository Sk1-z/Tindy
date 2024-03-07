cargo build --release
mkdir build
cp target/release/tim build/
cp -r py build/
tar -cf tindy.tar build/
