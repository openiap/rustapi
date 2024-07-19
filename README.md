# rust api for openiap and warppers for nodejs, python and dotnet7

build the rust library
```bash
cargo build
```
setup default credentials

```bash
export OPENIAP_USERNAME=username
export OPENIAP_PASSWORD=password
```

build and test nodejs
```bash
cd node
rm -rf lib *.tgz && mkdir lib && cp ../target/debug/libopeniap.so ./lib && npm pack
node test.js
```

build and test python
```bash
cd python
rm -rf build dist openiap/lib && mkdir openiap/lib && cp ../target/debug/libopeniap.so ./openiap/lib && python -m build --wheel
pip uninstall openiap -y && pip install dist/openiap-0.1.1-py3-none-any.whl && python test.py

```

build and test dotnet
```bash
cd dotnet
rm -rf bin lib && mkdir lib && cp ../target/debug/libopeniap.so ./lib && dotnet build && dotnet pack -p:NuspecFile=openiap.nuspec
dotnet run
```