#!/usr/bin/env bash
set -euo pipefail

branch="$1"
destination="$2"
repo_url="https://gitcode.com/CPF-Flutter/flutter_flutter.git"

for attempt in 1 2 3; do
  rm -rf "$destination"
  if git clone --branch "$branch" "$repo_url" "$destination"; then
    exit 0
  fi

  if [[ "$attempt" -eq 3 ]]; then
    break
  fi

  sleep_seconds=$((attempt * 20))
  echo "flutter-ohos clone attempt $attempt failed; retrying in ${sleep_seconds}s" >&2
  sleep "$sleep_seconds"
done

echo "Failed to clone flutter-ohos branch $branch from $repo_url" >&2
exit 1
