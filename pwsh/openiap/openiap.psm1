$ModuleRoot   = Split-Path -Parent $MyInvocation.MyCommand.Definition
$NativeFolder = Join-Path $ModuleRoot 'lib'
Write-Verbose "Loading OpenIap module from $ModuleRoot"
Write-Verbose "Native library folder: $NativeFolder"
# version of your native libraries on GitHub
$OpenIapVersion = '0.0.36'
$user = $null;
function Get-NativeLibFile {
    [CmdletBinding()] param()

    Write-Verbose "Looking for native library"

    # 1) pick the right base name + extension
    $arch = [System.Runtime.InteropServices.RuntimeInformation]::ProcessArchitecture
    if ($IsWindows) {
        if    ($arch -eq 'X64')   { $base = 'openiap-windows-x64' }
        elseif($arch -eq 'Arm64') { $base = 'openiap-windows-arm64' }
        else                      { $base = 'openiap-windows-i686' }
        $file = "$base.dll"
    }
    elseif ($IsMacOS) {
        if    ($arch -eq 'Arm64') { $base = 'openiap-macos-arm64' }
        else                      { $base = 'openiap-macos-x64' }
        $file = "lib$base.dylib"
    }
    else {
        # Linux: detect musl vs glibc
        $isMusl = Test-Path '/etc/alpine-release'
        $ext    = if ($isMusl) { '.a' } else { '.so' }
        if    ($arch -eq 'Arm64') { $base = 'openiap-linux-arm64' }
        else                      { $base = 'openiap-linux-x64' }
        $file = "lib$base$ext"
    }

    Write-Verbose "Native lib name: $file"

    if (-not (Test-Path $NativeFolder)) {
        New-Item -Path $NativeFolder -ItemType Directory | Out-Null
    }

    $localPath = Join-Path $NativeFolder $file

    # 3) if already downloaded, return it
    if (Test-Path $localPath) {
        Write-Verbose "Found locally at $localPath"
        return (Resolve-Path $localPath).Path
    }

    # 4) otherwise download from GitHub
    $releaseBase = "https://github.com/openiap/rustapi/releases/download/$OpenIapVersion"
    $downloadUrl = "$releaseBase/$file"

    Write-Verbose "Downloading native lib from $downloadUrl"
    Invoke-WebRequest -Uri $downloadUrl -OutFile $localPath -UseBasicParsing -ErrorAction Stop

    if (-not $IsWindows) {
        & chmod +x $localPath
    }

    Write-Verbose "Downloaded to $localPath"
    return (Resolve-Path $localPath).Path
}
Get-NativeLibFile
$ClientInstance = [OpenIap.Client]::new()
$firstCheck = $false;

function Ensure-Connected {
    if ($script:firstCheck -eq $false) {
        # You can prompt for URL or use a default/config value
        $url = $env:apiurl
        if (-not $url) { 
            $url = $env:grpcapiurl
        }
        if (-not $url) { 
            $url = $env:wsapiurl
        }
        if (-not $url) { 
            throw "Please make sure apiurl and jwt environment variables are set"
        }
        [OpenIap.Client]::client_set_agent_name($ClientInstance.clientPtr, "powershell")
        $user = $ClientInstance.connect($url)
        $script:firstCheck = $true
    } 
    return $user
}
function Get-State {
    Ensure-Connected | Out-Null
    $state = $ClientInstance.get_state();
    return $state
}
function Get-DefaultTimeout {
    Ensure-Connected | Out-Null
    $timeout = $ClientInstance.get_default_timeout();
    return $timeout
}
function Set-DefaultTimeout {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory)][int]$timeout
    )
    Ensure-Connected | Out-Null
    $ClientInstance.set_default_timeout($timeout);
}
function Invoke-Query {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory)][string]$Collection,
        [Parameter()][string]$Query = '{}',
        [Parameter()][string]$Projection = '',
        [Parameter()][string]$OrderBy = '',
        [Parameter()][string]$QueryAs = '',
        [Parameter()][bool]$Explain = $false,
        [Parameter()][int]$Skip = 0,
        [Parameter()][int]$Top = 100
    )
    Ensure-Connected | Out-Null
    $json = $ClientInstance.Query[string]($Collection, $Query, $Projection, $OrderBy, $QueryAs, $Explain, $Skip, $Top).GetAwaiter().GetResult()
    $result = $null
    if ($json) {
        $result = $json | ConvertFrom-Json
    }
    return $result
}

