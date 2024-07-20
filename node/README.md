# Sure, go a head, read me...
setup default credentials
```bash
export OPENIAP_USERNAME=username
export OPENIAP_PASSWORD=password
```

build and test nodejs
```bash
rm -rf lib *.tgz && mkdir lib && cp ../target/release/libopeniap.* ./lib  && npm pack
node test.js
```