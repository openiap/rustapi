import asyncio
from ctypes import c_char_p, ArgumentError
import json
from openiap import Client, ClientError
import os
import threading
import time
import random

from openiap.main import ColTimeseriesWrapper

# Function to read keyboard input asynchronously
async def keyboard_input():
    return await asyncio.get_event_loop().run_in_executor(None, input, "Enter your message: ")

# Watch event handler
def on_watch(event, counter):
    document = event.get("document")
    operation = event.get("operation")
    print(f"WATCH EVENT: {operation} on {document}")

# Queue event
def on_queue(event, counter):
    data = event.get("data")
    print(f"QUEUE EVENT: Received {data} event")
    return {"payload": "Bettina"}

# Calculate factorial (to generate CPU load)
def factorial(num):
    result = 1
    for i in range(1, num + 1):
        result *= i
    return result

# Perform a calculation to generate CPU load
def add_one_loop(n_loops):
    for _ in range(n_loops):
        factorial(20)

# Main function
def start_cpu_load(num_iters, available_cores, iter_per_core):
    for _ in range(num_iters):
        threads = []
        for _ in range(available_cores):
            thread = threading.Thread(target=add_one_loop, args=(iter_per_core,))
            threads.append(thread)
            thread.start()
        for thread in threads:
            thread.join()
async def st_func(client: Client):
    try:
        i = 0
        
        print("Starting loop")
        while True:
            workitem = client.pop_workitem(wiq="q2")
            if workitem is not None:
                id = workitem["id"]
                print(f"Updating workitem: {id}")
                workitem["state"] = "successful"
                client.update_workitem(workitem)
            i += 1
            if i % 500 == 0:
                print(f"looped 500 times")
        print("loop completed")
    except ClientError as e:
        print(f"Failed to connect to server: {e}")
        return

# Add handler variables at module level
f64_thread = None
u64_thread = None
i64_thread = None
stop_threads = False

# Add handler functions
def f64_handler(client):
    while not stop_threads:
        try:
            value = random.uniform(0, 50)
            client.info(f"Setting test_f64 to {value}")
            client.set_f64_observable_gauge("test_f64", value, "test")
            time.sleep(30)
        except (ArgumentError, Exception) as e:
            client.error(f"Error in f64_handler: {str(e)}")
            break

def u64_handler(client):
    while not stop_threads:
        try:
            value = int(random.uniform(0, 50))
            client.info(f"Setting test_u64 to {value}")
            client.set_u64_observable_gauge("test_u64", value, "test")
            time.sleep(30)
        except (ArgumentError, Exception) as e:
            client.error(f"Error in u64_handler: {str(e)}")
            break

def i64_handler(client):
    while not stop_threads:
        try:
            value = int(random.uniform(0, 50))
            client.info(f"Setting test_i64 to {value}")
            client.set_i64_observable_gauge("test_i64", value, "test")
            time.sleep(30)
        except (ArgumentError, Exception) as e:
            client.error(f"Error in i64_handler: {str(e)}")
            break

