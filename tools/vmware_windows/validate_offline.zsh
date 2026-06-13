#!/usr/bin/env zsh
set -euo pipefail

SCRIPT_DIR="${0:A:h}"
REPO_ROOT="${SCRIPT_DIR:h:h}"

run() {
  echo
  echo "EXEC: $*"
  "$@"
}

run_if_available() {
  local tool="$1"
  shift
  if command -v "${tool}" >/dev/null 2>&1; then
    run "$@"
    return
  fi

  echo
  echo "SKIP: ${tool} is not available on PATH"
}

cd "${REPO_ROOT}"

run python3 -m py_compile tools/vmware_windows/frb_vmware_windows_env.py
run tools/vmware_windows/frb_vmware_windows_env.py self-test
run zsh -f -n tools/vmware_windows/run_packer_build.zsh
run_if_available ruby ruby -c tools/vmware_windows/Vagrantfile
run_if_available packer packer fmt -check tools/vmware_windows/packer
run_if_available node node --check website/sidebars.js
run git diff --check

echo
echo "VMware Windows offline validation passed"
