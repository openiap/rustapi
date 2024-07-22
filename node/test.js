const { Client, ClientError } = require('./main');
const fs = require('fs');
(async () => {
    try {
        const client = new Client('');
        // const client = new Client('http://localhost:50051');
        // const client = new Client('http://grpc.localhost.openiap.io/');
        // const client = new Client('https://grpc.localhost.openiap.io/');
        // const client = new Client('https://grpc.demo.openiap.io/');
        const signin_result = client.signin();
        // console.log(signinResult);
        if(signin_result.success) {
            console.log("signed in", signin_result.success);
            for(let i = 0; i < 1; i++) {
                const query_result = client.query({ collectionname: 'entities', query: '{}', projection: '{"name":1}', orderby: '{}', queryas: '', explain: false, skip: 0, top: 0 });
                console.log(query_result.results);
            }

            // stay around a little, so we can enjoy watching the client connected to the server
            // await new Promise(resolve => setTimeout(resolve, 60000));

            const download_result = client.download({ collectionname: 'fs.files', id: '65a3aaf66d52b8c15131aebd', folder: '', filename: '' });
            if(download_result.success == false) {
                console.log("download failed", download_result.error);
            } else {
                console.log("download success", download_result.filename);
            }
            
            let filepath = 'testfile.csv';
            if(!fs.existsSync(filepath)) {
                filepath = '../testfile.csv';
            }
            
            const upload_result = client.upload({ filepath, filename: 'node-test.csv', mimetype: '', metadata: '', collectionname: 'fs.files' });
            if(upload_result.success == false) {
                console.log("upload failed", upload_result.error);
            } else {
                console.log("upload success", upload_result.id);
            }

            client.free();
        } else {
            console.log("signed failed", signin_result.error);
        }
    } catch (error) {
        if (error instanceof ClientError) {
            console.error(`An error occurred: ${error.message}`);
        } else {
            console.error(`An unexpected error occurred: ${error}`);
        }
    }
})();
