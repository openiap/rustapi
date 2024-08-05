# rustup target add x86_64-apple-darwin
# rustup target add x86_64-pc-windows-gnu
#cargo build --release
# cargo build --target x86_64-pc-windows-gnu --release
# cargo build --target x86_64-apple-darwin --release
# rm -rf node/lib *.tgz && mkdir node/lib && cp target/release/libopeniap.so node/lib && cp target/x86_64-pc-windows-gnu/release/openiap.dll node/lib && (cd node && npm pack)
# rm -rf dotnet/lib *.tgz && mkdir dotnet/lib && cp target/release/libopeniap.so dotnet/lib && cp target/x86_64-pc-windows-gnu/release/openiap.dll dotnet/lib && (cd dotnet && dotnet build)
# cargo build --target x86_64-apple-darwin --release
# cargo build -Zbuild-std --target x86_64-apple-darwin --release

rm -rf target/lib
mkdir -p target/lib
cross build --target x86_64-unknown-linux-gnu --release && cp target/x86_64-unknown-linux-gnu/release/libopeniap.so target/lib/libopeniap-linux-x64.so
#cross build --target aarch64-unknown-linux-gnu --release && cp target/aarch64-unknown-linux-gnu/release/libopeniap.so target/lib/libopeniap-linux-arm64.so
#cross build --target x86_64-unknown-linux-musl --release && cp target/x86_64-unknown-linux-musl/release/libopeniap.a target/lib/libopeniap-linux-musl-x64.a
#cross build --target aarch64-unknown-linux-musl --release && cp target/aarch64-unknown-linux-musl/release/libopeniap.a target/lib/libopeniap-linux-musl-arm64.a
#cross build --target x86_64-unknown-freebsd --release && cp target/x86_64-unknown-freebsd/release/libopeniap.so target/lib/libopeniap-freebsd-x64.so

cross build --target x86_64-pc-windows-gnu --release && cp target/x86_64-pc-windows-gnu/release/openiap.dll target/lib/openiap-windows-x64.dll
#cross build --target i686-pc-windows-gnu --release && cp target/i686-pc-windows-gnu/release/openiap.dll target/lib/openiap-windows-x86.dll

cross build --target aarch64-apple-darwin --release && cp target/aarch64-apple-darwin/release/libopeniap.dylib target/lib/libopeniap-macos-arm64.dylib
#cross build --target x86_64-apple-darwin --release && cp target/x86_64-apple-darwin/release/libopeniap.dylib target/lib/libopeniap-macos-x64.dylib

rm -rf node/lib *.tgz && mkdir node/lib && cp target/lib/* node/lib && (cd node && npm pack)
rm -rf dotnet/lib *.tgz && mkdir dotnet/lib && cp target/lib/* dotnet/lib && (cd dotnet && dotnet build --configuration Release && dotnet pack -p:NuspecFile=openiap.nuspec --configuration Release)
rm -rf python/lib *.tgz && mkdir python/lib && cp target/lib/* python/lib && (cd python && python setup.py sdist)



# ring is broken again
# cross build --target aarch64-pc-windows-msvc --release && cp target/aarch64-pc-windows-msvc/release/openiap.dll target/lib/openiap-windows-arm64.dll
