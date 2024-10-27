const readline = require('readline');
const { Client } = require('./lib');
const os = require('os');

// Reads a line from the keyboard input.
function keyboardInput() {
    return new Promise((resolve) => {
        const rl = readline.createInterface({
            input: process.stdin,
            output: process.stdout
        });
        rl.question('Enter your message: ', (input) => {
            rl.close();
            resolve(input.trim());
        });
    });
}

// Watch event handler
function onwatch(event) {
    const { document, operation } = event;
    console.log(`${operation} on ${document._id} ${document._type} ${document.name}`);
}

// Queue event handler
function onqueue(event) {
    const { queuename, correlation_id, replyto, routingkey, exchangename, data } = event;
    console.log(`Received message from ${queuename}: `, data);
}

// Do some calculation to generate CPU load
function factorial(num) {
    return Array.from({ length: num }, (_, i) => i + 1).reduce((acc, val) => acc * val, 1);
}

// Add one loop calculation for CPU load
target="_new">function addOneLoop(numLoops) {
    for (let i = 0; i < numLoops; i++) {
        factorial(20);
    }
}

// Main function
async function doit() {
    // Display system information

    const numCalcs = 100000;
    const availableCores = Math.floor(os.cpus().length / 2); // use half of the threads
    const iterPerCore = Math.floor(numCalcs / availableCores);
    const numIters = 5000;

    const client = new Client();

    try {
        await client.connect('');
    } catch (e) {
        console.error('Failed to connect to server:', e);
        return;
    }
    let eventid = client.on_client_event((event) => {
        console.log("client event", event);
    });

    console.log('? for help');
    let input = await keyboardInput();
    // client.enable_tracing("openiap=trace", "new");
    client.enable_tracing("openiap=debug", "new");
    // client.enable_tracing("openiap=info", "");

    try {
        while (input.toLowerCase() !== 'quit') {
            switch (input.toLowerCase()) {
                case '?':
                    console.log('Help:\nquit: to quit\nq: Query\nqq: Query all\ndi: Distinct\ns: Sign in as guest\ns2: Sign in as testuser\ni: Insert\nim: Insert Many\nd: Download\nu: Upload train.csv\nuu: Upload assistant-linux-x86_64.AppImage\nuuu: Upload virtio-win-0.1.225.iso\nw: Watch\nuw: Unwatch\nr: Register queue\nm: Queue message\nc: CPU Load Test');
                    break;
    
                case 'c':
                    console.log(`Calculating factorial of 20 ${numCalcs} times`);
                    for (let i = 0; i < numIters; i++) {
                        const threads = [];
                        for (let j = 0; j < availableCores; j++) {
                            threads.push(
                                new Promise((resolve) => {
                                    addOneLoop(iterPerCore);
                                    resolve();
                                })
                            );
                        }
                        await Promise.all(threads);
                    }
                    break;
                case 'dis':
                    client.disconnect();
                    break;
                case 'q':
                    try {
                        const response = await client.query_async({
                            collection: 'entities',
                            query: '{}',
                            projection: '{ "name": 1 }'
                        });
                        console.log(response);
                    } catch (e) {
                        console.error('Failed to query:', e);
                    }
                    break;
    
                case 'qq':
                    try {
                        const response = await client.query_async({ collection: 'entities', query: '{}' });
                        console.log(response);
                    } catch (e) {
                        console.error('Failed to query all:', e);
                    }
                    break;
                
                case 'di':
                    try {
                        const response = await client.distinct({
                            collectionname: 'entities',
                            field: '_type'
                        });
                        console.log(response);
                    } catch (e) {
                        console.error('Failed to perform distinct query:', e);
                    }
                    break;
    
                case 's':
                    try {
                        const response = await client.signin({
                            username: 'guest',
                            password: 'password'
                        });
                        console.log(`Signed in as guest`);
                    } catch (e) {
                        console.error('Failed to sign in:', e);
                    }
                    break;
    
                case 's2':
                    try {
                        const response = await client.signin({
                            username: 'testuser',
                            password: 'badpassword'
                        });
                        console.log(`Signed in as testuser`);
                    } catch (e) {
                        console.error('Failed to sign in:', e);
                    }
                    break;
    
                case 'i':
                    try {
                        const response = await client.insert_one({
                            collectionname: 'entities',
                            item: '{"name":"Allan", "_type":"test"}'
                        });
                        console.log(`Inserted as`, response);
                    } catch (e) {
                        console.error('Failed to insert:', e);
                    }
                    break;
    
                case 'im':
                    try {
                        const responses = await client.insert_many({
                            collectionname: 'entities',
                            items: '[{"name":"Allan", "_type":"test"}, {"name":"Allan2", "_type":"test"}]'
                        });
                        console.log(`Inserted as`, responses);
                    } catch (e) {
                        console.error('Failed to insert many:', e);
                    }
                    break;
                
                case 'd':
                    try {
                        const response = await client.download({
                            id: '65a3aaf66d52b8c15131aebd'
                        });
                        console.log(`Downloaded as ${response}`);
                    } catch (e) {
                        console.error('Failed to download:', e);
                    }
                    break;
                
                case 'u':
                    try {
                        const response = await client.upload({
                            filename: 'train.csv',
                            filepath: 'train.csv'
                        });
                        console.log(`Uploaded as ${response}`);
                    } catch (e) {
                        console.error('Failed to upload:', e);
                    }
                    break;
                
                case 'w':
                    try {
                        const watchId = await client.watch({
                            collectionname: 'entities',
                        }, onwatch);
                        console.log(`Watch created with id ${watchId}`);
                    } catch (e) {
                        console.error('Failed to watch:', e);
                    }
                    break;
                
                case 'uw':
                    try {
                        const watchId = 'replace_with_actual_watch_id';
                        const response = await client.unwatch(watchId);
                        console.log(`Removed watch for id ${watchId}`);
                    } catch (e) {
                        console.error('Failed to unwatch:', e);
                    }
                    break;
                
                case 'r':
                    try {
                        const response = await client.register_queue({
                            queuename: 'test2queue'
                        }, onqueue);
                        console.log(`Registered queue as ${response}`);
                    } catch (e) {
                        console.error('Failed to register queue:', e);
                    }
                    break;
                
                case 'm':
                    try {
                        await client.queue_message({
                            queuename: 'test2queue',
                            data: '{"message":"Test message"}'
                        });
                        console.log(`Queued message to test2queue`);
                    } catch (e) {
                        console.error('Failed to queue message:', e);
                    }
                    break;
    
                default:
                    console.log('Invalid command');
            }
            input = await keyboardInput();
        }
    } catch (error) {
        console.error('Error:', error);        
    }
    client.free();
}

// Execute main function
doit().catch((err) => console.error(err));
