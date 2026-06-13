# VMware Windows local development environment

This directory contains the VMware Fusion + Vagrant + Packer workflow for local Windows 11 Arm
validation of `flutter_rust_bridge`.

Use this path only for Windows desktop validation. Keep using Docker for Linux/headless checks, Tart
for macOS/iOS checks, and the host Android Emulator workflow for Android runtime checks.

Configure a local storage root before creating or building VMs. The helper reads:

1. CLI flags, where available.
2. Environment variables such as `FRB_WINDOWS_VM_ROOT`.
3. The user config file, defaulting to `~/.config/flutter_rust_bridge/vmware_windows.json`.

Example config:

```json
{
  "vm_root": "/path/to/frb-windows-vmware",
  "host_proxy_url": "http://localhost:PORT",
  "iso_path": "/path/to/frb-windows-vmware/downloads/iso/Windows11_Arm64.iso",
  "iso_sha256": "sha256:<digest>",
  "vm_gui": true
}
```

Do not store long-lived Windows VM bundles or boxes on the internal disk. When the storage root is
under `/Volumes/<name>/`, the helper refuses
to create directories unless that volume is actually mounted, preventing accidental writes to the
internal disk if an external drive is absent.

If host-side Windows VM downloads and Packer network operations need a proxy, set:

```bash
export FRB_WINDOWS_HOST_PROXY_URL=http://localhost:PORT
```

Leave `host_proxy_url` unset when direct network access is preferred.

## Model

The reusable base is built by Packer from a Windows 11 Arm ISO and then consumed by Vagrant on top
of VMware Fusion:

```text
Windows 11 Arm ISO
  -> packer build ... frb-windows11-arm64.box
  -> vagrant box add frb/windows11-arm64 <box>
  -> vagrant up per-worktree VM under the configured VM root
```

The base box is immutable after promotion. Do not manually install Flutter, Rust, Visual Studio
Build Tools, or other dependencies into the promoted base and then forget to update Packer. Manual
experiments are allowed only as discovery; every durable base change must be reflected in the Packer
template or provision scripts.

Use `tools/manual_tests/frb-local-vmware-windows-dev-env.md` for audit-ready local validation and
execution records.

## Quick Start

```bash
tools/vmware_windows/frb_vmware_windows_env.py init-root
tools/vmware_windows/frb_vmware_windows_env.py check-host
tools/vmware_windows/frb_vmware_windows_env.py self-test
tools/vmware_windows/frb_vmware_windows_env.py packer-init
tools/vmware_windows/frb_vmware_windows_env.py prepare-iso \
  --iso /path/to/frb-windows-vmware/downloads/iso/Windows11_Arm64.iso
tools/vmware_windows/frb_vmware_windows_env.py packer-build
tools/vmware_windows/frb_vmware_windows_env.py vagrant-box-add \
  --box-file /path/to/frb-windows-vmware/packer/boxes/frb-windows11-arm64.box
tools/vmware_windows/frb_vmware_windows_env.py start
tools/vmware_windows/frb_vmware_windows_env.py upload
tools/vmware_windows/frb_vmware_windows_env.py test-flutter-windows --package frb_example/flutter_via_create
```

For the initial base-image build on a developer Mac, prefer the Terminal wrapper:

```bash
tools/vmware_windows/run_packer_build.zsh \
  --iso /path/to/frb-windows-vmware/downloads/iso/Windows11_Arm64.iso \
  --add-box
```

The wrapper uses the same helper, applies the configured host proxy when present, writes a full log
under the configured VM root, shows the VMware console by default, and stops the active build if `/`
or the VM storage root drops below 100GB free.
When a host proxy is configured, it also adds the Packer WinRM guest host to `NO_PROXY` so local VM
control traffic does not get sent through the host download proxy.

Before starting the long VMware build, the wrapper checks that the ISO exists, `hdiutil`, `xorriso`,
Packer, Vagrant, and VMware `vmrun` are available, and the Packer output directory is not already
occupied. Use `--preflight-only` to run only those wrapper checks without remastering an ISO,
starting Packer, or booting VMware.

