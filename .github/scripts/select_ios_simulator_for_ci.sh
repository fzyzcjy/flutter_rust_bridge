#!/usr/bin/env bash
set -euo pipefail

requested_device="${1:?requested device is required}"

devices="$(xcrun xctrace list devices)"
printf '%s\n' "$devices" >&2

selected_line="$(
  awk -v requested="$requested_device" '
    /^[[:space:]]*iPhone .* Simulator \([^)]+\) \([A-F0-9-]+\)$/ {
      if ($0 ~ "^" requested " \\(") {
        exact = $0
      }
      if ($0 ~ /^[[:space:]]*iPhone .* Pro Max Simulator \(/) {
        pro_max = $0
      }
      if (first == "") {
        first = $0
      }
    }
    END {
      if (exact != "") {
        print exact
      } else if (pro_max != "") {
        print pro_max
      } else {
        print first
      }
    }
  ' <<< "$devices"
)"

if [[ -z "$selected_line" ]]; then
  echo "No iPhone simulator found for requested device: $requested_device" >&2
  exit 1
fi

echo "Selected simulator: $selected_line" >&2
awk '{print $NF}' <<< "$selected_line" | tr -d '()'
