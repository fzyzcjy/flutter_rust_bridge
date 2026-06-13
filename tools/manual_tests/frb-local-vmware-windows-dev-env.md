# Local VMware Windows Development Environment Coverage

## Purpose

Verify that a local Apple Silicon macOS host can run FRB Windows validation through VMware Fusion, Vagrant, and a Packer-built Windows 11 Arm base box stored on an external disk.

## Source

- Context: Maintenance check for the local FRB Windows VM workflow.
- Related docs or skills: `.claude/skills/frb-dev-env/SKILL.md`, `.claude/skills/frb-vmware-windows-prepare/SKILL.md`, `.claude/skills/frb-manual-test/SKILL.md`

## When To Run

Run this after changing the VMware Windows helper, Packer template, Vagrantfile, Windows provision scripts, Flutter/Rust/Visual Studio toolchain versions, or Windows manual validation workflow.

## Preconditions

- Repository: `fzyzcjy/flutter_rust_bridge`
- Required checkout state: clean checkout with submodules initialized. Intentional local changes are allowed only if the execution record lists them.
- Required credentials or account state: Windows guest credentials must be supplied through environment variables or secret files. Do not record passwords.
- Required device or simulator state: VMware Fusion Pro, Vagrant, Vagrant VMware Desktop provider, Packer, and a Windows 11 Arm ISO or already imported `frb/windows11-arm64` Vagrant box.
- Required storage state: `vm_root` must be configured through `~/.config/flutter_rust_bridge/vmware_windows.json`, `FRB_WINDOWS_VM_ROOT`, or a command-specific flag; it must exist or be creatable and have at least 100GB free.
- Required network state: if host-side Windows VM downloads and Packer network operations need a proxy, configure `host_proxy_url` or `FRB_WINDOWS_HOST_PROXY_URL`.
- Required checksum state: real Packer builds must use `FRB_WINDOWS_ISO_CHECKSUM=sha256:<digest>` or an `.iso.sha256` file next to the Windows ISO.
- Required VM display state: default daily use starts VMware Fusion with a visible GUI because Fusion 26 on Apple Silicon can power off this Windows Arm guest shortly after boot in `nogui` mode.

## Environment

- OS: Apple Silicon macOS host.
- Flutter: record `flutter --version` inside the Windows VM.
- Dart: record `dart --version` inside the Windows VM.
- Rust: record `rustc --version` and `cargo --version` inside the Windows VM.
- Visual Studio: record that Build Tools includes the VCTools workload, ARM64 C++ tools, and CMake project component.
- Device or simulator: Windows desktop target inside VMware Fusion.
- Browser or external service: not required.

## Preparation

Run preparation commands from the repository root.

```bash
git rev-parse --show-toplevel
git status --short
git submodule status --recursive
tools/vmware_windows/frb_vmware_windows_env.py init-root
tools/vmware_windows/frb_vmware_windows_env.py info
tools/vmware_windows/frb_vmware_windows_env.py check-host
tools/vmware_windows/frb_vmware_windows_env.py self-test
tools/vmware_windows/validate_offline.zsh
```

Confirm the helper reports the expected `storage_root` and, when needed, the expected `host_proxy_url`.

If the base box is not imported yet, build and import it:

```bash
tools/vmware_windows/run_packer_build.zsh \
  --iso /path/to/frb-windows-vmware/downloads/iso/Windows11_Arm64.iso \
  --preflight-only
```

```bash
tools/vmware_windows/run_packer_build.zsh \
  --iso /path/to/frb-windows-vmware/downloads/iso/Windows11_Arm64.iso \
  --add-box
```

The wrapper records a full build log under the configured VM root, applies the configured host proxy when present, regenerates the UEFI remastered autounattend ISO, and stops the active build if `/` or the VM storage root drops below 100GB free.

If the full VMware build is not ready to run yet, first validate only the ISO remastering path:

```bash
tools/vmware_windows/run_packer_build.zsh \
  --iso /path/to/frb-windows-vmware/downloads/iso/Windows11_Arm64.iso \
  --prepare-only
```

## Test Data

- Input files, API examples, account fixtures, or generated assets: checked-in package `frb_example/flutter_via_create`.
- Reset procedure before each run: return to a clean checkout or record intentional local changes in the execution record.

## Steps

1. Start the per-worktree Windows VM.

   ```bash
   tools/vmware_windows/frb_vmware_windows_env.py start
   ```

