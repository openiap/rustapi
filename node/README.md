# getting started
Install package
```bash
npm install openiap
```
Then use it in your code
```javascript
const { Client, ClientError } = require('./main');
client.enable_tracing("openiap=info", "");
client.connect();
const signin_result = client.signin();
if (signin_result.success) {
    client.log("signed in");
    const query_result = await client.query_async({ collectionname: 'entities', query: '{}', projection: '{"name":1}');
    console.log("Got", query_result.length, "results");
} else {
    console.log(signin_result.error);
}
```


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