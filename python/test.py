import json
from openiap import Client, ClientError
import time, os



# Main function
if __name__ == "__main__":
    try:
        client = Client()
        # client.enable_tracing("openiap=trace", "new")
        client.enable_tracing("openiap=info", "")
        client.connect()
        signin_result = client.signin()
        print(signin_result)

        # for x in range(1, 10):
        #     client.query(collectionname="entities", query="{}", projection="{\"name\": 1}", orderby="", queryas="", explain=False, skip=0, top=0)


        print("*****************")
        workitem = client.push_workitem( name="python test", wiq="rustqueue", payload="{}")
        print(workitem)
            
        query_result = client.query(collectionname="entities", query="{}", projection="{\"name\": 1}", orderby="", queryas="", explain=False, skip=0, top=0)
        print(query_result)

        aggregate_result = client.aggregate(collectionname="entities", aggregates="[]")
        print(aggregate_result)
        
        insert_one_result = client.insert_one(collectionname="entities", item="{\"name\": \"test from python\", \"_type\": \"test\"}")
        print(insert_one_result)
        item = json.loads(insert_one_result)
        item["name"] = "test from python updated"
        update_one_result = client.update_one(collectionname="entities", item=json.dumps(item))
        print(update_one_result)
        id = item["_id"]

        insert_one_result = client.insert_one(collectionname="entities", item="{\"name\": \"test from python\", \"_type\": \"test\"}")
        print(insert_one_result)
        item = json.loads(insert_one_result)
        id2 = item["_id"]


        delete_many_query = client.delete_many(collectionname="entities", ids=[id, id2])
        print("Deleted ", delete_many_query, " items using ids")

        client.insert_many(collectionname="entities", items="[ {\"name\": \"test from python updated\", \"_type\": \"test\"}, {\"name\": \"test from python updated\", \"_type\": \"test\"} ]")
        print("added 2 items")

        delete_many_query = client.delete_many(collectionname="entities", query="{\"name\": \"test from python updated\"}")
        print("Deleted ", delete_many_query, " items using query")
        
        insert_or_update_one_result = client.insert_or_update_one(collectionname="entities", item="{\"name\": \"test insert or update from python\", \"_type\": \"test\"}")
        print(insert_or_update_one_result)
        item = json.loads(insert_or_update_one_result)

        client.delete_one(collectionname="entities", id=item["_id"])

        download_result = client.download(collectionname="fs.files", id="65a3aaf66d52b8c15131aebd", folder="", filename="")
        print(download_result)

        filepath = "testfile.csv"
        # file exists ?
        if not os.path.exists(filepath):
            filepath = "../" + filepath
        upload_result = client.upload(filepath=filepath, filename="python-test.csv", mimetype="", metadata="", collectionname="fs.files")

        watchcounter = [0]  
        def onwatch(event, counter):
            watchcounter[0] += 1
            operation = event["operation"]
            # print(f"{counter} Received event: {json.dumps(event, indent=2)}")
            print(f"{counter} Received {operation} event: ")

        watch_result = client.watch(collectionname="entities", paths="", callback=onwatch)
        print(watch_result)
        # client.insert_one(collectionname="entities", item="{\"name\": \"watch test from python 1\", \"_type\": \"test\"}")
        # client.insert_one(collectionname="entities", item="{\"name\": \"watch test from python 2\", \"_type\": \"test\"}")
        client.insert_many(collectionname="entities", items="[ {\"name\": \"watch test from python 1\", \"_type\": \"test\"}, {\"name\": \"watch test from python 2\", \"_type\": \"test\"} ]")

        while watchcounter[0] < 2:
            time.sleep(1)
        unwatch_result =  client.unwatch(watch_result)
        print(unwatch_result)

        count_result = client.count(collectionname="entities", query="{}")
        print(count_result)

        distinct_result = client.distinct(collectionname="entities", field="_type", query="{}")
        print(distinct_result)

    except ClientError as e:
        print(f"An error occurred: {e}")
