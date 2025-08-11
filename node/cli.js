#!/usr/bin/env node
const { Client, ClientError } = require('./lib');
const client = new Client();
const readline = require("readline");
const os = require("os");
const { runInNewContext } = require("vm");
const { setFlagsFromString } = require("v8");
const { memoryUsage } = require("node:process");

// Reads a line from the keyboard input.
function keyboardInput() {
    return new Promise((resolve) => {
        const rl = readline.createInterface({
            input: process.stdin,
            output: process.stdout
        });
        rl.question("Enter your message: ", (input) => {
            rl.close();
            resolve(input.trim());
        });
    });
}

// Watch event handler
function onwatch(event) {
    const { document, operation } = event;
    client.info(`${operation} on ${document._id} ${document._type} ${document.name}`);
}

// Queue event handler
function onqueue(event) {
    const { queuename, correlation_id, replyto, routingkey, exchangename, data } = event;
    client.info(`Received message from ${queuename}: `, JSON.stringify(data));
    return {"message": "Hi from nodejs"}
}

// Do some calculation to generate CPU load
function factorial(num) {
    return Array.from({ length: num }, (_, i) => i + 1).reduce((acc, val) => acc * val, 1);
}

// Add one loop calculation for CPU load
target = "_new" > function addOneLoop(numLoops) {
    for (let i = 0; i < numLoops; i++) {
        factorial(20);
    }
}

