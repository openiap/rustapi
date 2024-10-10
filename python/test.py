import json
from openiap import Client, ClientError
import time, os



# Main function
if __name__ == "__main__":
    client = Client()
    try:
        # client.enable_tracing("openiap=trace", "new")
        client.enable_tracing("openiap=info", "")
        client.info("Connecting to OpenIAP")
        client.connect()

        eventcounter = [0]  
        def onclientevent(result, counter):
            eventcounter[0] += 1
            event = result["event"]
            reason = result["reason"]
            print(f"Client event #{counter} Received {result} event: {reason}")

        eventid = client.on_client_event(callback=onclientevent)
        print("Client event, registered with id: ", eventid)
        signin_result = client.signin()
        print(signin_result)

        # # for x in range(1, 10):
        # #     client.query(collectionname="entities", query="{}", projection="{\"name\": 1}", orderby="", queryas="", explain=False, skip=0, top=0)


        print("Turning off client event, id: ", eventid)
        client.off_client_event(eventid)

        files = []
        if(os.path.exists("testfile.csv")):
            files.append("testfile.csv")
        else:
            files.append("../testfile.csv")
        workitem = client.push_workitem( name="python test with file", wiq="rustqueue", payload="{}", files=files)
        print(workitem)

        workitem = client.pop_workitem( wiq="rustqueue")
        print(workitem)
        workitem["state"] = "successful"
        client.update_workitem(workitem)
        print(workitem)

        client.delete_workitem(workitem["id"])

        workitem = client.push_workitem( name="python without file", wiq="rustqueue", payload="{}")
        print(workitem)

        workitem = client.pop_workitem( wiq="rustqueue")
        print(workitem)
        workitem["state"] = "successful"
        workitem["name"] = "python updated, now including a file"
        files = []
        if(os.path.exists("testfile.csv")):
            files.append("testfile.csv")
        else:
            files.append("../testfile.csv")

        client.update_workitem(workitem, files)
        print(workitem)
        client.delete_workitem(workitem["id"])

        files = []
        workitem = client.push_workitem( name="python without file", wiq="rustqueue", payload="{}")
        print(workitem)
        workitem = client.pop_workitem( wiq="rustqueue")
        workitem["state"] = "successful"
        client.update_workitem(workitem, files)


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
        client.insert_many(collectionname="entities", items="[ {\"name\": \"watch test from python 1\", \"_type\": \"test\"}, {\"name\": \"watch test from python 2\", \"_type\": \"test\"} ]")

        while watchcounter[0] < 2:
            time.sleep(1)
        unwatch_result =  client.unwatch(watch_result)
        print(unwatch_result)

        count_result = client.count(collectionname="entities", query="{}")
        print(count_result)

        distinct_result = client.distinct(collectionname="entities", field="_type", query="{}")
        print(distinct_result)

        queuecounter = [0]  
        def onmessage(event, counter):
            queuecounter[0] += 1
            data = event["data"]
            # print(f"{counter} Received event: {json.dumps(event, indent=2)}")
            print(f"{counter} Received {data} event: ")

        register_queue_result = client.register_queue(queuename="test2queue", callback=onmessage)
        print(register_queue_result)

        client.queue_message(queuename="test2queue", data="{\"test\": \"message 1\"}", striptoken=True)
        client.queue_message(queuename="test2queue", data="{\"test\": \"message 2\"}", striptoken=True)

        while queuecounter[0] < 2:
            time.sleep(1)
        unregister_queue = client.unregister_queue(register_queue_result)
        print(unregister_queue)


        exchangecounter = [0]  
        def onexchange(event, counter):
            exchangecounter[0] += 1
            data = event["data"]
            # print(f"{counter} Received event: {json.dumps(event, indent=2)}")
            print(f"{counter} Received {data} event: ")

        register_exchange_result = client.register_exchange(exchangename="testexc", callback=onexchange)
        print(register_exchange_result)

        client.queue_message(exchangename="testexc", data="{\"test\": \"message 1\"}", striptoken=True)
        client.queue_message(exchangename="testexc", data="{\"test\": \"message 2\"}", striptoken=True)

        while exchangecounter[0] < 2:
            time.sleep(1)
        unregister_queue =  client.unregister_queue(register_exchange_result)
        print(unregister_queue)

    except ClientError as e:
        print(f"An error occurred: {e}")
    print("*********************************")
    print("done, free client")
    print("*********************************")
    client.free()
