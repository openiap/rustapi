# Sure, go a head, read me...
setup default credentials
```bash
export OPENIAP_USERNAME=username
export OPENIAP_PASSWORD=password
```

build and test nodejs
```bash
rm -rf bin lib && mkdir lib && cp ../target/debug/libopeniap.so ./lib && cp ../target/debug/libopeniap.dylib ./lib && dotnet build && dotnet pack -p:NuspecFile=openiap.nuspec
dotnet run
```