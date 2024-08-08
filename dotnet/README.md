# Sure, go a head, read me...
setup default credentials
```bash
export apiurl=grpc://grpc.app.openiap.io:443
# username/password
export OPENIAP_USERNAME=username
export OPENIAP_PASSWORD=password
# or better, use a jwt token ( open https://app.openiap.io/jwtlong and copy the jwt value)
export OPENIAP_JWT=eyJhbGciOiJI....
```

test dotnet
```bash
rm -rf lib && mkdir lib && cp ../target/lib/* lib && dotnet build --configuration Release && dotnet pack -p:NuspecFile=openiap.nuspec --configuration Release
dotnet run
```
