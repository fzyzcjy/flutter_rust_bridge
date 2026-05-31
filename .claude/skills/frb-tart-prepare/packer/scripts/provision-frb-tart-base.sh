#!/bin/zsh
set -euo pipefail

export PATH="/opt/homebrew/bin:/usr/local/bin:$HOME/.cargo/bin:$PATH"
export LANG="en_US.UTF-8"

RUST_VERSION="1.93.1"

log() {
  printf '[frb-tart-base] %s\n' "$*"
}

require_command() {
  local name="$1"
  if ! command -v "$name" >/dev/null 2>&1; then
    printf 'Required command not found in base image: %s\n' "$name" >&2
    exit 1
  fi
}

install_rustup_if_needed() {
  if command -v rustup >/dev/null 2>&1; then
    return
  fi

  log "Installing rustup"
  curl --proto '=https' --tlsv1.2 -fsSL https://sh.rustup.rs | sh -s -- -y --profile minimal --default-toolchain stable
  export PATH="$HOME/.cargo/bin:$PATH"
}

configure_xcode() {
  if [ -d /Applications/Xcode.app/Contents/Developer ]; then
    sudo xcode-select -s /Applications/Xcode.app/Contents/Developer
  fi

  sudo xcodebuild -license accept
  sudo xcodebuild -runFirstLaunch
}

install_flutter_if_needed() {
  if command -v flutter >/dev/null 2>&1; then
    return
  fi

  if [ -x "$HOME/flutter/bin/flutter" ]; then
    export PATH="$HOME/flutter/bin:$PATH"
    return
  fi

  printf 'Flutter was not found in the source Tart image. Use a source image with Flutter preinstalled, or extend this Packer template with a fast local Flutter archive upload.\n' >&2
  exit 1
}

install_cocoapods_if_needed() {
  if command -v pod >/dev/null 2>&1; then
    return
  fi

  require_command brew

  log "Installing CocoaPods"
  HOMEBREW_NO_AUTO_UPDATE=1 HOMEBREW_NO_INSTALL_CLEANUP=1 brew install cocoapods
}

persist_shell_path() {
  local profile="$HOME/.zprofile"
  local marker="# FRB Tart base PATH"

  if [ -f "$profile" ] && grep -F "$marker" "$profile" >/dev/null 2>&1; then
    return
  fi

  {
    printf '\n%s\n' "$marker"
    printf 'export PATH="$HOME/flutter/bin:$HOME/.cargo/bin:/opt/homebrew/bin:/usr/local/bin:$PATH"\n'
    printf 'export LANG="en_US.UTF-8"\n'
  } >> "$profile"
}

log "Checking source image tools"
require_command xcodebuild
require_command xcrun
require_command curl

configure_xcode
install_flutter_if_needed
install_cocoapods_if_needed
install_rustup_if_needed
persist_shell_path

require_command flutter
require_command pod
require_command rustup

log "Installing Rust ${RUST_VERSION} and iOS targets"
rustup toolchain install "$RUST_VERSION" --profile minimal
rustup default "$RUST_VERSION"
rustup target add aarch64-apple-ios-sim
rustup target add x86_64-apple-ios

log "Pre-caching Flutter iOS artifacts"
flutter config --no-analytics
flutter precache --ios

log "Disabling Spotlight indexing inside the VM"
sudo mdutil -a -i off

log "Printing tool versions"
sw_vers
xcodebuild -version
flutter --version
pod --version
rustc --version
cargo --version
rustup target list --installed | grep '^aarch64-apple-ios-sim$'
rustup target list --installed | grep '^x86_64-apple-ios$'

log "Provisioning complete"
