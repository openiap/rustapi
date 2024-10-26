
rm -rf node/lib node/*.tgz node/*.csv dotnet/lib dotnet/*.csv dotnet/bin dotnet/obj python/openiap/lib python//*.csv python/build python/dist
rm -rf target/lib target/cli
mkdir -p target/lib
mkdir -p target/cli
cross build --target x86_64-unknown-linux-gnu --release && cp target/x86_64-unknown-linux-gnu/release/libopeniap_clib.so target/lib/libopeniap-linux-x64.so
cp target/x86_64-unknown-linux-gnu/release/openiap target/cli/linux-x64-openiap
cross build --target aarch64-unknown-linux-gnu --release && cp target/aarch64-unknown-linux-gnu/release/libopeniap_clib.so target/lib/libopeniap-linux-arm64.so
cp target/aarch64-unknown-linux-gnu/release/openiap target/cli/linux-arm64-openiap
# skip for now, to save space
# cross build --target x86_64-unknown-linux-musl --release && cp target/x86_64-unknown-linux-musl/release/libopeniap_clib.a target/lib/libopeniap-linux-musl-x64.a
# cross build --target aarch64-unknown-linux-musl --release && cp target/aarch64-unknown-linux-musl/release/libopeniap_clib.a target/lib/libopeniap-linux-musl-arm64.a
# cross build --target x86_64-unknown-freebsd --release && cp target/x86_64-unknown-freebsd/release/libopeniap_clib.so target/lib/libopeniap-freebsd-x64.so

cross build --target aarch64-apple-darwin --release && cp target/aarch64-apple-darwin/release/libopeniap_clib.dylib target/lib/libopeniap-macos-arm64.dylib
cp target/aarch64-apple-darwin/release/openiap target/cli/macos-arm64-openiap
cross build --target x86_64-apple-darwin --release && cp target/x86_64-apple-darwin/release/libopeniap_clib.dylib target/lib/libopeniap-macos-x64.dylib
cp target/x86_64-apple-darwin/release/openiap target/cli/macos-x64-openiap

cross build --target x86_64-pc-windows-gnu -v --release && cp target/x86_64-pc-windows-gnu/release/openiap_clib.dll target/lib/openiap-windows-x64.dll
cp target/x86_64-pc-windows-gnu/release/openiap.exe target/cli/windows-x64-openiap.exe
cross build --target i686-pc-windows-gnu -v --release && cp target/i686-pc-windows-gnu/release/openiap_clib.dll target/lib/openiap-windows-i686.dll
cp target/i686-pc-windows-gnu/release/openiap.exe target/cli/windows-i686-openiap.exe

echo "done"
# cross build --target aarch64-pc-windows-msvc --release && cp target/aarch64-pc-windows-msvc/release/openiap.dll target/lib/openiap-windows-arm64.dll
