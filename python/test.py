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

        query_result = client.query(collectionname="entities", query="{}", projection="{\"name\": 1}", orderby="", queryas="", explain=False, skip=0, top=0)
        print(query_result)

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
        unwatch_result =  client.unwatch(watch_result["watchid"])
        print(unwatch_result)

    except ClientError as e:
        print(f"An error occurred: {e}")
