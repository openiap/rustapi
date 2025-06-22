export CROSS_CONTAINER_ENGINE_NO_BUILDKIT=1
rm -rf node/lib node/*.tgz node/*.csv dotnet/lib dotnet/runtime dotnet/*.csv dotnet/bin dotnet/obj python/openiap/lib python//*.csv python/build python/dist
rm -rf target/lib target/cli
mkdir -p target/lib
mkdir -p target/cli
cross build --target x86_64-unknown-linux-gnu --release && cp target/x86_64-unknown-linux-gnu/release/libopeniap_clib.so target/lib/libopeniap-linux-x64.so
cp target/x86_64-unknown-linux-gnu/release/openiap target/cli/linux-x64-openiap
cross build --target aarch64-unknown-linux-gnu --release && cp target/aarch64-unknown-linux-gnu/release/libopeniap_clib.so target/lib/libopeniap-linux-arm64.so
cp target/aarch64-unknown-linux-gnu/release/openiap target/cli/linux-arm64-openiap
# skip for now, to save space
cross build --target x86_64-unknown-linux-musl --release && cp target/x86_64-unknown-linux-musl/release/libopeniap_clib.a target/lib/libopeniap-linux-musl-x64.a
cp target/x86_64-unknown-linux-musl/release/openiap target/cli/linux-musl-x64-openiap
cross build --target aarch64-unknown-linux-musl --release && cp target/aarch64-unknown-linux-musl/release/libopeniap_clib.a target/lib/libopeniap-linux-musl-arm64.a
cp target/aarch64-unknown-linux-musl/release/openiap target/cli/linux-musl-arm64-openiap
cross build --target x86_64-unknown-freebsd --release && cp target/x86_64-unknown-freebsd/release/libopeniap_clib.so target/lib/libopeniap-freebsd-x64.so
cp target/x86_64-unknown-freebsd/release/openiap target/cli/freebsd-x64-openiap

cross build --target aarch64-apple-darwin --release && cp target/aarch64-apple-darwin/release/libopeniap_clib.dylib target/lib/libopeniap-macos-arm64.dylib
cp target/aarch64-apple-darwin/release/openiap target/cli/macos-arm64-openiap
cross build --target x86_64-apple-darwin --release && cp target/x86_64-apple-darwin/release/libopeniap_clib.dylib target/lib/libopeniap-macos-x64.dylib
cp target/x86_64-apple-darwin/release/openiap target/cli/macos-x64-openiap

cross build --target x86_64-pc-windows-gnu -v --release && cp target/x86_64-pc-windows-gnu/release/openiap_clib.dll target/lib/openiap-windows-x64.dll
cp target/x86_64-pc-windows-gnu/release/openiap.exe target/cli/windows-x64-openiap.exe
cross build --target i686-pc-windows-gnu -v --release && cp target/i686-pc-windows-gnu/release/openiap_clib.dll target/lib/openiap-windows-i686.dll
cp target/i686-pc-windows-gnu/release/openiap.exe target/cli/windows-i686-openiap.exe

# build bootstrap helper for each target
cross build --target x86_64-unknown-linux-gnu --release -p openiap-bootstrap && cp target/x86_64-unknown-linux-gnu/release/openiap-bootstrap target/cli/linux-x64-bootstrap
cross build --target aarch64-unknown-linux-gnu --release -p openiap-bootstrap && cp target/aarch64-unknown-linux-gnu/release/openiap-bootstrap target/cli/linux-arm64-bootstrap
cross build --target x86_64-apple-darwin --release -p openiap-bootstrap && cp target/x86_64-apple-darwin/release/openiap-bootstrap target/cli/macos-x64-bootstrap
cross build --target aarch64-apple-darwin --release -p openiap-bootstrap && cp target/aarch64-apple-darwin/release/openiap-bootstrap target/cli/macos-arm64-bootstrap
cross build --target x86_64-pc-windows-gnu --release -p openiap-bootstrap && cp target/x86_64-pc-windows-gnu/release/openiap-bootstrap.exe target/cli/windows-x64-bootstrap.exe
cross build --target i686-pc-windows-gnu --release -p openiap-bootstrap && cp target/i686-pc-windows-gnu/release/openiap-bootstrap.exe target/cli/windows-i686-bootstrap.exe

echo "done"
# cross build --target aarch64-pc-windows-msvc --release && cp target/aarch64-pc-windows-msvc/release/openiap.dll target/lib/openiap-windows-arm64.dll


# https://learn.microsoft.com/en-us/dotnet/core/rid-catalog
# echo "Building dotnet"
# mkdir -p dotnet/runtimes/linux-arm64/native && cp target/aarch64-unknown-linux-gnu/release/libopeniap_clib.so dotnet/runtimes/linux-arm64/native/libopeniap_clib.so
# mkdir -p dotnet/runtimes/linux-x64/native && cp target/x86_64-unknown-linux-gnu/release/libopeniap_clib.so dotnet/runtimes/linux-x64/native/libopeniap_clib.so
# mkdir -p dotnet/runtimes/linux-musl-x64/native && cp target/x86_64-unknown-linux-musl/release/libopeniap_clib.a dotnet/runtimes/linux-musl-x64/native/libopeniap_clib.a
# mkdir -p dotnet/runtimes/linux-musl-arm64/native && cp target/aarch64-unknown-linux-musl/release/libopeniap_clib.a dotnet/runtimes/linux-musl-arm64/native/libopeniap_clib.a
# mkdir -p dotnet/runtimes/osx-arm64/native && cp target/aarch64-apple-darwin/release/libopeniap_clib.dylib dotnet/runtimes/osx-arm64/native/libopeniap_clib.dylib
# mkdir -p dotnet/runtimes/osx-x64/native && cp target/x86_64-apple-darwin/release/libopeniap_clib.dylib dotnet/runtimes/osx-x64/native/libopeniap_clib.dylib
# mkdir -p dotnet/runtimes/win-x64/native && cp target/x86_64-pc-windows-gnu/release/openiap_clib.dll dotnet/runtimes/win-x64/native/openiap_clib.dll
# mkdir -p dotnet/runtimes/win-x86/native && cp target/i686-pc-windows-gnu/release/openiap_clib.dll dotnet/runtimes/win-x86/native/openiap_clib.dll
# (cd dotnet && dotnet build --configuration Release openiap.csproj  && dotnet pack -p:NuspecFile=openiap.nuspec --configuration Release openiap.csproj) 
# (cd dotnet && dotnet build --configuration Release openiap-slim.csproj  && dotnet pack -p:NuspecFile=openiap.nuspec --configuration Release openiap-slim.csproj) 

# dotnet nuget push dotnet/bin/Release/openiap.0.0.16.nupkg --source https://api.nuget.org/v3/index.json --api-key $NUGET_API_KEY
