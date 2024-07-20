const { Client, ClientError } = require('./main');
(async () => {
    try {
        const client = new Client('http://localhost:50051');
        // const client = new Client('http://grpc.localhost.openiap.io/');
        // const client = new Client('https://grpc.localhost.openiap.io/');
        // const client = new Client('https://grpc.demo.openiap.io/');
        const signinResult = client.signin();
        // console.log(signinResult);
        if(signinResult.success) {
            console.log("signed in", signinResult.success);
            for(let i = 0; i < 1; i++) {
                const result = client.query({ collectionname: 'entities', query: '{}', projection: '{"name":1}', orderby: '{}', queryas: '', explain: false, skip: 0, top: 0 });
                console.log(result.results);
            }

            // stay around a little, so we can enjoy watching the client connected to the server
            // await new Promise(resolve => setTimeout(resolve, 30000));



            client.free();
        } else {
            console.log("signed failed", signinResult.error);
        }
    } catch (error) {
        if (error instanceof ClientError) {
            console.error(`An error occurred: ${error.message}`);
        } else {
            console.error(`An unexpected error occurred: ${error}`);
        }
    }
})();