## Host Installation

`tools/vmware_windows/frb_vmware_windows_env.py check-host` verifies VMware `vmrun`, Vagrant, and
Packer. The helper itself uses only the Python standard library, so it does not need `uv`, `pip`, or
a first-run Python package download. Packer can be installed with Homebrew. VMware Fusion Pro may
require an authenticated Broadcom/VMware download, and Vagrant's macOS pkg may require an
interactive sudo password. Install those tools through their normal macOS flows when non-interactive
Homebrew installation is blocked.

If your network requires a proxy for host-side downloads, configure it explicitly before preparing
this VM workflow:

```bash
export HTTP_PROXY=http://localhost:PORT
export HTTPS_PROXY=http://localhost:PORT
export http_proxy=http://localhost:PORT
export https_proxy=http://localhost:PORT
```

If using Homebrew, keep auto-update disabled:

```bash
HOMEBREW_NO_AUTO_UPDATE=1 brew install --cask vmware-fusion
HOMEBREW_NO_AUTO_UPDATE=1 brew install --cask vagrant
HOMEBREW_NO_AUTO_UPDATE=1 brew install --cask vagrant-vmware-utility
HOMEBREW_NO_AUTO_UPDATE=1 brew install packer
HOMEBREW_NO_AUTO_UPDATE=1 brew install xorriso
vagrant plugin install vagrant-vmware-desktop
```

In practice, these host installers can require human action:

- `vmware-fusion` may be disabled in Homebrew because Broadcom/VMware requires an authenticated
  download. Download VMware Fusion Pro manually from the VMware/Broadcom portal, install `VMware
  Fusion.app`, then verify `/Applications/VMware Fusion.app/Contents/Library/vmrun` exists.
- `vagrant` is a macOS `.pkg` cask. A non-interactive agent shell can download it, but
  installation may fail at `/usr/sbin/installer` because `sudo` needs an administrator password.
  Install it manually with the macOS installer, or run the Homebrew cask command yourself from an
  interactive terminal.
- `vagrant-vmware-utility` is also a macOS `.pkg` cask and can require the same interactive
  administrator installation path.
- `packer` can be installed non-interactively with Homebrew and should appear as
  `/opt/homebrew/bin/packer`.
- `xorriso` is required to remaster the Windows ISO so `Autounattend.xml` is available from the
  root of the boot media. Install it with `HOMEBREW_NO_AUTO_UPDATE=1 brew install xorriso`.
- Vagrant's VMware provider requires both the Vagrant VMware Utility and the
  `vagrant-vmware-desktop` plugin. Install the utility with
  `brew install --cask vagrant-vmware-utility`, then install the plugin with Vagrant.
- VMware Fusion 26 provides the Arm64 Windows drivers used during Packer setup at
  `/Applications/VMware Fusion.app/Contents/Library/isoimages/arm64/drivers-arm64.zip`. The helper
  injects those drivers into the remastered ISO and installs `vmxnet3.inf` during Windows setup so
  the guest can obtain a NAT address before Packer waits for WinRM. Set
  `FRB_VMWARE_FUSION_ARM64_DRIVERS_ZIP` only if Fusion stores the archive somewhere else.

After installing VMware Fusion and Vagrant, install the VMware Vagrant provider utility and plugin:

```bash
HOMEBREW_NO_AUTO_UPDATE=1 brew install --cask vagrant-vmware-utility
vagrant plugin install vagrant-vmware-desktop
```

Then re-run:

```bash
tools/vmware_windows/frb_vmware_windows_env.py check-host
```

Expected result:

```json
{
  "missing": [],
  "tools": {
    "packer": "/opt/homebrew/bin/packer",
    "vagrant": "/opt/vagrant/bin/vagrant",
    "vmrun": "/Applications/VMware Fusion.app/Contents/Library/vmrun"
  }
}
```

