#!/usr/bin/env zsh
set -euo pipefail

SCRIPT_DIR="${0:A:h}"
REPO_ROOT="${SCRIPT_DIR:h:h}"
HELPER="${SCRIPT_DIR}/frb_vmware_windows_env.py"

json_field() {
  local key="$1"
  /usr/bin/python3 -c 'import json, sys; value = json.load(sys.stdin).get(sys.argv[1]); print("" if value is None else value)' "${key}" <<< "${CONFIG_JSON}"
}

STORAGE_ROOT="${FRB_WINDOWS_VM_ROOT:-}"
ISO_PATH="${FRB_WINDOWS_ISO:-}"
ISO_PATH_WAS_DEFAULT=1
if [[ -n "${FRB_WINDOWS_ISO:-}" ]]; then
  ISO_PATH_WAS_DEFAULT=0
fi
HOST_PROXY_URL="${FRB_WINDOWS_HOST_PROXY_URL:-}"
PACKER_WINRM_HOST="${FRB_WINDOWS_PACKER_WINRM_HOST:-}"
MIN_FREE_GB="${FRB_WINDOWS_MIN_FREE_GB:-100}"
MONITOR_INTERVAL_SECONDS="${FRB_WINDOWS_DISK_MONITOR_INTERVAL_SECONDS:-300}"

GUI_FLAG="--gui"
ON_ERROR="abort"
FORCE_PREPARE_ISO=1
ADD_BOX=0
FORCE_BOX=0
PREPARE_ONLY=0
PREFLIGHT_ONLY=0

usage() {
  cat <<'USAGE'
Usage:
  tools/vmware_windows/run_packer_build.zsh [options]

Options:
  --iso PATH             Windows 11 Arm ISO path.
  --headless             Run Packer without showing the VMware console.
  --on-error VALUE       Packer on-error behavior: abort, cleanup, ask, or run-cleanup-provisioner.
  --no-force-prepare-iso Reuse an existing remastered ISO instead of regenerating it.
  --preflight-only       Run preflight checks and disk-space checks, then exit.
  --prepare-only         Stop after creating the remastered autounattend ISO.
  --add-box              Import the produced Vagrant box after a successful Packer build.
  --force-box            Replace an existing local Vagrant box when used with --add-box.
  -h, --help             Show this help.

Environment:
  FRB_WINDOWS_CONFIG overrides the user config path
  FRB_WINDOWS_VM_ROOT overrides vm_root from the user config
  FRB_WINDOWS_ISO overrides iso_path from the user config
  FRB_WINDOWS_HOST_PROXY_URL overrides host_proxy_url from the user config
  FRB_WINDOWS_MIN_FREE_GB defaults to 100
USAGE
}

require_value() {
  local option="$1"
  local value="${2:-}"
  if [[ -z "${value}" || "${value}" == --* ]]; then
    echo "ERROR: ${option} requires a value." >&2
    usage >&2
    exit 2
  fi
}

while (($# > 0)); do
  case "$1" in
    --iso)
      require_value "$1" "${2:-}"
      ISO_PATH="$2"
      ISO_PATH_WAS_DEFAULT=0
      shift 2
      ;;
    --headless)
      GUI_FLAG=""
      shift
      ;;
    --on-error)
      require_value "$1" "${2:-}"
      ON_ERROR="$2"
      shift 2
      ;;
    --no-force-prepare-iso)
      FORCE_PREPARE_ISO=0
      shift
      ;;
    --prepare-only)
      PREPARE_ONLY=1
      shift
      ;;
    --preflight-only)
      PREFLIGHT_ONLY=1
      shift
      ;;
    --add-box)
      ADD_BOX=1
      shift
      ;;
    --force-box)
      FORCE_BOX=1
      shift
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "Unknown option: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

case "${ON_ERROR}" in
  abort|cleanup|ask|run-cleanup-provisioner)
    ;;
  *)
    echo "ERROR: unsupported --on-error value: ${ON_ERROR}" >&2
    usage >&2
    exit 2
    ;;
esac

CONFIG_JSON="$("${HELPER}" config --json)"
CONFIG_STORAGE_ROOT="$(json_field storage_root)"
CONFIG_ISO_PATH="$(json_field iso_path)"
CONFIG_HOST_PROXY_URL="$(json_field host_proxy_url)"
CONFIG_PACKER_WINRM_HOST="$(json_field packer_winrm_host)"

