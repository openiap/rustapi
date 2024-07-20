# rust api for openiap and warppers for nodejs, python and dotnet7

make sure you have rust installed
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
make sure protoc is installed
```bash
# linux
sudo apt install protobuf-compiler
# macos
brew install protobuf
# windows
# download at https://github.com/protocolbuffers/protobuf/releases
```
install os targets
```
rustup target add x86_64-pc-windows-msvc
rustup target add x86_64-apple-darwin
rustup target add x86_64-unknown-linux-gnu

```

build the rust library
```bash
cargo build
cargo build --target x86_64-apple-darwin --release
cargo build --target x86_64-unknown-linux-gnu --release
```
setup default credentials

```bash
export OPENIAP_USERNAME=username
export OPENIAP_PASSWORD=password
```

build and test nodejs
```bash
cd node
rm -rf lib *.tgz && mkdir lib && cp ../target/debug/libopeniap.so ./lib && cp ../target/debug/libopeniap.dylib ./lib && npm pack
node test.js
```


build and test python
```bash
cd python
rm -rf build dist openiap/lib && mkdir openiap/lib && cp ../target/debug/libopeniap.so ./openiap/lib && cp ../target/debug/libopeniap.dylib ./lib && python -m build --wheel
pip uninstall openiap -y && pip install dist/openiap-0.1.1-py3-none-any.whl && python test.py

```

build and test dotnet
```bash
cd dotnet
rm -rf bin lib && mkdir lib && cp ../target/debug/libopeniap.so ./lib && cp ../target/debug/libopeniap.dylib ./lib && dotnet build && dotnet pack -p:NuspecFile=openiap.nuspec
dotnet run
```