Validate the host without starting a VM:

```bash
tools/vmware_windows/frb_vmware_windows_env.py check-host
tools/vmware_windows/frb_vmware_windows_env.py self-test
tools/vmware_windows/validate_offline.zsh
```

`self-test` is a dependency-free helper check. It does not start VMware, Vagrant, Packer, or touch
the external VM root; it only validates local helper logic such as path handling, checksum sidecars,
and guest command argument normalization.

`validate_offline.zsh` is the local no-VM regression check for this directory. It checks Python
syntax and helper self-tests, the Packer build wrapper syntax, Vagrantfile syntax when Ruby is
available, Packer formatting when Packer is available, website sidebar JavaScript syntax when Node
is available, and whitespace errors.

## Windows ISO

Download the Windows 11 Arm ISO from Microsoft's official Arm64 download page:

```text
https://www.microsoft.com/software-download/windows11arm64
```

The page generates ISO links dynamically and those links expire. Choose `Windows 11 (multi-edition
ISO for Arm64)`, then choose the language you want to install. For the default English base VM,
choose `English`.

Place the ISO under:

```text
/path/to/frb-windows-vmware/downloads/iso/Windows11_Arm64.iso
```

Set `FRB_WINDOWS_ISO` if using another filename. Do not commit ISO files or generated boxes. If
`Windows11_Arm64.iso` is absent and exactly one non-`autounattend` `.iso` exists in the ISO
directory, the helper and wrapper use that source ISO automatically.

Provide a SHA-256 checksum before a real Packer build. Either set:

```bash
export FRB_WINDOWS_ISO_CHECKSUM=sha256:<digest>
```

or place the digest in:

```text
/path/to/frb-windows-vmware/downloads/iso/Windows11_Arm64.iso.sha256
```

For the current Microsoft Windows 11 Arm64 English ISO, the official verify table lists this
SHA-256:

```text
638AA2C88E94385B00F4F178D071E3DF0B7D9E335577A83BD533B7F2EB65ADF0
```

Write the checksum file as either the bare digest or `sha256:<digest>`:

```bash
printf '%s\n' '638AA2C88E94385B00F4F178D071E3DF0B7D9E335577A83BD533B7F2EB65ADF0' \
  > /path/to/frb-windows-vmware/downloads/iso/Windows11_Arm64.iso.sha256
```

`packer-build --validate` uses a placeholder checksum only to validate template structure. A real
`packer-build` fails fast until a checksum is provided.

## Build The Base Box With Packer

Initialize and build from the checked-in template:

```bash
tools/vmware_windows/frb_vmware_windows_env.py packer-init
tools/vmware_windows/frb_vmware_windows_env.py prepare-iso
tools/vmware_windows/frb_vmware_windows_env.py packer-build
```

The helper sets `PACKER_CACHE_DIR` under the external root and passes explicit variables for the
ISO, checksum, generated unattended-install file, output directory, and provision scripts. By
default, `packer-build` first remasters the source Windows ISO with `hdiutil` and `xorriso` so
`Autounattend.xml` is present at the root of a UEFI-bootable install ISO; this matches Windows
Setup's most reliable unattended-install discovery path under VMware Fusion on Apple Silicon. The
template also exposes `Autounattend.xml` through both Packer floppy media and a secondary
`AUTOUNATTEND` CD as a fallback. The template provisions guest command access, OpenSSH, PowerShell
tooling, Rust, Flutter, CMake/Ninja, and Visual Studio Build Tools.

Packer connects to the freshly installed guest over VMware Fusion NAT WinRM. The helper passes
`FRB_WINDOWS_PACKER_WINRM_HOST` to the template, defaulting to `172.16.0.128`, which is the first
lease observed for the generated Packer VM on VMware Fusion NAT. Override this value if another VM
already occupies that address or if your Fusion NAT subnet assigns a different first lease:

```bash
export FRB_WINDOWS_PACKER_WINRM_HOST=172.16.0.128
```