async def main():
    client = Client()
    client.enable_tracing("openiap=info", "")
    # client.enable_tracing("openiap=debug", "new")
    # client.enable_tracing("openiap=trace", "")
    client.info("Connecting to OpenIAP...")

    try:
        client.connect()
        client.info("Successfully connected to server")
    except ClientError as e:
        client.error(f"Failed to connect to server: {e}")
        return

    print("? for help")
    sthandle = None
    watch_id = ""

    while True:
        input_command = await keyboard_input()

        if input_command == "quit":
            global stop_threads
            stop_threads = True
            break
        elif input_command == "?":
            print("""
            ? for help
            quit: to quit
            q: Query
            qq: Query all
            di: Distinct
            s: Sign in as guest
            s2: Sign in as testuser
            i: Insert
            im: Insert Many
            d: Download
            u: Upload train.csv
            w: Watch
            uw: Unwatch
            r: Register queue
            m: Queue message
            o: Toggle f64 observable gauge
            o2: Toggle u64 observable gauge
            o3: Toggle i64 observable gauge
            cc: CustomCommand get users
            rpa: Invoke openrpa workflow "whoami" on "allan5"
            """)
        elif input_command == "0":
            client.disable_tracing()
        elif input_command == "1":
            client.enable_tracing("openiap=info", "")
        elif input_command == "2":
            client.enable_tracing("openiap=debug", "new")
        elif input_command == "3":
            client.enable_tracing("openiap=trace", "new")
        elif input_command == "dis":
            try:
                query_result = client.disconnect()
                client.info(query_result)
            except ClientError as e:
                client.error(f"Failed to query: {e}")
        elif input_command == "st":
            if sthandle is None:
                sthandle = asyncio.create_task(st_func(client))
            else:
                sthandle.cancel()
                sthandle = None
        elif input_command == "q":
            try:
                query_result = client.query(collectionname="entities", query="{}", projection="{\"name\":1}")
                client.info(query_result)
            except ClientError as e:
                client.error(f"Failed to query: {e}")
        elif input_command == "qq":
            try:
                query_result = client.query(collectionname="entities", query="{}")
                client.info(query_result)
            except ClientError as e:
                client.error(f"Failed to query: {e}")
        elif input_command == "di":
            try:
                distinct_result = client.distinct(collectionname="entities", field="_type", query="{}")
                client.info(distinct_result)
            except ClientError as e:
                client.error(f"Failed to query distinct: {e}")
        elif input_command in ["s", "s1"]:
            try:
                signin_result = client.signin(username="guest", password="password")
                client.info(f"Signed in as {signin_result.get('user').get('username')}")
            except ClientError as e:
                client.error(f"Failed to sign in: {e}")
        elif input_command in ["s2", "ss"]:
            try:
                signin_result = client.signin(username="testuser", password="badpassword")
                client.info(f"Signed in as {signin_result.get('user').get('username')}")
            except ClientError as e:
                client.error(f"Failed to sign in: {e}")
        elif input_command == "i":
            try:
                insert_one_result = client.insert_one(collectionname="entities", item=json.dumps({"name": "Allan", "_type": "test"}))
                client.info(f"Inserted as {insert_one_result}")
            except ClientError as e:
                client.error(f"Failed to insert: {e}")
        elif input_command == "im":
            try:
                items = [
                    {"name": "Allan", "_type": "test"},
                    {"name": "Allan2", "_type": "test"}
                ]
                insert_many_result = client.insert_many(collectionname="entities", items=json.dumps(items))
                client.info(f"Inserted as {insert_many_result}")
            except ClientError as e:
                client.error(f"Failed to insert: {e}")
        elif input_command == "d":
            try:
                download_result = client.download(collectionname="fs.files", id="65a3aaf66d52b8c15131aebd")
                client.info(f"Downloaded as {download_result}")
            except ClientError as e:
                client.error(f"Failed to download: {e}")
        elif input_command == "u":
            filepath = "train.csv"
            if not os.path.exists(filepath):
                filepath = "../" + filepath
            try:
                upload_result = client.upload(filepath=filepath, filename="train.csv")
                client.info(f"Uploaded as {upload_result}")
            except ClientError as e:
                client.error(f"Failed to upload: {e}")
        elif input_command == "r":
            try:
                register_result = client.register_queue(queuename="test2queue", callback=on_queue)
                client.info(f"Registered queue with id {register_result}")
            except ClientError as e:
                client.error(f"Failed to register queue: {e}")
        elif input_command == "r2":
            try:
                result = client.rpc_async('{"some": "data"}', queuename="test2queue", striptoken=True)
                client.info(result)  # Prints the RPC response
                result = client.rpc_async({"cmd": "getpackages"}, queuename="test2queue", striptoken=True)
                client.info(result)  # Prints the RPC response
                
            except ClientError as e:
                client.error(f"Failed to to RPC call: {e}")
        elif input_command == "m":
            try:
                client.queue_message( data="{\"message\": \"Hello, World as string!\"}", queuename="test2queue", striptoken=True)
                client.queue_message( data={"message": "Hello, World as object!"}, queuename="test2queue", striptoken=True)
            except ClientError as e:
                client.error(f"Failed to register queue: {e}")
        elif input_command == "w":
            try:
                watch_id = client.watch(collectionname="entities", callback=on_watch)
                client.info(f"Watch created with id {watch_id}")
            except ClientError as e:
                client.error(f"Failed to watch: {e}")
        elif input_command == "uw":
            try:
                if watch_id == "":
                    client.info("No watch to unwatch")
                    continue
                unwatch_result = client.unwatch(watch_id)
                client.info(f"Unwatched successfully: {unwatch_result}")
            except ClientError as e:
                client.error(f"Failed to unwatch: {e}")
        elif input_command == "c":
            try:
                client.create_collection("pythoncollection")
                client.info(f"Collection pythoncollection created")
            except ClientError as e:
                client.error(f"Failed to create collection: {e}")
        elif input_command == "c2":
            try:
                ts = ColTimeseriesWrapper(time_field=c_char_p("ts".encode('utf-8')))
                client.create_collection("pythontscollection", timeseries=ts)
                client.info(f"Collection pythontscollection created")
            except ClientError as e:
                client.error(f"Failed to unwatch: {e}")
        elif input_command == "gi":
            try:
                result = client.get_indexes("entities")
                client.info(f"indexes: {result}")
            except ClientError as e:
                client.error(f"Failed to get indexes: {e}")
        elif input_command == "c" or input_command == "cpu":
            num_calcs = 100000
            available_cores = os.cpu_count() // 2
            iter_per_core = num_calcs // available_cores
            num_iters = 5000
            threading.Thread(target=start_cpu_load, args=(num_iters, available_cores, iter_per_core)).start()
        elif input_command == "o":
            global f64_thread
            if f64_thread and f64_thread.is_alive():
                client.disable_observable_gauge("test_f64")
                client.info("stopped test_f64")
                stop_threads = True
                f64_thread.join()
                f64_thread = None
                stop_threads = False
            else:
                client.set_f64_observable_gauge("test_f64", 42.7, "test")
                client.info("started test_f64 to 42.7")
                f64_thread = threading.Thread(target=f64_handler, args=(client,))
                f64_thread.daemon = True
                f64_thread.start()

        elif input_command == "o2":
            global u64_thread
            if u64_thread and u64_thread.is_alive():
                client.disable_observable_gauge("test_u64")
                client.info("stopped test_u64")
                stop_threads = True
                u64_thread.join()
                u64_thread = None
                stop_threads = False
            else:
                client.set_u64_observable_gauge("test_u64", 42, "test")
                client.info("started test_u64 to 42")
                u64_thread = threading.Thread(target=u64_handler, args=(client,))
                u64_thread.daemon = True
                u64_thread.start()

        elif input_command == "o3":
            global i64_thread
            if i64_thread and i64_thread.is_alive():
                client.disable_observable_gauge("test_i64")
                client.info("stopped test_i64")
                stop_threads = True
                i64_thread.join()
                i64_thread = None
                stop_threads = False
            else:
                client.set_i64_observable_gauge("test_i64", 42, "test")
                client.info("started test_i64 to 42")
                i64_thread = threading.Thread(target=i64_handler, args=(client,))
                i64_thread.daemon = True
                i64_thread.start()

        elif input_command == "cc":
            try:
                # cmd = input("Enter custom command: ")
                cmd = "getclients"
                result = client.custom_command(cmd)
                print(f"Custom command result: {result}")
            except ClientError as e:
                print(f"Custom command failed: {e}")

        elif input_command == "rpa":
            try:
                result = client.invoke_openrpa(
                    "5ce94386320b9ce0bc2c3d07",
                    "5e0b52194f910e30ce9e3e49",
                    {"test": "test"},
                    timeout=10
                )
                print("Invoke OpenRPA result:", result)
            except Exception as e:
                print(f"Invoke OpenRPA failed: {e}")

    # Make sure to clean up threads before exiting
    stop_threads = True
    for thread in [f64_thread, u64_thread, i64_thread]:
        if thread and thread.is_alive():
            thread.join()

    if sthandle is not None:
        sthandle.cancel()
        sthandle = None

    client.info("*********************************")
    client.info("done, free client")
    client.info("*********************************")
    client.free()

if __name__ == "__main__":
    asyncio.run(main())
