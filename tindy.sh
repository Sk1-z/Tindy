git clone https://github.com/Sk1-z/Tindy.git
cargo build --release

mkdir ~/.tindy
cp target/release/tim ~/.tindy/
cp -r py ~/.tindy/
chmod u+x tim

echo "The executable can be added to the path with a symlink to ~/.tindy/tim. The app will not run outside of the directory it was installed to by this script. This build directory can be removed now."
