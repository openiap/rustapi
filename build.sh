
rm -rf node/lib node/*.tgz node/*.csv dotnet/lib dotnet/*.csv dotnet/bin dotnet/obj python/openiap/lib python//*.csv python/build python/dist
rm -rf target/lib
mkdir -p target/lib
cross build --target x86_64-unknown-linux-gnu --release && cp target/x86_64-unknown-linux-gnu/release/libopeniap_clib.so target/lib/libopeniap-linux-x64.so
cross build --target aarch64-unknown-linux-gnu --release && cp target/aarch64-unknown-linux-gnu/release/libopeniap_clib.so target/lib/libopeniap-linux-arm64.so
# skip for now, to save space
# cross build --target x86_64-unknown-linux-musl --release && cp target/x86_64-unknown-linux-musl/release/libopeniap_clib.a target/lib/libopeniap-linux-musl-x64.a
# cross build --target aarch64-unknown-linux-musl --release && cp target/aarch64-unknown-linux-musl/release/libopeniap_clib.a target/lib/libopeniap-linux-musl-arm64.a
# cross build --target x86_64-unknown-freebsd --release && cp target/x86_64-unknown-freebsd/release/libopeniap_clib.so target/lib/libopeniap-freebsd-x64.so

cross build --target aarch64-apple-darwin --release && cp target/aarch64-apple-darwin/release/libopeniap_clib.dylib target/lib/libopeniap-macos-arm64.dylib
cross build --target x86_64-apple-darwin --release && cp target/x86_64-apple-darwin/release/libopeniap_clib.dylib target/lib/libopeniap-macos-x64.dylib

cross build --target x86_64-pc-windows-gnu -v --release && cp target/x86_64-pc-windows-gnu/release/openiap_clib.dll target/lib/openiap-windows-x64.dll
cross build --target i686-pc-windows-gnu -v --release && cp target/i686-pc-windows-gnu/release/openiap_clib.dll target/lib/openiap-windows-x86.dll

echo "Building node"
rm -rf node/lib *.tgz && mkdir node/lib && cp target/lib/* node/lib && (cd node && npm pack)
(cd node && npm publish)
echo "Building dotnet"
rm -rf dotnet/lib && mkdir dotnet/lib && cp target/lib/* dotnet/lib && (cd dotnet && dotnet build --configuration Release && dotnet pack -p:NuspecFile=openiap.nuspec --configuration Release) 
(cd dotnet && dotnet nuget push packages/openiap.0.0.6.nupkg --source https://api.nuget.org/v3/index.json --api-key $NUGET_API_KEY)

echo "Building python"
rm -rf python/openiap/lib  build dist lib && mkdir -p python/openiap/lib && cp target/lib/* python/openiap/lib && (cd python && python setup.py sdist) 
(cd python && python3 -m twine upload dist/*)


cargo publish -p openiap-proto --allow-dirty
cargo publish -p openiap-client --allow-dirty
cargo publish -p openiap --allow-dirty
cargo publish -p openiap-clib --allow-dirty
# 
echo "done"
# cross build --target aarch64-pc-windows-msvc --release && cp target/aarch64-pc-windows-msvc/release/openiap.dll target/lib/openiap-windows-arm64.dll