function Invoke-Aggregate {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory)][string]$Collection,
        [Parameter(Mandatory)][string]$Aggregates,
        [Parameter()][string]$QueryAs = '',
        [Parameter()][string]$Hint = '',
        [Parameter()][bool]$Explain = $false
    )
    Ensure-Connected | Out-Null
    $json = $ClientInstance.Aggregate[string]($Collection, $Aggregates, $QueryAs, $Hint, $Explain).GetAwaiter().GetResult()
    $result = $null
    if ($json) {
        $result = $json | ConvertFrom-Json
    }
    return $result
}

function Invoke-OpenRPA {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory)][string]$RobotId,
        [Parameter(Mandatory)][string]$WorkflowId,
        [Parameter()][string]$Payload = '{}',
        [Parameter()][bool]$Rpc = $true,
        [Parameter()][int]$Timeout = -1
    )
    Ensure-Connected | Out-Null
    $json = $ClientInstance.InvokeOpenRPA($RobotId, $WorkflowId, $Payload, $Rpc, $Timeout)
    $result = $null
    if ($json) {
        $result = $json | ConvertFrom-Json
    }
    return $result
}

function Invoke-InsertOne {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory)][string]$Collection,
        [Parameter(Mandatory)][string]$Item,
        [Parameter()][int]$W = 1,
        [Parameter()][bool]$J = $false
    )
    Ensure-Connected | Out-Null
    $json = $ClientInstance.InsertOne[string]($Collection, $Item, $W, $J).GetAwaiter().GetResult()
    if (-not $json) {
        return @()
    }
    $parsedObjects = @(
        $json | ConvertFrom-Json
    )
    return $parsedObjects

}

function Invoke-InsertMany {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory)][string]$Collection,
        [Parameter(Mandatory)][string]$Items,
        [Parameter()][int]$W = 1,
        [Parameter()][bool]$J = $false,
        [Parameter()][bool]$SkipResults = $false
    )
    Ensure-Connected | Out-Null
    $json = $ClientInstance.InsertMany[string]($Collection, $Items, $W, $J, $SkipResults).GetAwaiter().GetResult()
    if (-not $json) {
        return @()
    }
    $parsedObjects = @(
        $json | ConvertFrom-Json
    )
    return $parsedObjects
}

function Invoke-UpdateOne {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory)][string]$Collection,
        [Parameter(Mandatory)][string]$Item,
        [Parameter()][int]$W = 1,
        [Parameter()][bool]$J = $false
    )
    Ensure-Connected | Out-Null
    $json = $ClientInstance.UpdateOne[string]($Collection, $Item, $W, $J).GetAwaiter().GetResult()
    if (-not $json) {
        return @()
    }
    $parsedObjects = @(
        $json | ConvertFrom-Json
    )
    return $parsedObjects

}

function Invoke-InsertOrUpdateOne {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory)][string]$Collection,
        [Parameter(Mandatory)][string]$Item,
        [Parameter()][string]$Uniqeness = "_id",
        [Parameter()][int]$W = 1,
        [Parameter()][bool]$J = $false
    )
    Ensure-Connected | Out-Null
    $json = $ClientInstance.InsertOrUpdateOne[string]($Collection, $Item, $Uniqeness, $W, $J).GetAwaiter().GetResult()
    if (-not $json) {
        return @()
    }
    $parsedObjects = @(
        $json | ConvertFrom-Json
    )
    return $parsedObjects

}