2. Record Windows and toolchain versions.

   ```bash
   tools/vmware_windows/frb_vmware_windows_env.py exec -- powershell -NoProfile -Command "Get-ComputerInfo | Select-Object WindowsProductName,OsArchitecture,WindowsVersion; flutter --version; dart --version; rustc --version; cargo --version; cmake --version; ninja --version"
   ```

3. Upload the host checkout to the VM-local build path.

   ```bash
   tools/vmware_windows/frb_vmware_windows_env.py upload
   ```

4. Run a focused Windows Flutter native test.

   ```bash
   tools/vmware_windows/frb_vmware_windows_env.py test-flutter-windows --package frb_example/flutter_via_create
   ```

5. Confirm the host checkout did not gain unexpected generated or cache files.

   ```bash
   git status --short
   ```

6. Stop the VM if it is not needed for follow-up debugging.

   ```bash
   tools/vmware_windows/frb_vmware_windows_env.py stop
   ```

## Expected Result

The VMware Windows environment coverage test passes when the helper resolves all heavy paths under the configured VM root, the Windows guest command channel works, the checkout uploads to the VM-local worktree, and the Flutter Windows native test completes successfully.

## Failure Criteria

The test fails if any of the following happens:

- Any long-lived VM bundle, Vagrant box, Packer output, or cache is created on the internal disk unexpectedly.
- VMware Fusion, Vagrant, Packer, or the Vagrant VMware Desktop provider is unavailable after preparation.
- Packer cannot build the base box from the Windows 11 Arm ISO after template/provision fixes are applied.
- Guest command execution through WinRM fails after the VM is booted.
- The focused Windows Flutter native test exits non-zero unexpectedly after toolchain setup is complete.
- `git status --short` shows unexpected local changes after the run.

The test is blocked, not failed, if VMware Fusion or the Windows 11 Arm ISO cannot be installed or downloaded on the host.

## Results To Capture

- Full terminal log for all preparation and test commands.
- Host macOS version, VMware Fusion version, Vagrant version, Packer version, and Vagrant plugin list.
- Resolved helper paths proving external disk storage.
- Windows version, architecture, VM name, box name, Flutter, Dart, Rust, Cargo, CMake, Ninja, and Visual Studio Build Tools evidence.
- Packer output path and Vagrant box name.
- `run_packer_build.zsh` log path if the base box was built during this run.
- Final `git status --short` output.

## Troubleshooting

- If `check-host` reports missing tools, install VMware Fusion Pro, Vagrant, Packer, and the Vagrant VMware Desktop provider.
- If Homebrew reports the VMware Fusion cask is disabled because authentication is required, install VMware Fusion Pro from the authenticated Broadcom/VMware download.
- If the Vagrant cask downloads but fails because the pkg installer needs sudo, install Vagrant through the normal interactive macOS installer flow.
- If the configured external disk path is missing, mount that disk and rerun `init-root`.
- If Vagrant cannot authenticate to WinRM, verify `FRB_WINDOWS_VM_USERNAME`, `FRB_WINDOWS_VM_PASSWORD`, or the helper-generated `credentials/guest-password.txt` under `FRB_WINDOWS_VM_ROOT` without printing the password.
- If the VM powers off shortly after boot in headless mode, use the default GUI startup or unset `FRB_WINDOWS_VM_GUI=false` before rerunning `start`.
- If CMake fails to configure ARM64 Windows with a `VCTargetsPath` or `BaseOutputPath/OutputPath` error, reinstall the Visual Studio Build Tools VCTools workload with `Microsoft.VisualStudio.Component.VC.Tools.ARM64` and `Microsoft.VisualStudio.Component.VC.CMake.Project`, then delete the guest `build\windows\arm64` directory before retrying.
- If build performance is poor, verify the test is running from `C:\frb\worktree`, not directly from `C:\frb-host`.
- If Packer unattended install fails, keep the full log and update the helper-generated external `packer/autounattend/autounattend.xml` path or the PowerShell provision scripts.

## Cleanup

Stop the VM:

```bash
tools/vmware_windows/frb_vmware_windows_env.py stop
git status --short
```

Delete the disposable per-worktree VM only when reclaiming external disk space:

```bash
tools/vmware_windows/frb_vmware_windows_env.py destroy --force
```

Keep the Packer-built base box unless rebuilding the Windows base.

## Future Automation

The helper commands are intended to become automatable on a dedicated Apple Silicon Windows validation host once VMware Fusion, Windows licensing, and base box refresh are managed outside individual PRs.
