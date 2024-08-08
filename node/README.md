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

build and test nodejs
```bash
rm -rf lib *.tgz && mkdir lib && cp ../target/lib/* lib && npm pack
node test.js
```