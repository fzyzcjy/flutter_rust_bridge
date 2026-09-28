@ECHO off
if defined FRB_INTERNAL_DART (
  set "DART_BIN=%FRB_INTERNAL_DART%"
) else (
  set "DART_BIN=dart"
)
set SCRIPT_PATH=%~dp0
cd /d "%SCRIPT_PATH%\tools\frb_internal"
"%DART_BIN%" run flutter_rust_bridge_internal %*
