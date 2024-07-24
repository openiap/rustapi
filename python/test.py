import json
from openiap import Client, ClientError
import time, os



# Main function
if __name__ == "__main__":
    try:
        client = Client()
        client.connect()
        signin_result = client.signin()
        print(signin_result)

        for y in range(1, 3):
            promises = []
            for x in range(1, 10):
                client.query(collectionname="entities", query="{}", projection="{\"name\": 1}", orderby="", queryas="", explain=False, skip=0, top=0)
            
        # query_result = client.query(collectionname="entities", query="{}", projection="{\"name\": 1}", orderby="", queryas="", explain=False, skip=0, top=0)
        # print(query_result)

        aggregate_result = client.aggregate(collectionname="entities", aggregates="[]")
        print(aggregate_result)
        
        insert_one_result = client.insert_one(collectionname="entities", item="{\"name\": \"test from python\", \"_type\": \"test\"}")
        print(insert_one_result)


        download_result = client.download(collectionname="fs.files", id="65a3aaf66d52b8c15131aebd", folder="", filename="")
        print(download_result)

        filepath = "testfile.csv"
        # file exists ?
        if not os.path.exists(filepath):
            filepath = "../" + filepath
        upload_result = client.upload(filepath=filepath, filename="python-test.csv", mimetype="", metadata="", collectionname="fs.files")

        watchcounter = [0]  
        def onwatch(event):
            watchcounter[0] += 1
            print(f"Received event: {json.dumps(event, indent=2)}")

        watch_result = client.watch(collectionname="entities", paths="", callback=onwatch)
        print(watch_result)

        while watchcounter[0] < 2:
            time.sleep(1)
        unwatch_result =  client.unwatch(watch_result)
        print(unwatch_result)

    except ClientError as e:
        print(f"An error occurred: {e}")
