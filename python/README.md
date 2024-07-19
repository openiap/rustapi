# Sure, go a head, read me...
setup default credentials
```bash
export OPENIAP_USERNAME=username
export OPENIAP_PASSWORD=password
```

build and test nodejs
```bash
rm -rf build dist openiap/lib && mkdir openiap/lib && cp ../target/debug/libopeniap.so ./openiap/lib && python -m build --wheel
pip uninstall openiap -y && pip install dist/openiap-0.1.1-py3-none-any.whl && python test.py
```