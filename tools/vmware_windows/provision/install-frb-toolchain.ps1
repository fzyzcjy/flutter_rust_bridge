Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"
$ProgressPreference = "SilentlyContinue"

Write-Information -MessageData "[frb-windows-toolchain] Installing FRB Windows toolchain" -InformationAction Continue

if ($env:FRB_WINDOWS_GUEST_PROXY_URL) {
    $proxyMessage = "[frb-windows-toolchain] Configuring guest proxy from FRB_WINDOWS_GUEST_PROXY_URL"
    Write-Information -MessageData $proxyMessage -InformationAction Continue
    netsh winhttp set proxy $env:FRB_WINDOWS_GUEST_PROXY_URL
    [Environment]::SetEnvironmentVariable("HTTP_PROXY", $env:FRB_WINDOWS_GUEST_PROXY_URL, "Machine")
    [Environment]::SetEnvironmentVariable("HTTPS_PROXY", $env:FRB_WINDOWS_GUEST_PROXY_URL, "Machine")
    $env:HTTP_PROXY = $env:FRB_WINDOWS_GUEST_PROXY_URL
    $env:HTTPS_PROXY = $env:FRB_WINDOWS_GUEST_PROXY_URL
}

if (-not (Get-Command -Name winget -ErrorAction SilentlyContinue)) {
    throw "winget is required in the Windows base image."
}

winget source update
winget install --accept-package-agreements --accept-source-agreements --silent Microsoft.PowerShell
winget install --accept-package-agreements --accept-source-agreements --silent Git.Git
winget install --accept-package-agreements --accept-source-agreements --silent Kitware.CMake
winget install --accept-package-agreements --accept-source-agreements --silent Ninja-build.Ninja
winget install --accept-package-agreements --accept-source-agreements --silent Rustlang.Rustup
$vsBuildToolsArgs = @(
    "install",
    "--accept-package-agreements",
    "--accept-source-agreements",
    "--silent",
    "Microsoft.VisualStudio.2022.BuildTools",
    "--override",
    "--wait --quiet --norestart --includeRecommended " +
        "--add Microsoft.VisualStudio.Workload.NativeDesktop " +
        "--add Microsoft.VisualStudio.Workload.VCTools " +
        "--add Microsoft.VisualStudio.Component.VC.Tools.ARM64 " +
        "--add Microsoft.VisualStudio.Component.VC.CMake.Project"
)
winget @vsBuildToolsArgs

$flutterRoot = "C:\flutter"
if (-not (Test-Path -Path "$flutterRoot\bin\flutter.bat")) {
    $flutterVersion = "3.44.1"
    $flutterZip = "C:\frb\downloads\flutter_windows.zip"
    New-Item -ItemType Directory -Force -Path "C:\frb\downloads" | Out-Null
    $flutterUrl = "https://storage.googleapis.com/flutter_infra_release/" +
        "releases/stable/windows/flutter_windows_${flutterVersion}-stable.zip"
    $webRequestArgs = @{
        Uri = $flutterUrl
        OutFile = $flutterZip
    }
    if ($env:FRB_WINDOWS_GUEST_PROXY_URL) {
        $webRequestArgs.Proxy = $env:FRB_WINDOWS_GUEST_PROXY_URL
    }
    try {
        Invoke-WebRequest @webRequestArgs
        Expand-Archive -Path $flutterZip -DestinationPath C:\ -Force
    } catch {
        Write-Information -MessageData "[frb-windows-toolchain] Flutter zip download failed; falling back to git clone" -InformationAction Continue
        Remove-Item -Path $flutterZip -Force -ErrorAction SilentlyContinue
        git clone --branch $flutterVersion --depth 1 https://github.com/flutter/flutter.git $flutterRoot
    }
}

$machinePath = [Environment]::GetEnvironmentVariable("Path", "Machine")
$machineExtraPaths = @(
    "C:\flutter\bin",
    "C:\Program Files\Git\cmd",
    "C:\Program Files\CMake\bin"
)
foreach ($path in $machineExtraPaths) {
    if ($machinePath -notlike "*$path*") {
        $machinePath = "$machinePath;$path"
    }
}
[Environment]::SetEnvironmentVariable("Path", $machinePath, "Machine")

$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
$userExtraPaths = @(
    "$env:USERPROFILE\.cargo\bin"
)
foreach ($path in $userExtraPaths) {
    if ($userPath -notlike "*$path*") {
        $userPath = "$userPath;$path"
    }
}
[Environment]::SetEnvironmentVariable("Path", $userPath, "User")
$env:Path = "$machinePath;$userPath;$env:Path"

rustup toolchain install stable
rustup default stable
flutter config --enable-windows-desktop --no-analytics
flutter precache --windows

foreach ($commandName in @("git", "rustc", "cargo", "cmake", "ninja", "flutter")) {
    if (-not (Get-Command -Name $commandName -ErrorAction SilentlyContinue)) {
        throw "Required command not found after provisioning: $commandName"
    }
}

git --version
rustc --version
cargo --version
cmake --version
ninja --version
flutter --version
flutter doctor -v

Write-Information -MessageData "[frb-windows-toolchain] FRB Windows toolchain ready" -InformationAction Continue
