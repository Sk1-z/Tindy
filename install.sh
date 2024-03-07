./build.sh
mkdir ~/.tindy
cp tindy.tar ~/.tindy
cd ~/.tindy
tar -xf tindy.tar
chmod u+x tim
cp -r build/** .
rm -r build/ tindy.tar

