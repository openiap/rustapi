import asyncio
import json
from openiap import Client, ClientError
import os
import threading

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

async def main():
    client = Client()
    client.enable_tracing("openiap=info", "")
    client.enable_tracing("openiap=debug", "new")
    client.enable_tracing("openiap=trace", "")
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
        elif input_command == "dis":
            try:
                query_result = client.disconnect()
                print(query_result)
            except ClientError as e:
                print(f"Failed to query: {e}")
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
                insert_one_result = client.insert_one(collectionname="entities", item=json.dumps({"name": "Allan", "_type": "Allan"}))
                print(f"Inserted as {insert_one_result}")
            except ClientError as e:
                print(f"Failed to insert: {e}")
        elif input_command == "im":
            try:
                items = [
                    {"name": "Allan", "_type": "Allan"},
                    {"name": "Allan2", "_type": "Allan"}
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
        elif input_command == "m":
            try:
                client.queue_message( data="{\"message\": \"Hello, World!\"}", queuename="test2queue", striptoken=True)
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
        elif input_command == "c" or input_command == "cpu":
            num_calcs = 100000
            available_cores = os.cpu_count() // 2
            iter_per_core = num_calcs // available_cores
            num_iters = 5000
            threading.Thread(target=start_cpu_load, args=(num_iters, available_cores, iter_per_core)).start()

    print("*********************************")
    print("done, free client")
    print("*********************************")
    client.free()

if __name__ == "__main__":
    asyncio.run(main())
