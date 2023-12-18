@ECHO off
set SCRIPT_PATH=%~dp0
cd /d "%SCRIPT_PATH%\tools\frb_internal"
dart run flutter_rust_bridge_internal %*
