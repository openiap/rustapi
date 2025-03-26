const { config } = require('koffi');
const { Client, ClientError } = require('./lib');
const fs = require('fs');
(async () => {
    let _int = setInterval(() => { 
        // console.log("Event loop is running"); 
    }, 200);
    const client = new Client();
    try {
        const test_async = true;
        const test_sync = true;
        const test_watch = true;
        const test_multiple_workitems = true;
        const test_message_queue = true;
        const test_exchange = true;
        const test_off_client_event = false;

        const url = '';
        // client.enable_tracing("openiap=trace", "new");
        // client.enable_tracing("openiap=debug", "new");
        client.enable_tracing("openiap=info", "");



        let filepath = 'testfile.csv';
        if (!fs.existsSync(filepath)) {
            filepath = '../testfile.csv';
        }

        if(test_sync) {
            client.connect(url);
        } else if (test_async) {
            await client.connect_async(url);
        } else {
            client.connect(url);
        }
        let eventid = client.on_client_event((event) => {
            console.log("client event", event);
        });


        client.info("connect completed, now call signin() again")
        if(test_async) {
            const signin_result2 = await client.signin_async();
            if(signin_result2.success) {
                client.info("async signed in", signin_result2.success);
            } else {
                client.info("async signed failed", signin_result2.error);
            }
        }
        const signin_result = client.signin();
        client.info("signin() complete")
        // client.info(signin_result);
        if (signin_result.success) {
            client.info("signed in", signin_result.success);

            if(test_sync) {
                let clients = client.custom_command({ command: 'getclients' });
                console.log("clients", clients);
            }

            if(test_async) {
                let clients = await client.custom_command_async({ command: 'getclients' });
                console.log("clients", clients);
            }

            if(test_sync) {
                let collections = client.list_collections();
                // client.info("collections", collections);
                let nodejs_testcol_exists = false;
                let nodejs_testtimeseriescol_exists = false;
                for(let i = 0; i < collections.length; i++) {
                    let col = collections[i];
                    client.info(col.type, col.name);
                    if (col.name == 'nodejs_testcol') {
                        nodejs_testcol_exists = true;
                    }
                    if (col.name == 'nodejs_testtimeseriescol') {
                        nodejs_testtimeseriescol_exists = true;
                    }
                }
                if (nodejs_testcol_exists == false) {
                    client.create_collection({ collectionname: 'nodejs_testcol' });
                }
                client.insert_one({ collectionname: 'nodejs_testcol', item: '{"name":"test watch from nodejs", "_type": "test"}' });
                if (nodejs_testtimeseriescol_exists == false) {
                    client.create_collection({ collectionname: 'nodejs_testtimeseriescol', timeseries: { time_field: 'time', meta_field: 'value', granularity: 'minutes' } });
                }
                client.insert_one({ collectionname: 'nodejs_testtimeseriescol', item: '{"time":"2024-10-13T08:52:41.430Z", "value": 1}' });
   
            }

            if(test_async) {
                let collections = await client.list_collections_async();
                // client.info("collections", collections);
                let nodejsa_testcol_exists = false;
                let nodejsa_testtimeseriescol_exists = false;
                for(let i = 0; i < collections.length; i++) {
                    let col = collections[i];
                    client.info(col.type, col.name);
                    if (col.name == 'nodejsa_testcol') {
                        nodejsa_testcol_exists = true;
                    }
                    if (col.name == 'nodejsa_testtimeseriescol') {
                        nodejsa_testtimeseriescol_exists = true;
                    }
                }
                if (nodejsa_testcol_exists == false) {
                    await client.create_collection_async({ collectionname: 'nodejsa_testcol' });
                }
                await client.insert_one_async ({ collectionname: 'nodejsa_testcol', item: '{"name":"test watch from nodejs", "_type": "test"}' });
                if (nodejsa_testtimeseriescol_exists == false) {
                    await client.create_collection_async({ collectionname: 'nodejsa_testtimeseriescol', timeseries: { time_field: 'time', meta_field: 'value', granularity: 'minutes' } });
                }
                await client.insert_one_async({ collectionname: 'nodejsa_testtimeseriescol', item: '{"time":"2024-10-13T08:52:41.430Z", "value": 1}' });

            } 

            if (test_sync) {
                let indexes = client.get_indexes('nodejs_testcol');
                let name_1_exists = false;
                for(let i = 0; i < indexes.length; i++) {
                    client.info("index", indexes[i].name);
                    if (indexes[i].name == 'name_1') {
                        name_1_exists = true;
                    }
                }

                if (name_1_exists == false) {
                    client.create_index({ collectionname: 'nodejs_testcol', index: '{"name":1}', unique: true });
                    indexes = client.get_indexes('nodejs_testcol');
                    for(let i = 0; i < indexes.length; i++) {
                        client.info("index", indexes[i].name);
                    }
                }

                client.drop_index('nodejs_testcol', 'name_1');
                indexes = client.get_indexes('nodejs_testcol');
                for(let i = 0; i < indexes.length; i++) {
                    client.info("index", indexes[i].name);
                }

                client.info("Clean up");
                client.drop_collection('nodejs_testcol');
                client.drop_collection('nodejs_testtimeseriescol');
            }


            if(test_async) {
                let indexes = await client.get_indexes_async('nodejsa_testcol');
                let name_1_exists = false;
                for(let i = 0; i < indexes.length; i++) {
                    client.info("index", indexes[i].name);
                    if (indexes[i].name == 'name_1') {
                        name_1_exists = true;
                    }
                }

                if (name_1_exists == false) {
                    await client.create_index_async({ collectionname: 'nodejsa_testcol', index: '{"name":1}', unique: true });
                    indexes = client.get_indexes('nodejsa_testcol');
                    for(let i = 0; i < indexes.length; i++) {
                        client.info("index", indexes[i].name);
                    }
                }

                await client.drop_index_async('nodejsa_testcol', 'name_1');
                indexes = await client.get_indexes_async('nodejsa_testcol');
                for(let i = 0; i < indexes.length; i++) {
                    client.info("index", indexes[i].name);
                }


                client.info("Clean up");
                await client.drop_collection_async('nodejsa_testcol');
                await client.drop_collection_async('nodejsa_testtimeseriescol');

            }


            if(test_off_client_event) {
                client.off_client_event(eventid);
            }

            if(test_sync) {
                for (y = 0; y < 1; y++) {
                    for (let i = 0; i < 15; i++) {
                        const query_result = client.query({ collectionname: 'entities', query: '{}', projection: '{"name":1}', orderby: '{}', queryas: '', explain: false, skip: 0, top: 0 });
                        console.log("query.sync.Got", query_result.length, "results");
                    }
                }
            }
            if(test_async) {
                let promises = [];
                for(y = 0; y < 1; y++) {
                    for(let i = 0; i < 15; i++) {
                        promises.push(client.query_async({ collectionname: 'entities', query: '{}', projection: '{"name":1}', orderby: '{}', queryas: '', explain: false, skip: 0, top: 0 }));
                    }
                    console.log("query_async, wait for reply");
                    client.info(
                        (await Promise.all(promises)).map(result => result.length)
                    );
                    promises = [];
                }
            }

            if(test_async) {
                const aggregate_async_result = await client.aggregate_async({ collectionname: 'entities', aggregates: '[]', explain: false });
                console.log("aggregate_async success", aggregate_async_result.length, " results");

            }
            if(test_sync) {
                const aggregate_result = client.aggregate({ collectionname: 'entities', aggregates: '[]', explain: false });
                console.log("aggregate success", aggregate_result.length, " results");
            }

            if(test_async) {
                const download_async_result = await client.download_async({ collectionname: 'fs.files', id: '65a3aaf66d52b8c15131aebd', folder: '', filename: '' });
                console.log("download async success", download_async_result);
    
                const upload_async_result = await client.upload_async({ filepath, filename: 'node-async-test.csv', mimetype: '', metadata: '', collectionname: 'fs.files' });
                console.log("upload async success", upload_async_result);
            }
            if(test_sync) {
                const download_result = client.download({ collectionname: 'fs.files', id: '65a3aaf66d52b8c15131aebd', folder: '', filename: '' });
                console.log("download success", download_result);

                const upload_result = client.upload({ filepath, filename: 'node-test.csv', mimetype: '', metadata: '', collectionname: 'fs.files' });
                console.log("upload success", upload_result);
            }

            if(test_async) {
                var count_async_result = await client.count_async({ collectionname: 'entities', query: '{}', queryas: '' });
                console.log("count success", count_async_result);
    
                var distinct_async_result = await client.distinct_async({ collectionname: 'entities', field: '_type' });
                console.log("distinct async success", distinct_async_result);
            }

            if(test_sync) {
                var count_result = client.count({ collectionname: 'entities', query: '{}', queryas: '' });
                console.log("count success", count_result);
    
                var distinct_result = client.distinct({ collectionname: 'entities', field: '_type' });
                console.log("distinct success", distinct_result);
            }


            if(test_watch) {
                let eventcount = 0;
                const watch_result = client.watch({ collectionname: 'entities', paths: '' }, (event, count) => {
                    console.log("watch " + event.operation + " #", count, " for " + event.document._type + " / " + event.document.test);
                    eventcount++;
                });
                console.log("watch created as", watch_result);
    
                await new Promise(resolve => setTimeout(resolve, 2000));
                client.insert_one({ collectionname: 'entities', item: '{"name":"test watch from nodejs", "_type": "test"}' });
    
    
                while (eventcount < 1) {
                    await new Promise(resolve => setTimeout(resolve, 1000));
                }
    
                client.unwatch(watch_result);
            }

            if(test_async) {
                const insert_one_async_result = await client.insert_one_async({ collectionname: 'entities', item: '{"name":"test from nodejs", "_type": "test"}' });
                console.log("insert_one async success", insert_one_async_result._id);

                const update_one_async_result = await client.update_one_async({ collectionname: 'entities', item: `{"name":"test update async from nodejs", "_id": "${insert_one_async_result._id}"}` });
                console.log("update_one async success", update_one_async_result);

                const delete_one_async_result = await client.delete_one_async({ collectionname: 'entities', id: update_one_async_result._id });
                console.log("delete_one async success", delete_one_async_result == 1, delete_one_async_result);
            }

            if(test_sync) {
                const insert_one_result = client.insert_one({ collectionname: 'entities', item: '{"name":"test from nodejs", "_type": "test"}' });
                console.log("insert_one success", insert_one_result._id);
    
                const update_one_result = client.update_one({ collectionname: 'entities', item: `{"name":"test update from nodejs", "_id": "${insert_one_result._id}"}` });
                console.log("update_one success", update_one_result);
    
                const delete_one_result = client.delete_one({ collectionname: 'entities', id: insert_one_result._id });
                console.log("delete_one success", delete_one_result == 1, delete_one_result);
            }

            if(test_async) {
                const insert_or_update_one_async_result = await client.insert_or_update_one_async({ collectionname: 'entities', item: `{"name":"test insert_or_update one async from nodejs", "age": 21 }`, uniqeness: "name" } );
                console.log("insert_or_update_one success", insert_or_update_one_async_result._id, insert_or_update_one_async_result.age);
                const insert_or_update_one_async_result2 = await client.insert_or_update_one_async({ collectionname: 'entities', item: `{"name":"test insert_or_update one async from nodejs", "age": 22 }`, uniqeness: "name" });
                console.log("insert_or_update_one success2", insert_or_update_one_async_result2._id, insert_or_update_one_async_result2.age);

                const delete_many_ids_async_result = await client.delete_many_async({ collectionname: 'entities', ids: [insert_or_update_one_async_result._id, insert_or_update_one_async_result2._id] });
                console.log("delete_many success", delete_many_ids_async_result);

                const insert_many_async_result = await client.insert_many_async({ collectionname: 'entities', items: '[{"name":"test insert many async from nodejs", "_type": "test"}, {"name":"test insert many async from nodejs", "_type": "test"}]' });
                console.log("insert_many_async_result success", insert_many_async_result.map(result => result._id));

                const delete_many_query_async_result = await client.delete_many_async({ collectionname: 'entities', query: '{"name":"test insert many async from nodejs"}' });
                console.log("delete_many success", delete_many_query_async_result);
            }

            if(test_sync) {
                const insert_or_update_one_result = client.insert_or_update_one({ collectionname: 'entities', item: `{"name":"test insert_or_update one from nodejs", "age": 21 }`, uniqeness: "name" } );
                console.log("insert_or_update_one success", insert_or_update_one_result._id, insert_or_update_one_result.age);
                const insert_or_update_one_result2 = client.insert_or_update_one({ collectionname: 'entities', item: `{"name":"test insert_or_update one from nodejs", "age": 22 }`, uniqeness: "name" });
                console.log("insert_or_update_one success2", insert_or_update_one_result2._id, insert_or_update_one_result2.age);
    
                const delete_many_ids_result = client.delete_many({ collectionname: 'entities', ids: [insert_or_update_one_result._id, insert_or_update_one_result2._id] });
                console.log("delete_many success", delete_many_ids_result);

                const insert_many_result = client.insert_many({ collectionname: 'entities', items: '[{"name":"test insert many from nodejs", "_type": "test"}, {"name":"test insert many from nodejs", "_type": "test"}]' });
                console.log("insert_many success", insert_many_result.map(result => result._id));

                const delete_many_query_result = client.delete_many({ collectionname: 'entities', query: '{"name":"test insert many from nodejs"}' });
                console.log("delete_many success", delete_many_query_result);
            }

            let nextrun = new Date();
            nextrun.setSeconds(nextrun.getSeconds() + 60);
            nextrun = undefined;
            let files1 = [];
            files1.push(filepath);
            if(test_async) {
                let push_workitem_async_result = await client.push_workitem_async({ wiq: "rustqueue", name: "node test async", nextrun, files: files1});
                console.log("push_workitem_async success", push_workitem_async_result);

                let pop_workitem_async_result = await client.pop_workitem_async({ wiq: "rustqueue" });
                console.log("pop_workitem_async success", pop_workitem_async_result);

                console.log("set pop_workitem_async_result.state = successful");
                pop_workitem_async_result.state = "successful";
                const update_workitem_async_result = await client.update_workitem_async({ workitem: pop_workitem_async_result });
                console.log("update_workitem_async success", update_workitem_async_result);

                await client.delete_workitem_async(update_workitem_async_result.id);
    
                if(test_multiple_workitems) {
                    let promises = [];
                    for(let i = 0; i < 10; i++) {
                        let files2 = [];
                        files2.push(filepath);
                        promises.push(client.push_workitem_async({ wiq: "rustqueue", name: "node test", nextrun, files: files2}));
                    }
                    console.log("push_workitem_async (multiple) success", (await Promise.all(promises)).map(result => result.id));

                    let workitem = await client.pop_workitem_async({ wiq: "rustqueue" });
                    do {
                        console.log("pop_workitem async (multiple) success", workitem);
                        workitem.state = "successful";
                        await client.update_workitem_async({ workitem });
                        console.log("update_workitem async (multiple) success");
                        workitem = await client.pop_workitem_async({ wiq: "rustqueue" });
                    } while (workitem != null);
                }
            }


            if(test_sync) {
                let push_workitem_result = client.push_workitem({ wiq: "rustqueue", name: "node test", nextrun, files: files1});
                console.log("push_workitem success", push_workitem_result);

                console.log("now pop the workitem");

                let pop_workitem_result = client.pop_workitem({ wiq: "rustqueue" });
                console.log("pop_workitem success", pop_workitem_result);

                console.log("set pop_workitem_result.state = successful");
                pop_workitem_result.state = "successful";
                const update_workitem_result = client.update_workitem({ workitem: pop_workitem_result});
                console.log("update_workitem success", update_workitem_result);

                client.delete_workitem(update_workitem_result.id);

                if(test_multiple_workitems) {
                    for(let i = 0; i < 10; i++) {
                        let files2 = [];
                        files2.push(filepath);
                        let push_workitem_result = client.push_workitem({ wiq: "rustqueue", name: "node test", nextrun, files: files2});
                        console.log("push_workitem (multiple 2) success", push_workitem_result);
                    }
                    let workitem = client.pop_workitem({ wiq: "rustqueue" });
                    do {
                        console.log("pop_workitem success", workitem);

                        workitem.state = "successful";
                        client.update_workitem({ workitem });

                        workitem = client.pop_workitem({ wiq: "rustqueue" });
                    } while (workitem != null);
                }
            }

            if(test_message_queue) {
                let queuecount = 0;
                const queuename = client.register_queue({ queuename: 'testq' }, (event) => {
                    console.log("#", queuecount, "queue". event?.queuename, "event from", event?.replyto, ":", event?.data);
                    queuecount++;
                });
                console.log("queue registered with name", queuename);
                client.queue_message({ queuename, data: "{\"message\": \"test 1\"}" });
                client.queue_message({ queuename, data: "{\"message\": \"test 2\"}" });
                do {
                    await new Promise(resolve => setTimeout(resolve, 1000));
                    // console.log("#", queuecount, "queue event loop");
                } while (queuecount < 2);
                console.log("queue event loop done");
                client.unregister_queue(queuename);
                console.log("Un registered queue", queuename);
            }
            if(test_exchange) {
                let exchangecount = 0;
                const queuename = client.register_exchange({ exchangename: 'testexc' }, (event) => {
                    console.log("exchange event " + event.exchangename + " from " + event.replyto + " / " + event.data);
                    exchangecount++;
                });
                console.log("exchange registered with queue name", queuename);

                client.queue_message({ exchangename: 'testexc', data: "{\"message\": \"test 1\"}" });
                client.queue_message({ exchangename: 'testexc', data: "{\"message\": \"test 2\"}" });

                while (exchangecount < 2) {
                    await new Promise(resolve => setTimeout(resolve, 1000));
                }
                console.log("exchange event loop done");
                client.unregister_queue(queuename);
            }
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
        if(client) {
            client.info("*********************************")
            client.info("done, free client");
            client.info("*********************************")
            client.free();
        }
    }
})();
