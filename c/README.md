# getting started
compile and run in debug mode

setup default credentials
```bash
export apiurl=grpc://grpc.app.openiap.io:443
# username/password
export OPENIAP_USERNAME=username
export OPENIAP_PASSWORD=password
# or better, use a jwt token ( open https://app.openiap.io/jwtlong and copy the jwt value)
export OPENIAP_JWT=eyJhbGciOiJI....
```

Then compile and run the main program
```bash
cargo build
(cd c && gcc test_package/main.c -L../target/debug -Iinclude -lopeniap_clib -Wl,-rpath=../target/debug -o client_cli && ./client_cli )
```

Or when building for release, we will have a lib folder with the shared library and the client_cli executable
```bash
make build-all
(cd c && gcc test_package/main.c -Llib -Iinclude -l:libopeniap-linux-x64.so -Wl,-rpath=lib -o client_cli && ./client_cli)

```

see the Makefile for more information and options, make will compile for release by default
```bash
make
```
or
```bash
make debug
```