When a host proxy is configured for downloads, the helper and wrapper add this WinRM host to
`NO_PROXY` and `no_proxy`. Without that bypass, Packer's WinRM HTTP requests can be routed through
the proxy and fail with `http response error: 502 - invalid content type`.

The remastered ISO is stored next to the source ISO:

```text
/path/to/frb-windows-vmware/downloads/iso/<source>.frb-autounattend-uefi.iso
```

Use `prepare-iso --force` only when intentionally regenerating the remastered ISO after changing
`Autounattend.xml` generation. Use `packer-build --no-remaster-iso` only for low-level
troubleshooting of the original Microsoft ISO plus Packer's separate autounattend media.

Packer requires the output directory to be absent before a build starts. The helper removes an empty
stale output directory automatically, but it stops if that directory contains files so an existing
box is not overwritten accidentally.

For unattended first-run builds from an ordinary macOS Terminal, use:

```bash
tools/vmware_windows/run_packer_build.zsh \
  --iso /path/to/frb-windows-vmware/downloads/iso/Windows11_Arm64.iso \
  --add-box
```

Useful variants:

```bash
tools/vmware_windows/run_packer_build.zsh --iso /path/to/Windows11_Arm64.iso
tools/vmware_windows/run_packer_build.zsh --iso /path/to/Windows11_Arm64.iso --preflight-only
tools/vmware_windows/run_packer_build.zsh --iso /path/to/Windows11_Arm64.iso --prepare-only
tools/vmware_windows/run_packer_build.zsh --iso /path/to/Windows11_Arm64.iso --headless
tools/vmware_windows/run_packer_build.zsh --iso /path/to/Windows11_Arm64.iso --no-force-prepare-iso
tools/vmware_windows/run_packer_build.zsh --iso /path/to/Windows11_Arm64.iso --add-box --force-box
```

The wrapper is intentionally thin: it calls `init-root`, `check-host`, `packer-init`, `prepare-iso`,
`packer-build`, and optionally `vagrant-box-add`. It exists to keep the proxy, log path, external
storage root, preflight checks, and disk-space guard consistent during the long VMware build. Use
`--preflight-only` to validate host readiness without starting the build; use `--prepare-only` to
validate ISO remastering before starting VMware.

If the first unattended install fails, keep the full Packer log under `logs/`, record the exact
failure in the execution artifact, fix the template or scripts, and rerun. Do not fix the base by
booting it and manually installing tools without updating the source files.

After Packer produces a box, import it with:

```bash
tools/vmware_windows/frb_vmware_windows_env.py vagrant-box-add \
  --box-file /path/to/frb-windows-vmware/packer/boxes/frb-windows11-arm64.box
```

The default box name is `frb/windows11-arm64`. Override with `FRB_WINDOWS_VAGRANT_BOX` only when
intentionally testing another base.

The base VM installs Flutter 3.44.1, Dart 3.12.1, Rust stable, Git, CMake, Ninja, and Visual Studio
Build Tools with the C++ workload. Keep this list synchronized with
`provision/install-frb-toolchain.ps1` when the repository raises its Windows toolchain floor.

## Daily Use

Run from the repository root:

```bash
tools/vmware_windows/frb_vmware_windows_env.py info
tools/vmware_windows/frb_vmware_windows_env.py start
tools/vmware_windows/frb_vmware_windows_env.py upload
tools/vmware_windows/frb_vmware_windows_env.py exec -- powershell -NoProfile \
  -Command "cd C:\frb\worktree; flutter --version"
tools/vmware_windows/frb_vmware_windows_env.py stop
```

The Vagrantfile starts VMware Fusion with a visible GUI by default because VMware Fusion 26 on Apple
Silicon can power off this Windows Arm guest shortly after boot in `nogui` mode. Set
`FRB_WINDOWS_VM_GUI=false` only when explicitly revalidating headless startup on a newer Fusion or
provider combination.

