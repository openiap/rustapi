cargo build --release
# cargo build --target x86_64-pc-windows-gnu --release
rm -rf node/lib *.tgz && mkdir node/lib && cp target/release/libopeniap.so node/lib && cp target/x86_64-pc-windows-gnu/release/openiap.dll node/lib && (cd node && npm pack)
rm -rf dotnet/lib *.tgz && mkdir dotnet/lib && cp target/release/libopeniap.so dotnet/lib && cp target/x86_64-pc-windows-gnu/release/openiap.dll dotnet/lib && (cd dotnet && dotnet build)
# cargo build --target x86_64-apple-darwin --release
# cargo build -Zbuild-std --target x86_64-apple-darwin --release
# rustup target add x86_64-apple-darwin