STORAGE_ROOT="${STORAGE_ROOT:-${CONFIG_STORAGE_ROOT}}"
ISO_PATH="${ISO_PATH:-${CONFIG_ISO_PATH:-${STORAGE_ROOT}/downloads/iso/Windows11_Arm64.iso}}"
HOST_PROXY_URL="${HOST_PROXY_URL:-${CONFIG_HOST_PROXY_URL}}"
PACKER_WINRM_HOST="${PACKER_WINRM_HOST:-${CONFIG_PACKER_WINRM_HOST:-172.16.0.128}}"

LOG_DIR="${STORAGE_ROOT}/logs"
STOP_MARKER="${LOG_DIR}/STOP_LOW_DISK"
PID_FILE="${LOG_DIR}/packer-build-child.pid"
TIMESTAMP="$(/bin/date '+%Y%m%d-%H%M%S')"
LOG_PATH="${LOG_DIR}/packer-build-${TIMESTAMP}.log"

discover_iso_path() {
  if [[ -f "${ISO_PATH}" || "${ISO_PATH_WAS_DEFAULT}" != 1 ]]; then
    return
  fi

  local iso_dir="${STORAGE_ROOT}/downloads/iso"
  local candidates=()
  local candidate
  for candidate in "${iso_dir}"/*.iso(N); do
    if [[ "${candidate:l}" == *autounattend* ]]; then
      continue
    fi
    candidates+=("${candidate}")
  done

  if (( ${#candidates[@]} == 1 )); then
    ISO_PATH="${candidates[1]}"
    return
  fi

  if (( ${#candidates[@]} > 1 )); then
    echo "ERROR: multiple Windows ISO candidates found under ${iso_dir}; pass --iso explicitly." >&2
    printf '  %s\n' "${candidates[@]}" >&2
    exit 2
  fi
}

if [[ "${STORAGE_ROOT}" == /Volumes/* ]]; then
  VOLUME_NAME="${${STORAGE_ROOT#/Volumes/}%%/*}"
  VOLUME_ROOT="/Volumes/${VOLUME_NAME}"
  if ! /sbin/mount | /usr/bin/grep -F " on ${VOLUME_ROOT} " >/dev/null; then
    echo "ERROR: ${VOLUME_ROOT} is not mounted; refusing to create VM files on the internal disk." >&2
    exit 1
  fi
fi

discover_iso_path

mkdir -p "${LOG_DIR}"
rm -f "${STOP_MARKER}" "${PID_FILE}"
exec > >(tee -a "${LOG_PATH}") 2>&1

export FRB_WINDOWS_VM_ROOT="${STORAGE_ROOT}"
export FRB_WINDOWS_ISO="${ISO_PATH}"
export NO_PROXY="localhost,127.0.0.1,::1,${PACKER_WINRM_HOST}"
export no_proxy="localhost,127.0.0.1,::1,${PACKER_WINRM_HOST}"
if [[ -n "${HOST_PROXY_URL}" ]]; then
  export FRB_WINDOWS_HOST_PROXY_URL="${HOST_PROXY_URL}"
  export HTTP_PROXY="${HOST_PROXY_URL}"
  export HTTPS_PROXY="${HOST_PROXY_URL}"
  export ALL_PROXY="${HOST_PROXY_URL}"
  export http_proxy="${HOST_PROXY_URL}"
  export https_proxy="${HOST_PROXY_URL}"
  export all_proxy="${HOST_PROXY_URL}"
fi

require_executable() {
  local name="$1"
  if ! command -v "${name}" >/dev/null 2>&1; then
    echo "ERROR: required executable not found on PATH: ${name}" >&2
    return 1
  fi
}

preflight() {
  echo
  echo "Preflight checks"

  if [[ ! -f "${ISO_PATH}" ]]; then
    echo "ERROR: Windows ISO does not exist: ${ISO_PATH}" >&2
    return 1
  fi

  require_executable hdiutil
  require_executable xorriso
  require_executable packer
  require_executable vagrant

  if [[ ! -x "/Applications/VMware Fusion.app/Contents/Library/vmrun" ]] && ! command -v vmrun >/dev/null 2>&1; then
    echo "ERROR: VMware vmrun not found. Expected /Applications/VMware Fusion.app/Contents/Library/vmrun" >&2
    return 1
  fi

  local output_dir="${STORAGE_ROOT}/packer/output"
  if [[ -d "${output_dir}" ]] && [[ -n "$(/bin/ls -A "${output_dir}")" ]]; then
    echo "ERROR: Packer output directory is not empty: ${output_dir}" >&2
    echo "Move or archive it before rebuilding the base box." >&2
    return 1
  fi

  echo "Preflight checks passed"
}

free_gb() {
  /bin/df -g "$1" | /usr/bin/awk 'NR == 2 { print $4 }'
}

check_free_space_once() {
  local path="$1"
  local free
  free="$(free_gb "${path}")"
  echo "DISK: ${path} has ${free}GB free; minimum is ${MIN_FREE_GB}GB"
  if (( free < MIN_FREE_GB )); then
    echo "ERROR: ${path} has only ${free}GB free, below ${MIN_FREE_GB}GB" >&2
    return 1
  fi
}

disk_monitor() {
  while true; do
    if ! check_free_space_once "/" || ! check_free_space_once "${STORAGE_ROOT}"; then
      /usr/bin/touch "${STOP_MARKER}"
      if [[ -s "${PID_FILE}" ]]; then
        local child_pid
        child_pid="$(< "${PID_FILE}")"
        echo "ERROR: stopping child process ${child_pid} because disk space is below threshold" >&2
        /bin/kill -TERM "${child_pid}" 2>/dev/null || true
      fi
      exit 99
    fi
    /bin/sleep "${MONITOR_INTERVAL_SECONDS}"
  done
}

run_step() {
  echo
  echo "EXEC: $*"
  "$@" &
  local child_pid=$!
  echo "${child_pid}" > "${PID_FILE}"
  set +e
  wait "${child_pid}"
  local exit_code=$?
  set -e
  : > "${PID_FILE}"
  if [[ -e "${STOP_MARKER}" ]]; then
    echo "ERROR: stopped because ${STOP_MARKER} exists" >&2
    exit 99
  fi
  return "${exit_code}"
}

cleanup() {
  if [[ -n "${MONITOR_PID:-}" ]]; then
    /bin/kill "${MONITOR_PID}" 2>/dev/null || true
  fi
  rm -f "${PID_FILE}"
}
trap cleanup EXIT INT TERM

echo "Repo: ${REPO_ROOT}"
echo "Storage root: ${STORAGE_ROOT}"
echo "ISO: ${ISO_PATH}"
if [[ -n "${HOST_PROXY_URL}" ]]; then
  echo "Proxy: ${HOST_PROXY_URL}"
else
  echo "Proxy: <none>"
fi
echo "Log: ${LOG_PATH}"

preflight
check_free_space_once "/"
check_free_space_once "${STORAGE_ROOT}"

if (( PREFLIGHT_ONLY )); then
  echo
  echo "Preflight-only checks passed. Full log: ${LOG_PATH}"
  exit 0
fi

disk_monitor &
MONITOR_PID=$!

cd "${REPO_ROOT}"

run_step "${HELPER}" init-root
run_step "${HELPER}" check-host
run_step "${HELPER}" packer-init

prepare_iso_args=("${HELPER}" prepare-iso --iso "${ISO_PATH}")
if (( FORCE_PREPARE_ISO )); then
  prepare_iso_args+=(--force)
fi
run_step "${prepare_iso_args[@]}"

if (( PREPARE_ONLY )); then
  echo
  echo "Done after prepare-iso because --prepare-only was set. Full log: ${LOG_PATH}"
  exit 0
fi

packer_args=("${HELPER}" packer-build --iso "${ISO_PATH}" --on-error "${ON_ERROR}")
if [[ -n "${GUI_FLAG}" ]]; then
  packer_args+=("${GUI_FLAG}")
fi
run_step "${packer_args[@]}"

if (( ADD_BOX )); then
  box_args=("${HELPER}" vagrant-box-add --box-file "${STORAGE_ROOT}/packer/boxes/frb-windows11-arm64.box")
  if (( FORCE_BOX )); then
    box_args+=(--force)
  fi
  run_step "${box_args[@]}"
fi

echo
echo "Done. Full log: ${LOG_PATH}"