`upload` creates a zip archive of the host checkout, uploads it through Vagrant's WinRM
communicator, and expands it into a VM-local directory before heavy builds. Do not compile from a
VMware shared folder because shared folders are slower for many small files, can distort build
behavior, and are sensitive to Vagrant VMware provider compatibility.

After the VM is running and code is uploaded:

```bash
tools/vmware_windows/frb_vmware_windows_env.py test-flutter-windows \
  --package frb_example/flutter_via_create
```

## Credentials

Do not put guest passwords in the repository. Use one of:

```text
FRB_WINDOWS_VM_USERNAME
FRB_WINDOWS_VM_PASSWORD
~/.secrets/FRB_WINDOWS_VM_PASSWORD
```

When a password is required, source it in the shell or pass it through the environment. Do not write
it into command logs.

Password resolution order is `FRB_WINDOWS_VM_PASSWORD`, then `~/.secrets/FRB_WINDOWS_VM_PASSWORD`,
then the helper-generated fallback file.

If `FRB_WINDOWS_VM_PASSWORD` is unset, the helper generates a local per-root fallback password at:

```text
/path/to/frb-windows-vmware/credentials/guest-password.txt
```

Treat that file as local machine state. It is not part of the repository, and the helper passes it
to both Packer and Vagrant through environment variables.

## Cleanup

Stop the per-worktree VM when it is not needed:

```bash
tools/vmware_windows/frb_vmware_windows_env.py stop
```

Delete disposable per-worktree VMs only when intentionally reclaiming space:

```bash
tools/vmware_windows/frb_vmware_windows_env.py destroy
```

Keep the Packer-built base box and external caches unless rebuilding the Windows base.

## Layout

```text
tools/vmware_windows/
├── Vagrantfile
├── frb_vmware_windows_env.py
├── run_packer_build.zsh
├── packer/
│   ├── windows11-arm64.pkr.hcl
└── provision/
    ├── expand-uploaded-worktree.ps1
    ├── install-frb-toolchain.ps1
    └── prepare-guest-control.ps1
```

The checked-in scripts are source of truth. Manual changes inside a Windows base VM should be
treated as experiments and then translated back into Packer or provision scripts.

## Troubleshooting

### VMware Cancels VM Startup

If `packer-build` reaches `Powering on virtual machine...` and then fails with:

```text
error starting virtual machine: ... Error: The operation was canceled
```

open VMware Fusion once from `/Applications`, finish any first-run prompts, accept the license, and
approve any macOS privacy, network, or helper permissions it requests. Then verify the command-line
bridge still works:

```bash
/Applications/VMware\ Fusion.app/Contents/Library/vmrun -T fusion list
launchctl print system/com.vagrant.vagrant-vmware-utility
```

After those commands work without permission dialogs, rerun:

```bash
tools/vmware_windows/frb_vmware_windows_env.py packer-build \
  --iso /path/to/frb-windows-vmware/downloads/iso/Windows11_Arm64.iso
```

If the VM still cancels immediately in headless mode, run once with `--gui` so VMware Fusion can
show any remaining prompt:

```bash
tools/vmware_windows/frb_vmware_windows_env.py packer-build --gui \
  --iso /path/to/frb-windows-vmware/downloads/iso/Windows11_Arm64.iso
```

If VMware Fusion reports that the `lsilogic` device type for `scsi0` is not supported, keep the
Packer template on the checked-in NVMe disk adapter and `pvscsi` VMX override. To preserve the
failed VM directory and inspect `vmware.log`, rerun with:

```bash
tools/vmware_windows/frb_vmware_windows_env.py packer-build --gui --on-error=abort \
  --iso /path/to/frb-windows-vmware/downloads/iso/Windows11_Arm64.iso
```

If the VM reaches UEFI Boot Manager instead of Windows Setup, the Packer boot command must select
the SATA CDROM entry and send boot keys during the Windows ISO's short prompt window before WinRM
can ever become available. Keep the checked-in boot command unless a future VMware Fusion release
changes the Boot Manager entry order.