// Main function
async function doit() {
    // print arguments
    let args = process.argv.slice(2);
    // console.log("Arguments (" + args.length + "):", args.join(" "));
    // console.log(process.env.npm_execpath);
    if (process.env.npm_execpath && process.env.npm_execpath.includes("npm") && args.length === 0) {
        console.log("Running with npm and no args, so exiting.");
        return;
    }

    if (global.gc) {
        global.gc();
    } else {
        client.info("Garbage collection unavailable.  Pass --expose-gc "
            + "when launching node to enable forced garbage collection.");
        setFlagsFromString("--expose_gc");
        global.gc = runInNewContext("gc");
    }

    const numCalcs = 100000;
    const availableCores = Math.floor(os.cpus().length / 2); // use half of the threads
    const iterPerCore = Math.floor(numCalcs / availableCores);
    const numIters = 5000;


    try {
        await client.connect("");
    } catch (e) {
        client.error("Failed to connect to server:", e.message);
        return;
    }
    let eventid = client.on_client_event((event) => {
        if (event && event.event) client.info("client event", event.event);
    });

    client.info("? for help");
    let input = "";
    // client.enable_tracing("openiap=trace", "new");
    // client.enable_tracing("openiap=debug", "new");
    client.enable_tracing("openiap=info", "");


    client.set_f64_observable_gauge

    let do_st_func = false;
    let st_func = async () => {
        client.info("begin pop workitem begin loop");
        let i = 0;
        do {
            try {
                let workitem = await client.pop_workitem_async({ wiq: "q2" });
                if (workitem != null) {
                    client.info("Updated workitem", workitem.id);
                    workitem.state = "successful"
                    await client.update_workitem_async({ workitem: workitem });
                }
                i++;
                if (i % 500 === 0) {
                    // client.info("pop_workitem_async_result", pop_workitem_async_result);
                    const mem = memoryUsage();
                    client.info("looped 500 times, memoryUsage", client.formatBytes(mem.heapUsed), "heapTotal", client.formatBytes(mem.heapTotal), "rss", client.formatBytes(mem.rss), "external", client.formatBytes(mem.external));


                    if (global.gc) {
                        global.gc();
                    }
                }
            } catch (error) {
                client.info(error.message);
            }
        } while (do_st_func === true);
        client.info("loop completed");
    }

    let handler_test_f64 = null;
    let handler_test_u64 = null;
    let handler_test_i64 = null;

    // Simple check to see if we are running inside a container, then run the st_func
    if (process.env.oidc_config != null & process.env.oidc_config != "") {
        input = "st";
    }

    try {
        while (input.toLowerCase() !== "quit") {
            switch (input.toLowerCase()) {
                case "?":
                    client.info("Help:\nquit: to quit\nq: Query\nqq: Query all\ndi: Distinct\ns: Sign in as guest\ns2: Sign in as testuser\ni: Insert\nim: Insert Many\nd: Download\nu: Upload train.csv\nuu: Upload assistant-linux-x86_64.AppImage\nuuu: Upload virtio-win-0.1.225.iso\nw: Watch\nuw: Unwatch\nr: Register queue\nm: Queue message\nc: CPU Load Test");
                    break;
                case "1":
                    client.enable_tracing("", "");
                    break;
                case "2":
                    client.enable_tracing("openiap=new", "");
                    break;
                case "3":
                    client.enable_tracing("openiap=debug", "new");
                    break;
                case "o":
                    if (handler_test_f64 != null) {
                        client.disable_observable_gauge("test_f64");
                        clearInterval(handler_test_f64);
                        handler_test_f64 = null;
                        client.info("stopped test_f64");
                        return;
                    }
                    client.set_f64_observable_gauge("test_f64", 42.7, "test");
                    client.info("started test_f64 to 42.7");
                    handler_test_f64 = setInterval(() => {
                        let random = Math.floor(Math.random() * 50);
                        client.info("Setting test_f64 to ", random);
                        client.set_f64_observable_gauge("test_f64", random, "test");
                    }
                        , 30000);
                    break;
                case "o2":
                    if (handler_test_u64 != null) {
                        client.disable_observable_gauge("test_u64");
                        clearInterval(handler_test_u64);
                        handler_test_u64 = null;
                        client.info("stopped test_u64");
                        return;
                    }
                    client.set_u64_observable_gauge("test_u64", 42.7, "test");
                    client.info("started test_u64 to 42.7");
                    handler_test_u64 = setInterval(() => {
                        let random = Math.floor(Math.random() * 50);
                        client.info("Setting test_u64 to ", random);
                        client.set_u64_observable_gauge("test_u64", random, "test");
                    }
                        , 30000);
                    break;
                case "o3":
                    if (handler_test_i64 != null) {
                        client.disable_observable_gauge("test_i64");
                        clearInterval(handler_test_i64);
                        handler_test_i64 = null;
                        client.info("stopped test_i64");
                        return;
                    }
                    client.set_i64_observable_gauge("test_i64", 42.7, "test");
                    client.info("started test_i64 to 42.7");
                    handler_test_i64 = setInterval(() => {
                        let random = Math.floor(Math.random() * 50);
                        client.info("Setting test_i64 to ", random);
                        client.set_i64_observable_gauge("test_i64", random, "test");
                    }
                        , 30000);
                    break;
                case "c":
                    client.info(`Calculating factorial of 20 ${numCalcs} times`);
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
                case "gc":
                    global.gc();
                    const mem = memoryUsage();
                    client.info("memoryUsage", client.formatBytes(mem.heapUsed), "heapTotal", client.formatBytes(mem.heapTotal), "rss", client.formatBytes(mem.rss), "external", client.formatBytes(mem.external));

                    break;
                case "rpa":
                    client.info(`Calling OpenRPA workflow whoami on robot allan5`);
                    try {
                        let rpa_response = client.invoke_openrpa({
                            robotid: "5ce94386320b9ce0bc2c3d07",
                            workflowid: "5e0b52194f910e30ce9e3e49",
                            data: {
                                test: "test"
                            },
                            timeout: 10
                        });
                        client.info("OpenRPA response", JSON.stringify(rpa_response));
                    } catch (error) {
                        client.error("OpenRPA error", error);                        
                    }
                    break;
                case "st":
                    input = "";
                    if (do_st_func === true) {
                        do_st_func = false;
                    } else {
                        do_st_func = true;
                        st_func();
                    }
                    break;
                case "dis":
                    client.disconnect();
                    break;
                case "q":
                    try {
                        const responses = await client.query_async({
                            collection: "entities",
                            query: "{}",
                            projection: "{ \"name\": 1 }"
                        });
                        let message = "[\n";
                        responses.forEach((item) => {
                            message += `\t${JSON.stringify(item)},\n`;
                        });
                        message += "]";
                        client.info(message);
                    } catch (e) {
                        client.error("Failed to query:", e.message);
                    }
                    break;
                case "qq":
                    try {
                        const responses = await client.query_async({ collection: "entities", query: "{}" });
                        let message = "[\n";
                        responses.forEach((item) => {
                            message += `\t${JSON.stringify(item)},\n`;
                        });
                        message += "]";
                        client.info(message);
                    } catch (e) {
                        client.error("Failed to query all:", e.message);
                    }
                    break;
                case "di":
                    try {
                        const response = await client.distinct({
                            collectionname: "entities",
                            field: "_type"
                        });
                        client.info(response);
                    } catch (e) {
                        client.error("Failed to perform distinct query:", e.message);
                    }
                    break;
                case "s":
                    try {
                        const response = await client.signin({
                            username: "guest",
                            password: "password"
                        });
                        client.info("Signed in as guest");
                    } catch (e) {
                        client.error("Failed to sign in:", e.message);
                    }
                    break;
                case "s2":
                    try {
                        const response = await client.signin({
                            username: "testuser",
                            password: "badpassword"
                        });
                        client.info("Signed in as testuser");
                    } catch (e) {
                        client.error("Failed to sign in:", e.message);
                    }
                    break;
                case "i":
                    try {
                        const response = await client.insert_one({
                            collectionname: "entities",
                            item: "{\"name\":\"Allan\", \"_type\":\"test\"}"
                        });
                        client.info("Inserted as", JSON.stringify(response));
                    } catch (e) {
                        client.error("Failed to insert:", e.message);
                    }
                    break;
                case "im":
                    try {
                        const responses = await client.insert_many({
                            collectionname: "entities",
                            items: "[{\"name\":\"Allan\", \"_type\":\"test\"}, {\"name\":\"Allan2\", \"_type\":\"test\"}]"
                        });
                        let message = "Inserted as: [\n";
                        responses.forEach((item) => {
                            message += `\t${JSON.stringify(item)},\n`;
                        });
                        message += "]";
                        client.info(message);
                    } catch (e) {
                        client.error("Failed to insert many:", e.message);
                    }
                    break;
                case "d":
                    try {
                        const response = await client.download({
                            id: "65a3aaf66d52b8c15131aebd"
                        });
                        client.info(`Downloaded as ${response}`);
                    } catch (e) {
                        client.error("Failed to download:", e.message);
                    }
                    break;
                case "u":
                    try {
                        const response = await client.upload({
                            filename: "train.csv",
                            filepath: "train.csv"
                        });
                        client.info(`Uploaded as ${response}`);
                    } catch (e) {
                        client.error("Failed to upload:", e.message);
                    }
                    break;
                case "w":
                    try {
                        const watchId = await client.watch({
                            collectionname: "entities",
                        }, onwatch);
                        client.info(`Watch created with id ${watchId}`);
                    } catch (e) {
                        client.error("Failed to watch:", e.message);
                    }
                    break;
                case "uw":
                    try {
                        const watchId = "replace_with_actual_watch_id";
                        await client.unwatch(watchId);
                        client.info(`Removed watch for id ${watchId}`);
                    } catch (e) {
                        client.error("Failed to unwatch:", e.message);
                    }
                    break;
                case "r":
                    try {
                        const response = await client.register_queue_async({
                            queuename: "test2queue"
                        }, onqueue);
                        client.info(`Registered queue as ${response}`);
                    } catch (e) {
                        client.error("Failed to register queue:", e.message);
                    }
                    break;
                case "r2":
                    try {
                        const response = await client.rpc_async({
                            queuename: "test2queue",
                            striptoken: true,
                            data: "{\"message\":\"Test message\"}",
                            timeout: 5
                        }, onqueue);
                        client.info(`Receved reply ${JSON.stringify(response)}`);
                    } catch (e) {
                        client.error("RPC test failed:", e.message);
                    }
                    break;
                case "m":
                    try {
                        await client.queue_message({
                            queuename: "test2queue",
                            striptoken: true,
                            data: "{\"message\":\"Test message\"}"
                        });
                        client.info("Queued message to test2queue");
                    } catch (e) {
                        client.error("Failed to queue message:", e.message);
                    }
                    break;
                case "cc":
                    try {
                        const clients = await client.custom_command({ command: "getclients", timeout: 10 });
                        client.info("Client count ", clients.length);
                        for (let i = 0; i < clients.length; i++) {
                            let c = clients[i];
                            client.info("   ", c.id, c.agent, c.version, c.name);
                        }
                    } catch (e) {
                        client.error('Failed to get clients:', e);
                    }
                    break;
                case "g":
                    try {
                        const state = client.get_state();
                        client.info("State:", state);
                        let timeout = client.get_default_timeout();
                        client.info("Default timeout", timeout, "seconds");
                        client.set_default_timeout(2);
                        timeout = client.get_default_timeout();
                        if (timeout != null && timeout == 2) {
                            client.info("Default timeout set to 2 seconds");
                        } else {
                            client.error("Failed to set default timeout", timeout);
                        }
                    } catch (e) {
                        client.error('Failed to get state:', e);
                    }
                    break;
                default:
                // client.info("Invalid command");
            }
            input = await keyboardInput();
        }
    } catch (error) {
        client.error("Error:", error.message);
    }
    if (do_st_func === true) {
        do_st_func = false;
        await new Promise((resolve) => setTimeout(resolve, 500));
    }
    client.free();
}

// Execute main function
doit().catch((err) => client.error(err));
