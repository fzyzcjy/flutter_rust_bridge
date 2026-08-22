#!/usr/bin/env bash

set -euo pipefail

input_directory=${1:?Input directory is required}
output_directory=${2:?Output directory is required}
release_tag=${3:?Release tag is required}

targets=(
  x86_64-unknown-linux-gnu
  x86_64-unknown-linux-musl
  i686-unknown-linux-musl
  aarch64-unknown-linux-musl
  arm-unknown-linux-musleabihf
  x86_64-apple-darwin
  aarch64-apple-darwin
  x86_64-pc-windows-msvc
  i686-pc-windows-msvc
  x86_64-unknown-freebsd
)

shopt -s nullglob dotglob
downloaded_directories=("$input_directory"/*)
if [[ ${#downloaded_directories[@]} -ne ${#targets[@]} ]]; then
  printf 'Expected %d artifact directories, found %d\n' "${#targets[@]}" "${#downloaded_directories[@]}" >&2
  exit 1
fi

archives=()
for target in "${targets[@]}"; do
  extension=tgz
  if [[ $target == *-windows-msvc ]]; then
    extension=zip
  fi

  archive_name="flutter_rust_bridge_codegen-${target}-${release_tag}.${extension}"
  artifact_directory="$input_directory/$archive_name"
  archive_path="$artifact_directory/$archive_name"
  artifact_entries=("$artifact_directory"/*)

  if [[ ! -d $artifact_directory ]]; then
    printf 'Missing artifact directory: %s\n' "$artifact_directory" >&2
    exit 1
  fi
  if [[ ${#artifact_entries[@]} -ne 1 || ${artifact_entries[0]} != "$archive_path" || ! -s $archive_path ]]; then
    printf 'Artifact directory must contain exactly one non-empty expected archive: %s\n' "$archive_path" >&2
    exit 1
  fi

  if [[ $extension == zip ]]; then
    unzip -t "$archive_path"
  else
    tar tzf "$archive_path"
  fi
  archives+=("$archive_path")
done

mkdir "$output_directory"
for archive_path in "${archives[@]}"; do
  archive_name=$(basename "$archive_path")
  cp "$archive_path" "$output_directory/$archive_name"
  openssl dgst -sha256 -r "$archive_path" | awk '{print $1}' > "$output_directory/$archive_name.sha256"
done

staged_files=("$output_directory"/*)
expected_staged_file_count=$((${#targets[@]} * 2))
if [[ ${#staged_files[@]} -ne $expected_staged_file_count ]]; then
  printf 'Expected %d staged release files, found %d\n' "$expected_staged_file_count" "${#staged_files[@]}" >&2
  exit 1
fi
