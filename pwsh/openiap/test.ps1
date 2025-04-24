# Import-Module ./ps/openiap.psd1 -Force -Verbose && Invoke-Aggregate entities []
Write-Host "Testing OpenIAP PowerShell module..."

# Query
Write-Host "`nInvoke-Query:"
$result = Invoke-Query -Collection "entities" -Top 10 -Projection '{"name":1}'
$result | Format-Table -AutoSize

# Aggregate
Write-Host "`nInvoke-Aggregate:"
$agg = '[{"$match": {"_type": "test"}}, {"$limit": 10}]'
$result = Invoke-Aggregate -Collection "entities" -Aggregates []
$result | Format-Table -AutoSize

# InsertOne
Write-Host "`nInvoke-InsertOne:"
$item = '{"name": "test from ps", "_type": "test"}'
$result = Invoke-InsertOne -Collection "entities" -Item $item
$result | Format-Table -AutoSize

# InsertMany
Write-Host "`nInvoke-InsertMany:"
$items = '[{"name": "ps1", "_type": "test"}, {"name": "ps2", "_type": "test"}]'
$result = Invoke-InsertMany -Collection "entities" -Items $items
$result | Format-Table -AutoSize

# UpdateOne
Write-Host "`nInvoke-UpdateOne:"
if ($result -and $result[0]._id) {
    $item = $result[0] | ConvertTo-Json
    $item = $item -replace '"name":\s*"[^"]+"', '"name":"updated from ps"'
    $result2 = Invoke-UpdateOne -Collection "entities" -Item $item
    $result2 | Format-Table -AutoSize
}

# InsertOrUpdateOne
Write-Host "`nInvoke-InsertOrUpdateOne:"
$item = '{"name": "unique from ps", "_type": "test", "_id": "ps-unique-1"}'
$result = Invoke-InsertOrUpdateOne -Collection "entities" -Item $item
$result | Format-Table -AutoSize

# DeleteOne
Write-Host "`nInvoke-DeleteOne:"
if ($result -and $result._id) {
    $delres = Invoke-DeleteOne -Collection "entities" -Id $result._id
    Write-Host "Deleted rows: $delres"
}

# DeleteMany
Write-Host "`nInvoke-DeleteMany:"
$delres = Invoke-DeleteMany -Collection "entities" -Query '{"name":"ps1"}'
Write-Host "Deleted rows: $delres"

# Count
Write-Host "`nInvoke-Count:"
$count = Invoke-Count -Collection "entities"
Write-Host "Count: $count"

# Distinct
Write-Host "`nInvoke-Distinct:"
$distinct = Invoke-Distinct -Collection "entities" -Field "name"
$distinct | Format-Table -AutoSize

# CreateCollection
Write-Host "`nInvoke-CreateCollection:"
Invoke-CreateCollection -Collection "testpscollection"
Write-Host "Created collection testpscollection"

# DropCollection
Write-Host "`nInvoke-DropCollection:"
Invoke-DropCollection -Collection "testpscollection"
Write-Host "Dropped collection testpscollection"

# GetIndexes
Write-Host "`nInvoke-GetIndexes:"
$indexes = Invoke-GetIndexes -Collection "entities"
$indexes | Format-Table -AutoSize

# CreateIndex
Write-Host "`nInvoke-CreateIndex:"
Invoke-CreateIndex -Collection "entities" -Index '{"name":1}' -Name "name_1"
Write-Host "Created index name_1"

# DropIndex
Write-Host "`nInvoke-DropIndex:"
Invoke-DropIndex -Collection "entities" -IndexName "name_1"
Write-Host "Dropped index name_1"

# Upload
Write-Host "`nInvoke-Upload:"
# Uncomment and set a valid file path to test
$uploadId = Invoke-Upload -FilePath "testfile.csv"
Write-Host "Uploaded file id: $uploadId"

# Download
Write-Host "`nInvoke-Download:"
# Uncomment and set a valid id to test
$filename = Invoke-Download -Collection "fs.files" -Id $uploadId -Filename "train.csv"
Write-Host "Downloaded file: $filename"

# Watch/UnWatch (event handler not supported in PS, just test registration)
Write-Host "`nInvoke-Watch:"
$watchid = Invoke-Watch -Collection "entities" -Paths "[]"
Write-Host "Watch id: $watchid"
Write-Host "`nInvoke-UnWatch:"
Invoke-UnWatch -WatchId $watchid
Write-Host "Unwatched $watchid"

# RegisterQueue/UnRegisterQueue (event handler not supported in PS, just test registration)
Write-Host "`nInvoke-RegisterQueue:"
Invoke-RegisterQueue -QueueName "testpsqueue"
Write-Host "Registered queue testpsqueue"
Write-Host "`nInvoke-UnRegisterQueue:"
Invoke-UnRegisterQueue -QueueName "testpsqueue"
Write-Host "Unregistered queue testpsqueue"

# QueueMessage
Write-Host "`nInvoke-QueueMessage:"
Invoke-QueueMessage -Data '{"message":"Hello from ps"}' -QueueName "testpsqueue"
Write-Host "Queued message"

# Rpc
Write-Host "`nInvoke-Rpc:"
Invoke-Rpc -Data '{"message":"Hello from ps"}' -QueueName "testpsqueue"
Write-Host "RPC message sent"

# CustomCommand
Write-Host "`nInvoke-CustomCommand:"
$cc = Invoke-CustomCommand -Command "getclients"
$cc | Format-Table -AutoSize

# PushWorkitem/PopWorkitem/UpdateWorkitem/DeleteWorkitem
Write-Host "`nInvoke-PushWorkitem:"
$wi = @{ name = "test from ps"; payload = '{"_type":"test"}' }
$wi = Invoke-PushWorkitem -Wiq "q2" -Item $wi
Write-Host "Pushed workitem as $($wi.id)"

Write-Host "`nInvoke-PopWorkitem:"
$popwi = Invoke-PopWorkitem -Wiq "q2"
$popwi | Format-Table -AutoSize

Write-Host "`nInvoke-UpdateWorkitem:"
if ($popwi) {
    $popwi.state = "successful"
    $result = Invoke-UpdateWorkitem -Workitem $popwi
    Write-Host "Updated workitem"
    $popwi | Format-Table -AutoSize
}

Write-Host "`nInvoke-DeleteWorkitem:"
if ($popwi -and $popwi.id) {
    Invoke-DeleteWorkitem -Id $popwi.id
    Write-Host "Deleted workitem"
}

# OnClientEvent/OffClientEvent (event handler not supported in PS, just test registration)
Write-Host "`nInvoke-OnClientEvent:"
$eventid = Invoke-OnClientEvent
Write-Host "Registered client event: $eventid"
Write-Host "`nInvoke-OffClientEvent:"
Invoke-OffClientEvent -EventId $eventid
Write-Host "Unregistered client event: $eventid"

# OpenRPA
Write-Host "`nInvoke-OpenRPA:"
Invoke-OpenRPA -RobotId "5ce94386320b9ce0bc2c3d07" -WorkflowId "5e0b52194f910e30ce9e3e49"
Write-Host "Invoked OpenRPA"

Write-Host "`nAll tests completed."