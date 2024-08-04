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
        // client.enable_tracing("openiap=info", "close");
        // client.enable_tracing("openiap=debug", "new");
        client.enable_tracing("openiap=debug", "");
        await client.connect(url);
        client.log("NodeJS:: connect completed, now call signin() again")
        const signin_result = await client.signin();
        client.log("NodeJS:: signin() complete")
        // client.log(signinResult);
        if(signin_result.success) {
            client.log("signed in", signin_result.success);
            let promises = [];
            for(y = 0; y < 1; y++) {
                for(let i = 0; i < 20; i++) {
                    // const query_result = await client.query({ collectionname: 'entities', query: '{}', projection: '{"name":1}', orderby: '{}', queryas: '', explain: false, skip: 0, top: 0 });
                    // console.log("Got ", query_result.results.length, " results");
                    promises.push(client.query({ collectionname: 'entities', query: '{}', projection: '{"name":1}', orderby: '{}', queryas: '', explain: false, skip: 0, top: 0 }));

                    
                }
                client.log(
                    (await Promise.all(promises)).map(result => result.results)
                );
                promises = [];
            }
            // client.log("AWAIT ALL begin")
            // client.log(
            //     (await Promise.all(promises)).map(result => result.results)
            // );
            // client.log("AWAIT ALL complete")
            // await new Promise(resolve => setTimeout(resolve, 2000));

            const aggregate_result = await client.aggregate({ collectionname: 'entities', aggregates: '[]', explain: false });
            if(aggregate_result.success == false) {
                console.error("aggregate failed", aggregate_result.error);
            } else {
                console.log("aggregate success", aggregate_result.results.length, " results");
            }

            const insert_one_result = await client.insert_one({ collectionname: 'entities', document: '{"name":"test from nodejs", "_type": "test"}' });
            if(insert_one_result.success == false) {
                console.error("insert_one failed", insert_one_result.error);
            } else {
                console.log("insert_one success", insert_one_result.id);
            }

            const download_result = client.download({ collectionname: 'fs.files', id: '65a3aaf66d52b8c15131aebd', folder: '', filename: '' });
            if(download_result.success == false) {
                console.error("download failed", download_result.error);
            } else {
                console.log("download success", download_result.filename);
            }
            
            let filepath = 'testfile.csv';
            if(!fs.existsSync(filepath)) {
                filepath = '../testfile.csv';
            }
            
            const upload_result = await client.upload({ filepath, filename: 'node-test.csv', mimetype: '', metadata: '', collectionname: 'fs.files' });
            if(upload_result.success == false) {
                console.error("upload failed", upload_result.error);
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
                console.error("watch failed", watch_result.error);
            } else {
                console.log("watch created as", watch_result.watchid);
            }

            while(eventcount < 2) {
                await new Promise(resolve => setTimeout(resolve, 1000));
            }

            const unwatch_result = client.unwatch(watch_result.watchid);
            if(unwatch_result.success == false) {
                console.error("remove watch failed", unwatch_result.error);
            } else {
                console.log("remove watch success");
            }
            var count_result = await client.count({ collectionname: 'entities', query: '{}', queryas: '' });
            if(count_result.success == false) {
                console.error("count failed", count_result.error);
            } else {
                console.log("count success", count_result.result);
            }
            var distinct_result = await client.distinct({ collectionname: 'entities', field: '_type' });
            if(distinct_result.success == false) {
                console.error("distinct failed", distinct_result.error);
            } else {
                console.log("distinct success", distinct_result.results);
            }

            client.log("*********************************")
            client.log("done, free client");
            client.log("*********************************")
            client.free();
        } else {
            client.log("signed failed", signin_result.error);
        }
    } catch (error) {
        if (error instanceof ClientError) {
            console.error(`An error occurred: ${error.message}`);
        } else {
            console.error(`An unexpected error occurred: ${error}`);
        }
    }
})();
