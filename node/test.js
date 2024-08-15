const { Client, ClientError } = require('./lib');
const fs = require('fs');
(async () => {
    let _int = setInterval(() => { 
        // console.log("Event loop is running"); 
    }, 200);
    try {
        // const url = 'http://localhost:50051';
        // const url = 'http://grpc.localhost.openiap.io/';
        // const url = 'https://grpc.localhost.openiap.io/';
        // const url = 'https://grpc.demo.openiap.io/';
        // const url = 'https://grpc.app.openiap.io/';
        const url = '';
        const client = new Client();

        let filepath = 'testfile.csv';
        if (!fs.existsSync(filepath)) {
            filepath = '../testfile.csv';
        }


        // console.log("test_add start.");
        // await client.test_add();
        // console.log("test_add done.");
        // console.log("test_add2 start.");
        // await client.test_add2();
        // console.log("test_add2 done.");

        // client.enable_tracing("openiap=debug", "close");
        client.enable_tracing("openiap=trace", "new");
        // client.enable_tracing("openiap=info", "");

        // setInterval(() => {
        //     console.log("Event loop is running");
        // }, 1000);

        // client.run_async_in_node(()=> {
        //     console.log("run_async_in_node done.");
        // })

        // console.log("endless loop");
        // while(true) {
        //     await new Promise(resolve => setTimeout(resolve, 1000));
        // }
        await client.connect_async(url);
        // await client.connect(url);
        client.info("connect completed, now call signin() again")
        const signin_result2 = await client.signin_async();
        if(signin_result2.success) {
            client.info("async signed in", signin_result2.success);
        } else {
            client.info("async signed failed", signin_result2.error);
        }
        const signin_result = client.signin();
        client.info("signin() complete")
        // client.info(signin_result);
        if (signin_result.success) {
            client.info("signed in", signin_result.success);

            for (y = 0; y < 1; y++) {
                for (let i = 0; i < 11; i++) {
                    const query_result = client.query({ collectionname: 'entities', query: '{}', projection: '{"name":1}', orderby: '{}', queryas: '', explain: false, skip: 0, top: 0 });
                    console.log("Got", query_result.length, "results");
                }
            }  
            let promises = [];
            for(y = 0; y < 1; y++) {
                for(let i = 0; i < 15; i++) {
                    promises.push(client.query_async({ collectionname: 'entities', query: '{}', projection: '{"name":1}', orderby: '{}', queryas: '', explain: false, skip: 0, top: 0 }));
                }
                client.info(
                    (await Promise.all(promises)).map(result => result.length)
                );
                promises = [];
            }

            const aggregate_async_result = await client.aggregate_async({ collectionname: 'entities', aggregates: '[]', explain: false });
            console.log("aggregate_async success", aggregate_async_result.length, " results");
            const aggregate_result = client.aggregate({ collectionname: 'entities', aggregates: '[]', explain: false });
            console.log("aggregate success", aggregate_result.length, " results");

            const insert_many_async_result = await client.insert_many_async({ collectionname: 'entities', documents: '[{"name":"test from nodejs", "_type": "test"}, {"name":"test from nodejs", "_type": "test"}]' });
            console.log("insert_many_async_result success", insert_many_async_result);
            const insert_many_result = client.insert_many({ collectionname: 'entities', documents: '[{"name":"test from nodejs", "_type": "test"}, {"name":"test from nodejs", "_type": "test"}]' });
            console.log("insert_many success", insert_many_result);

            const download_async_result = await client.download_async({ collectionname: 'fs.files', id: '65a3aaf66d52b8c15131aebd', folder: '', filename: '' });
            console.log("download async success", download_async_result);
            const download_result = client.download({ collectionname: 'fs.files', id: '65a3aaf66d52b8c15131aebd', folder: '', filename: '' });
            console.log("download success", download_result);


            const upload_async_result = await client.upload_async({ filepath, filename: 'node-async-test.csv', mimetype: '', metadata: '', collectionname: 'fs.files' });
            console.log("upload async success", upload_async_result);
            const upload_result = client.upload({ filepath, filename: 'node-test.csv', mimetype: '', metadata: '', collectionname: 'fs.files' });
            console.log("upload success", upload_result);


            var count_async_result = await client.count_async({ collectionname: 'entities', query: '{}', queryas: '' });
            console.log("count success", count_async_result);
            var count_result = client.count({ collectionname: 'entities', query: '{}', queryas: '' });
            console.log("count success", count_result);

            var distinct_async_result = await client.distinct_async({ collectionname: 'entities', field: '_type' });
            console.log("distinct async success", distinct_async_result);
            var distinct_result = client.distinct({ collectionname: 'entities', field: '_type' });
            console.log("distinct success", distinct_result);


            // // let eventcount = 0;
            // // // const watch_result2 = await client.watch_async({ collectionname: 'entities', paths: '' }, (event) => {
            // // //     event.document = JSON.parse(event.document);
            // // //     console.log("watch " + event.operation + " for " + event.document._type + " / " + event.document.test);
            // // //     eventcount++;
            // // // });
            // // // console.log("watch created as", watch_result2);
            // // const watch_result = client.watch({ collectionname: 'entities', paths: '' }, (event) => {
            // //     console.log("watch " + event.operation + " for " + event.document._type + " / " + event.document.test);
            // //     eventcount++;
            // // });
            // // console.log("watch created as", watch_result);


            // // while (eventcount < 1) {
            // //     await new Promise(resolve => setTimeout(resolve, 1000));
            // // }

            // // console.log("UNWATCH", watch_result)
            // // client.unwatch(watch_result);

            // // let queuecount = 0;
            // // const queuename = client.register_queue({ queuename: 'testq' }, (event) => {
            // //     console.log("queue event " + event.queuename + " from " + event.replyto + " / " + event.data);
            // //     queuecount++;
            // // });
            // // console.log("queue registered with name", queuename);
            // // while (queuecount < 2) {
            // //     await new Promise(resolve => setTimeout(resolve, 1000));
            // // }
            // // client.unregister_queue(queuename);
            // // console.log("Un registered queue", queuename);

            // // let exchangecount = 0;
            // // const exchangename = client.register_exchange({ exchangename: 'testexc' }, (event) => {
            // //     console.log("exchange event " + event.exchangename + " from " + event.replyto + " / " + event.data);
            // //     exchangecount++;
            // // });
            // // console.log("exchange registered with name", exchangename);
            // // while (exchangecount < 2) {
            // //     await new Promise(resolve => setTimeout(resolve, 1000));
            // // }

            const insert_one_async_result = await client.insert_one_async({ collectionname: 'entities', document: '{"name":"test from nodejs", "_type": "test"}' });
            console.log("insert_one async success", insert_one_async_result._id);
            const insert_one_result = client.insert_one({ collectionname: 'entities', document: '{"name":"test from nodejs", "_type": "test"}' });
            console.log("insert_one success", insert_one_result._id);


            const update_one_result = client.update_one({ collectionname: 'entities', item: `{"name":"test update from nodejs", "_id": "${insert_one_result._id}"}` });
            console.log("update_one success", update_one_result);
            const update_one_async_result = await client.update_one_async({ collectionname: 'entities', item: `{"name":"test update async from nodejs", "_id": "${insert_one_async_result._id}"}` });
            console.log("update_one async success", update_one_async_result);

            const delete_one_result = client.delete_one({ collectionname: 'entities', id: insert_one_result._id });
            console.log("delete_one success", delete_one_result == 1, delete_one_result);
            const delete_one_async_result = await client.delete_one_async({ collectionname: 'entities', id: update_one_async_result._id });
            console.log("delete_one async success", delete_one_async_result == 1, delete_one_async_result);

            const insert_or_update_one_async_result = await client.insert_or_update_one_async({ collectionname: 'entities', item: `{"name":"test insert_or_update one async from nodejs", "age": 21 }`, uniqeness: "name" } );
            console.log("insert_or_update_one success", insert_or_update_one_async_result._id, insert_or_update_one_async_result.age);
            const insert_or_update_one_async_result2 = await client.insert_or_update_one_async({ collectionname: 'entities', item: `{"name":"test insert_or_update one async from nodejs", "age": 22 }`, uniqeness: "name" });
            console.log("insert_or_update_one success2", insert_or_update_one_async_result2._id, insert_or_update_one_async_result2.age);
            const insert_or_update_one_result = client.insert_or_update_one({ collectionname: 'entities', item: `{"name":"test insert_or_update one from nodejs", "age": 21 }`, uniqeness: "name" } );
            console.log("insert_or_update_one success", insert_or_update_one_result._id, insert_or_update_one_result.age);
            const insert_or_update_one_result2 = client.insert_or_update_one({ collectionname: 'entities', item: `{"name":"test insert_or_update one from nodejs", "age": 22 }`, uniqeness: "name" });
            console.log("insert_or_update_one success2", insert_or_update_one_result2._id, insert_or_update_one_result2.age);

            const delete_many_query_result = client.delete_many({ collectionname: 'entities', query: '{"name":"test insert_or_update one from nodejs"}' });
            console.log("delete_many success", delete_many_query_result);
            const delete_many_ids_result = client.delete_many({ collectionname: 'entities', ids: [insert_or_update_one_async_result._id, insert_or_update_one_result2._id] });
            console.log("delete_many success", delete_many_ids_result);


            let nextrun = new Date();
            nextrun.setSeconds(nextrun.getSeconds() + 60);
            nextrun = undefined;
            let files1 = [];
            files1.push(filepath);
            let push_workitem_result = client.push_workitem({ wiq: "rustqueue", name: "node test", nextrun, files: files1});
            console.log("push_workitem success", push_workitem_result);

            // for(let i = 0; i < 10; i++) {
            //     let files2 = [];
            //     files2.push(filepath);
            //     let push_workitem_async_result = await client.push_workitem_async({ wiq: "rustqueue", name: "node test", nextrun, files: files2});
            //     console.log("push_workitem async success", push_workitem_async_result);
            // }


            let pop_workitem_result = client.pop_workitem({ wiq: "rustqueue" });
            console.log("pop_workitem success", pop_workitem_result);

            // let workitem = client.pop_workitem({ wiq: "rustqueue" });
            // do {
            //     console.log("pop_workitem success", workitem);
            //     workitem = client.pop_workitem({ wiq: "rustqueue" });
            // } while (workitem != null);


            client.info("*********************************")
            client.info("done, free client");
            client.info("*********************************")
            client.free();
        } else {
            client.info("signed failed", signin_result.error);
        }
    } catch (error) {
        if (error instanceof ClientError) {
            console.error(`An error occurred: ${error.message}`);
        } else {
            console.error(`An unexpected error occurred: ${error}`);
        }
    } finally {
        clearInterval(_int);
    }
})();
