
rm -rf node/lib node/*.tgz node/*.csv dotnet/lib dotnet/*.csv dotnet/bin dotnet/obj python/lib python//*.csv python/build python/dist python/lib 
rm -rf target/lib
mkdir -p target/lib
cross build --target x86_64-unknown-linux-gnu --release && cp target/x86_64-unknown-linux-gnu/release/libopeniap_clib.so target/lib/libopeniap-linux-x64.so
cross build --target aarch64-unknown-linux-gnu --release && cp target/aarch64-unknown-linux-gnu/release/libopeniap_clib.so target/lib/libopeniap-linux-arm64.so
cross build --target x86_64-unknown-linux-musl --release && cp target/x86_64-unknown-linux-musl/release/libopeniap_clib.a target/lib/libopeniap-linux-musl-x64.a
cross build --target aarch64-unknown-linux-musl --release && cp target/aarch64-unknown-linux-musl/release/libopeniap_clib.a target/lib/libopeniap-linux-musl-arm64.a
cross build --target x86_64-unknown-freebsd --release && cp target/x86_64-unknown-freebsd/release/libopeniap_clib.so target/lib/libopeniap-freebsd-x64.so

cross build --target aarch64-apple-darwin --release && cp target/aarch64-apple-darwin/release/libopeniap_clib.dylib target/lib/libopeniap-macos-arm64.dylib
cross build --target x86_64-apple-darwin --release && cp target/x86_64-apple-darwin/release/libopeniap_clib.dylib target/lib/libopeniap-macos-x64.dylib

cross build --target x86_64-pc-windows-gnu -v --release && cp target/x86_64-pc-windows-gnu/release/openiap_clib.dll target/lib/openiap-windows-x64.dll
cross build --target i686-pc-windows-gnu -v --release && cp target/i686-pc-windows-gnu/release/openiap_clib.dll target/lib/openiap-windows-x86.dll

echo "Building node"
rm -rf node/lib *.tgz && mkdir node/lib && cp target/lib/* node/lib && (cd node && npm pack)
echo "Building dotnet"
rm -rf dotnet/lib && mkdir dotnet/lib && cp target/lib/* dotnet/lib && (cd dotnet && dotnet build --configuration Release && dotnet pack -p:NuspecFile=openiap.nuspec --configuration Release)
echo "Building python"
rm -rf python/lib build dist lib && mkdir python/lib && cp target/lib/* python/lib && (cd python && python setup.py sdist)

cargo publish -p openiap-proto --allow-dirty
cargo publish -p openiap-client --allow-dirty
cargo publish -p openiap --allow-dirty
cargo publish -p openiap-clib --allow-dirty
# 
echo "done"
# cross build --target aarch64-pc-windows-msvc --release && cp target/aarch64-pc-windows-msvc/release/openiap.dll target/lib/openiap-windows-arm64.dll
