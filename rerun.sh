cargo build --release
cp target/release/rat rat/usr/bin
dpkg-deb --build rat
sudo dpkg -i rat.deb