param(
    [Parameter(Mandatory = $true)]
    [string] $ZipPath,

    [Parameter(Mandatory = $true)]
    [string] $Destination
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

Write-Information "[frb-windows-upload] Expanding uploaded worktree" -InformationAction Continue

if (-not (Test-Path -LiteralPath $ZipPath)) {
    throw "Upload archive does not exist: $ZipPath"
}

$parent = Split-Path -Parent $Destination
if (-not (Test-Path -LiteralPath $parent)) {
    New-Item -ItemType Directory -Path $parent -Force | Out-Null
}

if (Test-Path -LiteralPath $Destination) {
    Remove-Item -LiteralPath $Destination -Recurse -Force
}

New-Item -ItemType Directory -Path $Destination -Force | Out-Null
Expand-Archive -LiteralPath $ZipPath -DestinationPath $Destination -Force
Remove-Item -LiteralPath $ZipPath -Force

Write-Information "[frb-windows-upload] Worktree ready at $Destination" -InformationAction Continue
