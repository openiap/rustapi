const { Client, ClientError } = require('./main');
const fs = require('fs');
(async () => {
    try {
        // const url = 'http://localhost:50051';
        // const url = 'http://grpc.localhost.openiap.io/';
        // const url = 'https://grpc.localhost.openiap.io/';
        // const url = 'https://grpc.demo.openiap.io/';
        // const url = 'https://grpc.app.openiap.io/';
        const url = '';
        const client = new Client();
        await client.connect(url);
        console.log("NodeJS:: connect completed, now call signin() again")
        const signin_result = await client.signin();
        console.log("NodeJS:: signin() complete")
        // console.log(signinResult);
        if(signin_result.success) {
            console.log("signed in", signin_result.success);
            let promises = [];
            for(y = 0; y < 1; y++) {
                for(let i = 0; i < 5; i++) {
                    const query_result = await client.query({ collectionname: 'entities', query: '{}', projection: '{"name":1}', orderby: '{}', queryas: '', explain: false, skip: 0, top: 0 });
                    console.log(query_result.results);
                    // promises.push(client.query({ collectionname: 'entities', query: '{}', projection: '{"name":1}', orderby: '{}', queryas: '', explain: false, skip: 0, top: 0 }));

                    
                }
                console.log(
                    (await Promise.all(promises)).map(result => result.results)
                );
                promises = [];
            }
            // console.log("AWAIT ALL begin")
            // console.log(
            //     (await Promise.all(promises)).map(result => result.results)
            // );
            // console.log("AWAIT ALL complete")
            // await new Promise(resolve => setTimeout(resolve, 2000));

            const aggregate_result = await client.aggregate({ collectionname: 'entities', aggregates: '[]', explain: false });
            if(aggregate_result.success == false) {
                console.log("aggregate failed", aggregate_result.error);
            } else {
                console.log("aggregate success", aggregate_result.results);
            }


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
            
            const upload_result = await client.upload({ filepath, filename: 'node-test.csv', mimetype: '', metadata: '', collectionname: 'fs.files' });
            if(upload_result.success == false) {
                console.log("upload failed", upload_result.error);
            } else {
                console.log("upload success", upload_result.id);
            }

            let eventcount = 0;
            const watch_result = await client.watch({ collectionname: 'entities', paths: ''}, (event) => {
                event.document = JSON.parse(event.document);
                console.log("watch " + event.operation + " for " + event.document._type + " / " + event.document.test);
                eventcount++;
            });
            if(watch_result.success == false) {
                console.log("watch failed", watch_result.error);
            } else {
                console.log("watch created as", watch_result.watchid);
            }

            while(eventcount < 2) {
                await new Promise(resolve => setTimeout(resolve, 1000));
            }

            const unwatch_result = client.unwatch(watch_result.watchid);
            if(unwatch_result.success == false) {
                console.log("remove watch failed", unwatch_result.error);
            } else {
                console.log("remove watch success");
            }

            console.log("*********************************")
            console.log("done, free client");
            console.log("*********************************")
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
