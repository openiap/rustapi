# rust api for openiap and wrappers for nodejs, python and dotnet6


# build
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
install [cross](https://github.com/cross-rs/cross) ( requires docker or podman )
```
cargo install cross --git https://github.com/cross-rs/cross
```
then compile for each target platform by running
```
sh build.sh
```

# test
setup default credentials

```bash
export OPENIAP_USERNAME=username
export OPENIAP_PASSWORD=password
```

rust
```bash
cargo run
cargo test
```

nodejs
```bash
cd node
node test.js
```

python
```bash
cd python
pip uninstall openiap -y && pip install dist/openiap-0.0.28-py3-none-any.whl && python test.py

```
dotnet
```bash
cd dotnet
dotnet run
```
