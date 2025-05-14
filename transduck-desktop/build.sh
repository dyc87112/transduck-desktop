# build macos client
echo "Start build MacOS client..."
npm run tauri build 
cp src-tauri/target/release/bundle/dmg/*.dmg ../release/

# build windows x64 client
echo "Start build windows x64 client..."
npm run tauri build -- --runner cargo-xwin --target x86_64-pc-windows-msvc
cp src-tauri/target/x86_64-pc-windows-msvc/release/bundle/nsis/*.exe ../release/

# build windows arm client
echo "Start build windows arm client..."
npm run tauri build -- --runner cargo-xwin --target aarch64-pc-windows-msvc
cp src-tauri/target/aarch64-pc-windows-msvc/release/bundle/nsis/*.exe ../release/