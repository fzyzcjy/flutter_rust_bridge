#! /bin/bash

#
# Ensure all necessary dependencies are installed
#

if ! command -v rustup >/dev/null 2>&1; then
	echo -e "\e[34mINFO: rustup is not installed, attempting to install...\e[0m"
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
	# refresh environment to load rustup
	source "$HOME"/.cargo/env
fi

# Determines if a passed version of rust is compatible with the evaluated crate
function is_compatible() {
	local version="$1"

	# use rustup to install the version of rust
	rustup install "nightly-$version" --profile default
	rustup default "nightly-$version"

	# check if the crate compiles with the version of rust
	if cargo check --all; then
		echo -e "\e[32mSUCCESS: Found compatible version: $version\e[0m"
		return 0
	else
		echo -e "\e[31mINFO: Version $version is not compatible\e[0m"
		return 1
	fi
}

# copy the currently installed toolchain to restore later
current_toolchain=$(rustup show active-toolchain | cut -d ' ' -f 1)

# check for the declared edition of the crate
edition=$(grep -m 1 "edition" Cargo.toml | cut -d '"' -f 2)
# earliest date to check for compatibility
start_date="$edition-01-01"
msrv=$(date +%Y-%m-%d)

# Calculate the end date for the binary search
end_date="$msrv"

# use binary search to find the earliest compatible version of rust
while [[ "$start_date" != "$end_date" ]]; do
	# Display the progress: ruled-out count / total count
	echo -e "\e[36mINFO: Searching between nightly releases $start_date and $end_date\e[0m"

	# Calculate the midpoint date
	mid_date=$(date -d "$start_date + $((($(date -d "$end_date" +%s) - $(date -d "$start_date" +%s)) / 2)) seconds" +"%Y-%m-%d")

	# check if the crate is compatible with the version of rust
	if is_compatible "$mid_date"; then
		# If compatible, update the end date for the next iteration
		end_date="$mid_date"
	else
		# If not compatible, update the start date for the next iteration
		start_date=$(date -d "$mid_date + 1 day" +"%Y-%m-%d")
	fi
done

echo -e "\e[32mSUCCESS: Found Minimum Supported Rust Version: nightly-$end_date\e[0m"

echo -e "\e[34mINFO: Writing tools/.nightly_msrv ...\e[0m"

echo -e "nightly-$end_date" >tools/.nightly_msrv

echo -e "\e[32mSUCCESS: Successfully wrote tools/.nightly_msrv\e[0m"

echo -e "\e[34mINFO: Updating README.md\e[0m"

# find and update:
# https://img.shields.io/badge/nightly_msrv-nightly--2021--08--04+-orange.svg

# replace single dashes with double dashes
sed -i.bak "s/nightly_msrv-nightly--[0-9]\{4\}--[0-9]\{2\}--[0-9]\{2\}+-orange.svg/nightly_msrv-nightly--${end_date//-/--}+-orange.svg/g" README.md

echo -e "\e[32mSUCCESS: Successfully updated README.md\e[0m"

echo -e "\e[34mINFO: Updating book/src/msrv.md\e[0m"

# find and update:
# <!-- nightly msrv --> `nightly-2023-56-89+`

sed -i.bak "s/<!-- nightly msrv --> \`nightly-[0-9]\{4\}-[0-9]\{2\}-[0-9]\{2\}+\` |/<!-- nightly msrv --> \`nightly-${end_date}+\` |/g" book/src/msrv.md

echo -e "\e[32mSUCCESS: Successfully updated book/src/msrv.md\e[0m"

echo -e "\e[34mINFO: Updating .github/workflows/msrv.yml\e[0m"

# find and update:
# - nightly-2023-06-28 ### nightly MSRV (DO NOT EDIT THIS LINE):

sed -i.bak "s/- nightly-[0-9]\{4\}-[0-9]\{2\}-[0-9]\{2\} ### nightly MSRV (DO NOT EDIT THIS LINE):/- nightly-${end_date} ### nightly MSRV (DO NOT EDIT THIS LINE):/g" .github/workflows/ci.yml

echo -e "\e[32mSUCCESS: Successfully updated .github/workflows/msrv.yml\e[0m"

echo -e "\e[34mINFO: Restoring toolchain: $current_toolchain\e[0m"

# restore the original toolchain
rustup default "$current_toolchain"
