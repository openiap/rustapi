# Module manifest for module 'openiap'
# Generated by: Allan Ximmermann
# Generated on: 23/04/2025
@{

# Script module or binary module file associated with this manifest.
# RootModule = 'openiap.psm1'
RootModule = 'openiap.psm1'

# Version number of this module.
ModuleVersion = '0.0.38'

# Supported PSEditions
# CompatiblePSEditions = @()

# ID used to uniquely identify this module
GUID = 'b26d31fa-3c02-4d89-a2a0-45f36b9b24f6'

# Author of this module
Author = 'Allan Zimmermann'

# Company or vendor of this module
CompanyName = 'OpenIAP ApS'

# Copyright statement for this module
Copyright = '(c) OpenIAP. All rights reserved.'

# Description of the functionality provided by this module
Description = 'Interact and manage OpenIAP OpenCore installation from PowerShell'

# Functions to export from this module, for best performance, do not use wildcards and do not delete the entry, use an empty array if there are no functions to export.
FunctionsToExport = @(
    'Get-DefaultTimeout',
    'Set-DefaultTimeout',
    'Get-State',
    'Invoke-Query',
    'Invoke-Aggregate',
    'Invoke-OpenRPA',
    'Invoke-InsertOne',
    'Invoke-InsertMany',
    'Invoke-UpdateOne',
    'Invoke-InsertOrUpdateOne',
    'Invoke-DeleteOne',
    'Invoke-DeleteMany',
    'Invoke-Count',
    'Invoke-Distinct',
    'Invoke-CreateCollection',
    'Invoke-DropCollection',
    'Invoke-GetIndexes',
    'Invoke-CreateIndex',
    'Invoke-DropIndex',
    'Invoke-Upload',
    'Invoke-Download',
    'Invoke-QueueMessage',
    'Invoke-Rpc',
    'Invoke-CustomCommand',
    'Invoke-PushWorkitem',
    'Invoke-PopWorkitem',
    'Invoke-UpdateWorkitem',
    'Invoke-DeleteWorkitem',
    'Invoke-Signin'
)

# Cmdlets to export from this module, for best performance, do not use wildcards and do not delete the entry, use an empty array if there are no cmdlets to export.
CmdletsToExport = @()

# Variables to export from this module
VariablesToExport = '*'

# Minimum version of the PowerShell engine required by this module
PowerShellVersion = '7.0'

# Name of the PowerShell host required by this module
# PowerShellHostName = ''

# Minimum version of the PowerShell host required by this module
# PowerShellHostVersion = ''

# Minimum version of Microsoft .NET Framework required by this module. This prerequisite is valid for the PowerShell Desktop edition only.
# DotNetFrameworkVersion = ''

# Minimum version of the common language runtime (CLR) required by this module. This prerequisite is valid for the PowerShell Desktop edition only.
# ClrVersion = ''

# Processor architecture (None, X86, Amd64) required by this module
# ProcessorArchitecture = ''

# Modules that must be imported into the global environment prior to importing this module
# RequiredModules = @()

# Assemblies that must be loaded prior to importing this module
# RequiredAssemblies = @()

# Script files (.ps1) that are run in the caller's environment prior to importing this module.
# ScriptsToProcess = @()
# ScriptsToProcess = 'scripts\bootstrap.ps1'
RequiredAssemblies = @('openiap.dll')

# Type files (.ps1xml) to be loaded when importing this module
# TypesToProcess = @()

# Format files (.ps1xml) to be loaded when importing this module
# FormatsToProcess = @()

# Modules to import as nested modules of the module specified in RootModule/ModuleToProcess
# NestedModules = @()

# Aliases to export from this module, for best performance, do not use wildcards and do not delete the entry, use an empty array if there are no aliases to export.
AliasesToExport = @()

# DSC resources to export from this module
# DscResourcesToExport = @()

# List of all modules packaged with this module
# ModuleList = @()

# List of all files packaged with this module
# FileList = @()

# Private data to pass to the module specified in RootModule/ModuleToProcess. This may also contain a PSData hashtable with additional module metadata used by PowerShell.
PrivateData = @{

    PSData = @{

        # Tags applied to this module. These help with module discovery in online galleries.
        # Tags = @()

        # A URL to the license for this module.
        # LicenseUri = ''

        # A URL to the main website for this project.
        # ProjectUri = ''

        # A URL to an icon representing this module.
        # IconUri = ''

        # ReleaseNotes of this module
        # ReleaseNotes = ''

        # Prerelease string of this module
        # Prerelease = ''

        # Flag to indicate whether the module requires explicit user acceptance for install/update/save
        # RequireLicenseAcceptance = $false

        # External dependent modules of this module
        # ExternalModuleDependencies = @()

    } # End of PSData hashtable

} # End of PrivateData hashtable

# HelpInfo URI of this module
# HelpInfoURI = ''

# Default prefix for commands exported from this module. Override the default prefix using Import-Module -Prefix.
# DefaultCommandPrefix = ''

}