function Invoke-DeleteOne {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory)][string]$Collection,
        [Parameter(Mandatory)][string]$Id,
        [Parameter()][bool]$Recursive = $false
    )
    Ensure-Connected | Out-Null
    $result = $ClientInstance.DeleteOne($Collection, $Id, $Recursive).GetAwaiter().GetResult()
    $result
}

function Invoke-DeleteMany {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory)][string]$Collection,
        [Parameter()][string]$Query = "",
        [Parameter()][string[]]$Ids = @(),
        [Parameter()][bool]$Recursive = $false
    )
    Ensure-Connected | Out-Null
    $result = $ClientInstance.DeleteMany($Collection, $Query, $Ids, $Recursive).GetAwaiter().GetResult()
    $result
}

function Invoke-Count {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory)][string]$Collection,
        [Parameter()][string]$Query = "",
        [Parameter()][string]$QueryAs = "",
        [Parameter()][bool]$Explain = $false
    )
    Ensure-Connected | Out-Null
    $result = $ClientInstance.Count($Collection, $Query, $QueryAs, $Explain).GetAwaiter().GetResult()
    $result
}

function Invoke-Distinct {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory)][string]$Collection,
        [Parameter(Mandatory)][string]$Field,
        [Parameter()][string]$Query = "",
        [Parameter()][string]$QueryAs = "",
        [Parameter()][bool]$Explain = $false
    )
    Ensure-Connected | Out-Null
    $result = $ClientInstance.Distinct($Collection, $Field, $Query, $QueryAs, $Explain).GetAwaiter().GetResult()
    $result
}

function Invoke-CreateCollection {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory)][string]$Collection,
        [Parameter()][int]$ExpireAfterSeconds = 0,
        [Parameter()][bool]$ChangeStreamPreAndPostImages = $false,
        [Parameter()][bool]$Capped = $false,
        [Parameter()][int]$Max = 0,
        [Parameter()][int]$Size = 0
        # Collation and Timeseries omitted for simplicity
    )
    Ensure-Connected | Out-Null
    $ClientInstance.CreateCollection($Collection, $null, $null, $ExpireAfterSeconds, $ChangeStreamPreAndPostImages, $Capped, $Max, $Size).GetAwaiter().GetResult()
}

function Invoke-DropCollection {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory)][string]$Collection
    )
    Ensure-Connected | Out-Null
    $ClientInstance.DropCollection($Collection).GetAwaiter().GetResult()
}

function Invoke-GetIndexes {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory)][string]$Collection
    )
    Ensure-Connected | Out-Null
    $json = $ClientInstance.GetIndexes[string]($Collection).GetAwaiter().GetResult()
    if (-not $json) {
        return @()
    }
    $parsedObjects = @(
        $json | ConvertFrom-Json
    )
    return $parsedObjects

}

function Invoke-CreateIndex {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory)][string]$Collection,
        [Parameter(Mandatory)][string]$Index,
        [Parameter()][string]$Options = "",
        [Parameter()][string]$Name = ""
    )
    Ensure-Connected | Out-Null
    $ClientInstance.CreateIndex($Collection, $Index, $Options, $Name).GetAwaiter().GetResult()
}

function Invoke-DropIndex {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory)][string]$Collection,
        [Parameter(Mandatory)][string]$IndexName
    )
    Ensure-Connected | Out-Null
    $ClientInstance.DropIndex($Collection, $IndexName).GetAwaiter().GetResult()
}

function Invoke-Upload {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory)][string]$FilePath,
        [Parameter()][string]$FileName = "",
        [Parameter()][string]$MimeType = "",
        [Parameter()][string]$Metadata = "",
        [Parameter()][string]$Collection = ""
    )
    Ensure-Connected | Out-Null
    if (-not $FileName) {
        $FileName = [System.IO.Path]::GetFileName($FilePath)
    }
    $ClientInstance.upload($FilePath, $FileName, $MimeType, $Metadata, $Collection).GetAwaiter().GetResult()
}

