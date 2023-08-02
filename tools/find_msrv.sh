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
	rustup install "1.$version" --profile default
	rustup default "1.$version"

	# check if the crate compiles with the version of rust
	if cargo check --all; then
		echo -e "\e[32mSUCCESS: Found compatible version: 1.$version\e[0m"
		return 0
	else
		echo -e "\e[31mINFO: Version 1.$version is not compatible\e[0m"
		return 1
	fi
}

# copy the currently installed toolchain to restore later
current_toolchain=$(rustup show active-toolchain | cut -d ' ' -f 1)

declare -A rust_editions=(
	["2015"]="0"  # -> "1.0.x"
	["2018"]="31" # -> "1.31.x"
	["2021"]="51" # -> "1.51.x"
	# Extend as necessary
)

# check for the declared edition of the crate
edition=$(grep -m 1 "edition" Cargo.toml | cut -d '"' -f 2)

# earliest version to check for compatibility
start_version="${rust_editions[$edition]}"

# pull the latest tag from the rust repo
end_version=$(curl https://api.github.com/repos/rust-lang/rust/releases/latest | grep -m 1 "tag_name" | cut -d '"' -f 4 | cut -d '.' -f 2)

# use binary search to find the earliest compatible version of rust
while [[ "$start_version" -lt "$end_version" ]]; do
	# Display the progress: ruled-out count / total count
	echo -e "\e[36mINFO: Searching between $start_version and $end_version\e[0m"

	# Calculate the midpoint version
	mid_version="$(((start_version + end_version) / 2))"

	# check if the crate is compatible with the version of rust
	if is_compatible "$mid_version"; then
		# If compatible, upversion the end version for the next iteration
		end_version="$mid_version"
	else
		# If not compatible, upversion the start version for the next iteration
		start_version="$((mid_version + 1))"
	fi
done

echo -e "\e[32mSUCCESS: Found Minimum Supported Rust Version: 1.$end_version\e[0m"

echo -e "\e[34mINFO: Writing .msrv ...\e[0m"

echo -e "1.$end_version" >.msrv

echo -e "\e[32mSUCCESS: Successfully wrote .msrv\e[0m"

echo -e "\e[34mINFO: Updating README.md\e[0m"

# find and update:
# https://img.shields.io/badge/stable_msrv-1.6+-green.svg

sed -i.bak "s/stable_msrv-1.[0-9]\+\+/stable_msrv-1.$end_version/g" README.md

echo -e "\e[32mSUCCESS: Successfully updated README.md\e[0m"

echo -e "\e[34mINFO: Updating clippy.toml\e[0m"

# find and update:
# msrv = "1.6"

sed -i.bak "s/msrv = \"1.[0-9]\+\"/msrv = \"1.$end_version\"/g" clippy.toml

echo -e "\e[32mSUCCESS: Successfully updated clippy.toml\e[0m"

echo -e "\e[34mINFO: Updating book/src/msrv.md\e[0m"

# find and update:
# <!-- stable msrv --> `1.45+`

sed -i.bak "s/<!-- stable msrv --> \`1.[0-9]\+\+/<!-- stable msrv --> \`1.$end_version/g" book/src/msrv.md

echo -e "\e[32mSUCCESS: Successfully updated book/src/msrv.md\e[0m"

echo -e "\e[34mINFO: Updating .github/workflows/msrv.yml\e[0m"

# find and update:
# - 1.45 ### stable MSRV (DO NOT EDIT THIS LINE):

sed -i.bak "s/- 1.[0-9]\+ ### stable MSRV (DO NOT EDIT THIS LINE):/- 1.$end_version ### stable MSRV (DO NOT EDIT THIS LINE):/g" .github/workflows/msrv.yml

echo -e "\e[32mSUCCESS: Successfully updated .github/workflows/msrv.yml\e[0m"

echo -e "\e[34mINFO: Restoring toolchain: $current_toolchain\e[0m"

# restore the original toolchain
rustup default "$current_toolchain"
