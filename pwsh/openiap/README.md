Requires PowerShell Core, to download go to the official PowerShell GitHub releases page:
https://github.com/PowerShell/PowerShell/releases
To install
```bash
install-module openiap
```
To use it, you need to have setup credentials 
```bash
export apiurl=grpc://grpc.app.openiap.io:443
export jwt=eyJhbGciOiJI....
```
To uninstall the module
```bash
# remove from current runspace, incase it's been used in current session
Remove-Module openiap -ea 0
# remove the module from the machine
Uninstall-Module openiap
```

Doing development, you can load the module with
```
pwsh -NoExit -Command "Import-Module -DisableNameChecking ./pwsh/openiap"
```