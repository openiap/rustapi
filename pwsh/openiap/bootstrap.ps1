param(
    [string]$Debug = $env:DEBUG,
    [string]$SkipDebugCheck = $env:OPENIAP_SKIP_DEBUG_CHECK
)

function Get-Platform {
    if ($IsWindows) { return "windows" }
    elseif ($IsLinux) { return "linux" }
    elseif ($IsMacOS) { return "macos" }
    else { throw "Unsupported platform" }
}

function Get-Arch {
    if ($IsWindows) {
        switch ($env:PROCESSOR_ARCHITECTURE) {
            "AMD64" { return "x86_64" }
            "x86"   { return "x86" }
            "ARM64" { return "aarch64" }
            default { return $env:PROCESSOR_ARCHITECTURE }
        }
    } else {
        # On Linux/macOS, use uname to get architecture
        $unameOutput = uname -m
        switch ($unameOutput) {
            "x86_64"  { return "x86_64" }
            "amd64"   { return "x86_64" }
            "i386"    { return "x86" }
            "i686"    { return "x86" }
            "arm64"   { return "aarch64" }
            "aarch64" { return "aarch64" }
            default   { return $unameOutput }
        }
    }
}

$os = Get-Platform
$arch = Get-Arch
Write-Host "Detected OS: $os, Architecture: $arch"

# Determine debug library name
if ($os -eq "windows") {
    $debugLib = "openiap_clib.dll"
} elseif ($os -eq "linux") {
    if (Test-Path "/etc/alpine-release") {
        $debugLib = "libopeniap_clib_musl.so"
    } else {
        $debugLib = "libopeniap_clib.so"
    }
} elseif ($os -eq "macos") {
    $debugLib = "libopeniap_clib.dylib"
} else {
    throw "Unsupported platform: $os-$arch"
}

Write-Host "Debug library name: $debugLib"

# Search for debug/release library in parent directories
$libdir = Get-Location
$found = $false
if (-not $SkipDebugCheck) {
    for ($i = 0; $i -lt 10; $i++) {
        $debugPath = Join-Path $libdir "target/debug/$debugLib"
        $releasePath = Join-Path $libdir "target/release/$debugLib"
        if ($Debug) { Write-Host "Checking $debugPath" }
        if (Test-Path $debugPath) {
            Write-Output $debugPath
            $found = $true
            break
        }
        if (Test-Path $releasePath) {
            Write-Output $releasePath
            $found = $true
            break
        }
        $libdir = $libdir.Parent
        if (-not $libdir) { break }
    }
}

if (-not $found) {
    # Determine release library name
    if ($os -eq "windows" -and $arch -eq "x86") {
        $libName = "openiap-windows-i686.dll"
    } elseif ($os -eq "windows" -and $arch -eq "x86_64") {
        $libName = "openiap-windows-x64.dll"
    } elseif ($os -eq "windows" -and $arch -eq "aarch64") {
        $libName = "openiap-windows-arm64.dll"
    } elseif ($os -eq "linux" -and $arch -eq "x86_64") {
        if (Test-Path "/etc/alpine-release") {
            $libName = "libopeniap-linux-musl-x64.a"
        } else {
            $libName = "libopeniap-linux-x64.so"
        }
    } elseif ($os -eq "linux" -and $arch -eq "aarch64") {
        if (Test-Path "/etc/alpine-release") {
            $libName = "libopeniap-linux-musl-arm64.a"
        } else {
            $libName = "libopeniap-linux-arm64.so"
        }
    } elseif ($os -eq "macos" -and $arch -eq "x86_64") {
        $libName = "libopeniap-macos-x64.dylib"
    } elseif ($os -eq "macos" -and $arch -eq "aarch64") {
        $libName = "libopeniap-macos-arm64.dylib"
    } else {
        throw "Unsupported platform: $os-$arch"
    }

    # Create lib folder next to the PowerShell script
    $scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
    $libFolder = Join-Path $scriptDir "lib"
    if (-not (Test-Path $libFolder)) {
        New-Item -ItemType Directory -Path $libFolder | Out-Null
    }
    
    $dest = Join-Path $libFolder $libName
    if (Test-Path $dest) {
        Write-Output $dest
        exit 0
    }

    $url = "https://github.com/openiap/rustapi/releases/latest/download/$libName"
    if ($Debug) { Write-Host "Downloading $url to $dest" }
    try {
        Invoke-WebRequest -Uri $url -OutFile $dest
        Write-Output $dest
    } catch {
        throw "Error: failed to download $url"
    }
}