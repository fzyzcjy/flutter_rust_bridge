@ECHO off
if defined FRB_INTERNAL_DART (
  set DART_BIN=%FRB_INTERNAL_DART%
) else (
  set DART_BIN=dart
)
where "%DART_BIN%" >nul 2>&1
if errorlevel 1 (
  echo Cannot find Dart executable: %DART_BIN% 1>&2
  echo Set FRB_INTERNAL_DART to a Dart 3.13+ executable when the active Flutter SDK ships an older Dart. 1>&2
  exit /b 1
)
for %%I in ("%DART_BIN%") do set DART_DIR=%%~dpI
if defined DART_DIR set PATH=%DART_DIR%;%PATH%
set SCRIPT_PATH=%~dp0
cd /d "%SCRIPT_PATH%\tools\frb_internal"
"%DART_BIN%" run flutter_rust_bridge_internal %*