function Invoke-Download {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory)][string]$Collection,
        [Parameter(Mandatory)][string]$Id,
        [Parameter()][string]$Folder = "",
        [Parameter()][string]$FileName = ""
    )
    Ensure-Connected | Out-Null
    $ClientInstance.download($Collection, $Id, $Folder, $FileName).GetAwaiter().GetResult()
}

function Invoke-QueueMessage {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory)][string]$Data,
        [Parameter()][string]$QueueName = "",
        [Parameter()][string]$ExchangeName = "",
        [Parameter()][string]$ReplyTo = "",
        [Parameter()][string]$RoutingKey = "",
        [Parameter()][string]$CorrelationId = "",
        [Parameter()][bool]$StripToken = $false,
        [Parameter()][int]$Expiration = 0
    )
    Ensure-Connected | Out-Null
    $ClientInstance.QueueMessage($Data, $QueueName, $ExchangeName, $ReplyTo, $RoutingKey, $CorrelationId, $StripToken, $Expiration).GetAwaiter().GetResult()  | Out-Null
}

function Invoke-Rpc {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory)][string]$Data,
        [Parameter()][string]$QueueName = "",
        [Parameter()][string]$ExchangeName = "",
        [Parameter()][string]$RoutingKey = "",
        [Parameter()][bool]$StripToken = $false,
        [Parameter()][int]$Expiration = 0,
        [Parameter()][int]$timeout = -1
    )
    Ensure-Connected | Out-Null
    $ClientInstance.Rpc($Data, $QueueName, $ExchangeName, $RoutingKey, $StripToken, $Expiration, $timeout).GetAwaiter().GetResult()
}

function Invoke-CustomCommand {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory)][string]$Command,
        [Parameter()][string]$Id = "",
        [Parameter()][string]$Data = "",
        [Parameter()][string]$Name = "",
        [Parameter()][int]$Timeout = -1
    )
    Ensure-Connected | Out-Null
    $json = $ClientInstance.custom_command[string]($Command, $Id, $Data, $Name, $Timeout).GetAwaiter().GetResult()
    if (-not $json) {
        return @()
    }
    $parsedObjects = @(
        $json | ConvertFrom-Json
    )
    return $parsedObjects

}

function Invoke-PushWorkitem {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory)][string]$Wiq,
        [Parameter(Mandatory)][psobject]$Item,
        [Parameter()][string[]]$Files = @()
    )
    Ensure-Connected | Out-Null
    $ClientInstance.PushWorkitem($Wiq, $Item, $Files).GetAwaiter().GetResult()
}

function Invoke-PopWorkitem {
    [CmdletBinding()]
    param(
        [Parameter()][string]$Wiq = "",
        [Parameter()][string]$WiqId = "",
        [Parameter()][string]$DownloadFolder = "."
    )
    Ensure-Connected | Out-Null
    $ClientInstance.PopWorkitem($Wiq, $WiqId, $DownloadFolder).GetAwaiter().GetResult()
}

function Invoke-UpdateWorkitem {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory)][psobject]$Workitem,
        [Parameter()][string[]]$Files = @(),
        [Parameter()][bool]$IgnoreMaxRetries = $false
    )
    Ensure-Connected | Out-Null
    $ClientInstance.UpdateWorkitem($Workitem, $Files, $IgnoreMaxRetries).GetAwaiter().GetResult()
}

function Invoke-DeleteWorkitem {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory)][string]$Id
    )
    Ensure-Connected | Out-Null
    $ClientInstance.DeleteWorkitem($Id).GetAwaiter().GetResult() | Out-Null
}

function Invoke-Signin {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory)][string]$UsernameOrToken = "",
        [Parameter()][string]$Password = ""
    )
    Ensure-Connected | Out-Null
    $result = $ClientInstance.Signin($UsernameOrToken, $Password).GetAwaiter().GetResult()
    if($result -and $result[2] -eq $true) {
        return $result[0]
    } elseif ($result -and $result[2] -eq $false) {
        throw $result[1]
    } else {
        throw "Signin failed, unknown error"
    }
}
