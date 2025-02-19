import asyncio
from ctypes import c_char_p
import json
from openiap import Client, ClientError
import os
import threading

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

async def main():
    client = Client()
    client.enable_tracing("openiap=info", "")
    # client.enable_tracing("openiap=debug", "new")
    # client.enable_tracing("openiap=trace", "")
    print("Connecting to OpenIAP...")

    try:
        client.connect()
    except ClientError as e:
        print(f"Failed to connect to server: {e}")
        return

    print("? for help")
    sthandle = None
    watch_id = ""

    while True:
        input_command = await keyboard_input()

        if input_command == "quit":
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
                print(query_result)
            except ClientError as e:
                print(f"Failed to query: {e}")
        elif input_command == "st":
            if sthandle is None:
                sthandle = asyncio.create_task(st_func(client))
            else:
                sthandle.cancel()
                sthandle = None
        elif input_command == "q":
            try:
                query_result = client.query(collectionname="entities", query="{}", projection="{\"name\":1}")
                print(query_result)
            except ClientError as e:
                print(f"Failed to query: {e}")
        elif input_command == "qq":
            try:
                query_result = client.query(collectionname="entities", query="{}")
                print(query_result)
            except ClientError as e:
                print(f"Failed to query: {e}")
        elif input_command == "di":
            try:
                distinct_result = client.distinct(collectionname="entities", field="_type", query="{}")
                print(distinct_result)
            except ClientError as e:
                print(f"Failed to query distinct: {e}")
        elif input_command in ["s", "s1"]:
            try:
                signin_result = client.signin(username="guest", password="password")
                print(f"Signed in as {signin_result.get('user').get('username')}")
            except ClientError as e:
                print(f"Failed to sign in: {e}")
        elif input_command in ["s2", "ss"]:
            try:
                signin_result = client.signin(username="testuser", password="badpassword")
                print(f"Signed in as {signin_result.get('user').get('username')}")
            except ClientError as e:
                print(f"Failed to sign in: {e}")
        elif input_command == "i":
            try:
                insert_one_result = client.insert_one(collectionname="entities", item=json.dumps({"name": "Allan", "_type": "test"}))
                print(f"Inserted as {insert_one_result}")
            except ClientError as e:
                print(f"Failed to insert: {e}")
        elif input_command == "im":
            try:
                items = [
                    {"name": "Allan", "_type": "test"},
                    {"name": "Allan2", "_type": "test"}
                ]
                insert_many_result = client.insert_many(collectionname="entities", items=json.dumps(items))
                print(f"Inserted as {insert_many_result}")
            except ClientError as e:
                print(f"Failed to insert: {e}")
        elif input_command == "d":
            try:
                download_result = client.download(collectionname="fs.files", id="65a3aaf66d52b8c15131aebd")
                print(f"Downloaded as {download_result}")
            except ClientError as e:
                print(f"Failed to download: {e}")
        elif input_command == "u":
            filepath = "train.csv"
            if not os.path.exists(filepath):
                filepath = "../" + filepath
            try:
                upload_result = client.upload(filepath=filepath, filename="train.csv")
                print(f"Uploaded as {upload_result}")
            except ClientError as e:
                print(f"Failed to upload: {e}")
        elif input_command == "r":
            try:
                register_result = client.register_queue(queuename="test2queue", callback=on_queue)
                print(f"Registered queue with id {register_result}")
            except ClientError as e:
                print(f"Failed to register queue: {e}")
        elif input_command == "r2":
            try:
                result = client.rpc_async('{"some": "data"}', queuename="test2queue", striptoken=True)
                print(result)  # Prints the RPC response
                result = client.rpc_async({"cmd": "getpackages"}, queuename="test2queue", striptoken=True)
                print(result)  # Prints the RPC response
                
            except ClientError as e:
                print(f"Failed to to RPC call: {e}")
        elif input_command == "m":
            try:
                client.queue_message( data="{\"message\": \"Hello, World as string!\"}", queuename="test2queue", striptoken=True)
                client.queue_message( data={"message": "Hello, World as object!"}, queuename="test2queue", striptoken=True)
            except ClientError as e:
                print(f"Failed to register queue: {e}")
        elif input_command == "w":
            try:
                watch_id = client.watch(collectionname="entities", callback=on_watch)
                print(f"Watch created with id {watch_id}")
            except ClientError as e:
                print(f"Failed to watch: {e}")
        elif input_command == "uw":
            try:
                if watch_id == "":
                    print("No watch to unwatch")
                    continue
                unwatch_result = client.unwatch(watch_id)
                print(f"Unwatched successfully: {unwatch_result}")
            except ClientError as e:
                print(f"Failed to unwatch: {e}")
        elif input_command == "c":
            try:
                client.create_collection("pythoncollection")
                print(f"Collection pythoncollection created")
            except ClientError as e:
                print(f"Failed to create collection: {e}")
        elif input_command == "c2":
            try:
                ts = ColTimeseriesWrapper(time_field=c_char_p("ts".encode('utf-8')))
                client.create_collection("pythontscollection", timeseries=ts)
                print(f"Collection pythontscollection created")
            except ClientError as e:
                print(f"Failed to unwatch: {e}")
        elif input_command == "gi":
            try:
                result = client.get_indexes("entities")
                print(f"indexes: {result}")
            except ClientError as e:
                print(f"Failed to get indexes: {e}")
        elif input_command == "c" or input_command == "cpu":
            num_calcs = 100000
            available_cores = os.cpu_count() // 2
            iter_per_core = num_calcs // available_cores
            num_iters = 5000
            threading.Thread(target=start_cpu_load, args=(num_iters, available_cores, iter_per_core)).start()

    if sthandle is not None:
        sthandle.cancel()
        sthandle = None

    print("*********************************")
    print("done, free client")
    print("*********************************")
    client.free()

if __name__ == "__main__":
    asyncio.run(main())
