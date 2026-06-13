---
name: frb-vmware-windows-prepare
description: Use when preparing the FRB VMware Fusion Windows 11 Arm base VM.
---

# FRB VMware Windows Base VM Preparation

Use this skill when preparing or explaining the reusable Windows 11 Arm base VM used by `tools/vmware_windows/frb_vmware_windows_env.py`.

## Source Of Truth

Read the checked-in guide first:

```text
tools/vmware_windows/README.md
```

That README owns the detailed Packer, Vagrant, VMware Fusion, proxy, storage, host-installation, daily-use, and troubleshooting instructions. Keep this skill thin so the guide remains the audited source of truth for the Windows VM workflow.

## Required Reminders

- Heavy Windows VM artifacts belong under the per-user `vm_root` configured in
  `~/.config/flutter_rust_bridge/vmware_windows.json` or `FRB_WINDOWS_VM_ROOT`.
- Host-side Windows VM downloads and Packer network operations use `host_proxy_url` or
  `FRB_WINDOWS_HOST_PROXY_URL` only when a local network requires a proxy.
- Durable base image changes belong in the Packer template or PowerShell provision scripts under `tools/vmware_windows/`, not in manual mutations of a promoted base VM.
- For daily Windows VM usage, read the Windows VMware section inside `.claude/skills/frb-dev-env/SKILL.md`.
