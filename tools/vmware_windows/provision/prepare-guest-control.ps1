Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

Write-Information -MessageData "[frb-windows-guest] Preparing guest command access" -InformationAction Continue

Get-NetConnectionProfile |
    Where-Object -FilterScript { $_.NetworkCategory -eq "Public" } |
    Set-NetConnectionProfile -NetworkCategory Private

Set-Item -Path WSMan:\localhost\Service\Auth\Basic -Value $true
Set-Item -Path WSMan:\localhost\Service\AllowUnencrypted -Value $true
Enable-PSRemoting -Force -SkipNetworkProfileCheck

if (-not (
    Get-WindowsCapability -Online -Name OpenSSH.Server~~~~0.0.1.0 |
        Where-Object -FilterScript { $_.State -eq "Installed" }
)) {
    Add-WindowsCapability -Online -Name OpenSSH.Server~~~~0.0.1.0
}

Set-Service -Name sshd -StartupType Automatic
Start-Service -Name sshd

New-Item -ItemType Directory -Force -Path C:\frb | Out-Null
New-Item -ItemType Directory -Force -Path C:\frb\logs | Out-Null
New-Item -ItemType Directory -Force -Path C:\frb\worktree | Out-Null

Write-Information -MessageData "[frb-windows-guest] Guest command access ready" -InformationAction Continue
