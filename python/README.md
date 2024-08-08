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

test python
```bash
rm -rf build dist lib && mkdir lib && cp ../target/lib/* ./lib && python -m build --wheel
pip uninstall openiap -y && pip install dist/openiap-0.1.1-py3-none-any.whl && python test.py
```