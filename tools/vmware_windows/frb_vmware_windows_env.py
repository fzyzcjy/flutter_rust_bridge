#!/usr/bin/env python3
from __future__ import annotations

import argparse
import base64
import hashlib
import html
import json
import os
import plistlib
import secrets
import shlex
import shutil
# This helper intentionally runs fixed host tools with shell=False.
import subprocess  # nosec B404
import sys
import tempfile
import zipfile
from collections.abc import Callable
from dataclasses import dataclass
from pathlib import Path

DEFAULT_CONFIG_PATH = Path("~/.config/flutter_rust_bridge/vmware_windows.json")
DEFAULT_BOX_NAME = "frb/windows11-arm64"
DEFAULT_VM_USERNAME = "frb"
DEFAULT_MEMORY_MB = 16384
DEFAULT_CPUS = 6
DEFAULT_MIN_FREE_GB = 100
DEFAULT_COMMAND_TIMEOUT_SECONDS = 14_400
DEFAULT_PACKER_WINRM_HOST = "172.16.0.128"
DEFAULT_PASSWORD_SECRET_PATH = Path("~/.secrets/FRB_WINDOWS_VM_PASSWORD")
DEFAULT_VMWARE_FUSION_ARM64_DRIVERS_ZIP = Path(
    "/Applications/VMware Fusion.app/Contents/Library/isoimages/arm64/drivers-arm64.zip"
)
VALIDATE_ONLY_ISO_CHECKSUM = "sha256:0000000000000000000000000000000000000000000000000000000000000000"
GUEST_WORKTREE_ROOT = "C:\\frb\\worktree"
WINDOWS_ISO_VOLUME_ID = "CCCOMA_A64FRE_EN-US_DV9"
VMWARE_DRIVERS_ISO_DIR = "frb-vmware-drivers"
WINDOWS_BOOTSTRAP_SCRIPT_NAME = "frb-bootstrap.ps1"
VMXNET3_ARM64_INF_RELATIVE_PATH = Path("vmxnet3") / "Win10_1709" / "ARM64" / "vmxnet3.inf"
VMXNET3_ARM64_INF_WINDOWS_PATH = "\\".join(
    [VMWARE_DRIVERS_ISO_DIR, *VMXNET3_ARM64_INF_RELATIVE_PATH.parts]
)
EXCLUDED_UPLOAD_DIRS = [
    ".dart_tool",
    ".git",
    ".idea",
    ".vscode",
    "build",
    "target",
]


class CommandError(RuntimeError):
    pass


def config_path() -> Path:
    value = os.environ.get("FRB_WINDOWS_CONFIG")
    if value:
        return Path(value).expanduser().resolve()

    return DEFAULT_CONFIG_PATH.expanduser()


def load_user_config() -> dict[str, object]:
    path = config_path()
    if not path.exists():
        return {}

    try:
        data = json.loads(path.read_text())
    except json.JSONDecodeError as error:
        raise CommandError(f"Invalid VMware Windows config JSON at {path}: {error}") from error

    if not isinstance(data, dict):
        raise CommandError(f"VMware Windows config must be a JSON object: {path}")

    return data


def config_string(config: dict[str, object], key: str) -> str | None:
    value = config.get(key)
    if value is None:
        return None
    if not isinstance(value, str):
        raise CommandError(f"Config key {key!r} must be a string")
    if not value:
        return None
    return value


def config_bool(config: dict[str, object], key: str) -> bool | None:
    value = config.get(key)
    if value is None:
        return None
    if not isinstance(value, bool):
        raise CommandError(f"Config key {key!r} must be a boolean")
    return value


def env_or_config(config: dict[str, object], *, env_name: str, config_key: str) -> str | None:
    value = os.environ.get(env_name)
    if value:
        return value
    return config_string(config, config_key)


def resolve_storage_root_config(
    *,
    storage_root: Path | None = None,
    config: dict[str, object] | None = None,
) -> Path:
    if storage_root is not None:
        return storage_root.expanduser().resolve()

    loaded_config = load_user_config() if config is None else config
    value = env_or_config(
        loaded_config,
        env_name="FRB_WINDOWS_VM_ROOT",
        config_key="vm_root",
    )
    if value:
        return Path(value).expanduser().resolve()

    raise CommandError(
        "Windows VM storage root is not configured. Set FRB_WINDOWS_VM_ROOT "
        f"or add vm_root to {config_path()}."
    )


@dataclass(frozen=True)
class WindowsVmContext:
    worktree_root: Path
    storage_root: Path
    worktree_digest: str
    vagrant_project_dir: Path
    vmware_worktree_dir: Path
    packer_dir: Path
    packer_output_dir: Path
    packer_box_dir: Path
    packer_box_path: Path
    packer_cache_dir: Path
    packer_autounattend_path: Path
    packer_drivers_dir: Path
    logs_dir: Path
    downloads_iso_dir: Path
    iso_build_dir: Path
    shared_dir: Path
    box_name: str
    vm_username: str


def exec_command(
    cmd: list[str],
    *,
    cwd: Path | None = None,
    capture_output: bool = False,
    env: dict[str, str] | None = None,
    timeout_seconds: int = DEFAULT_COMMAND_TIMEOUT_SECONDS,
) -> str | None:
    print(f"EXEC: {' '.join(shlex.quote(part) for part in cmd)}", file=sys.stderr, flush=True)

    try:
        # cmd is a list and shell=False; callers are local tool wrappers.
        result = subprocess.run(  # nosec B603
            cmd,
            cwd=str(cwd) if cwd is not None else None,
            check=True,
            capture_output=capture_output,
            text=capture_output,
            env=env,
            timeout=timeout_seconds,
        )
    except subprocess.CalledProcessError as error:
        if capture_output:
            print(
                f"FAILED: stdout={error.stdout} stderr={error.stderr}",
                file=sys.stderr,
                flush=True,
            )
        raise CommandError(f"Command failed: {' '.join(cmd)}") from error
    except subprocess.TimeoutExpired as error:
        raise CommandError(
            f"Command timed out after {timeout_seconds}s: {' '.join(cmd)}"
        ) from error

    if capture_output:
        return result.stdout.strip()

    return None


def resolve_worktree_root(worktree_root: Path | None) -> Path:
    if worktree_root is not None:
        return worktree_root.expanduser().resolve()

    output = exec_command(["git", "rev-parse", "--show-toplevel"], capture_output=True)
    if output is None:
        raise CommandError("Unable to resolve git repository root")

    return Path(output).resolve()


def resolve_storage_root(storage_root: Path | None) -> Path:
    return resolve_storage_root_config(storage_root=storage_root)


def worktree_digest_for_path(worktree_root: Path) -> str:
    return hashlib.sha256(str(worktree_root).encode()).hexdigest()[:12]


def build_context(
    *,
    worktree_root: Path | None = None,
    storage_root: Path | None = None,
) -> WindowsVmContext:
    config = load_user_config()
    resolved_worktree_root = resolve_worktree_root(worktree_root)
    resolved_storage_root = resolve_storage_root_config(
        storage_root=storage_root,
        config=config,
    )
    digest = worktree_digest_for_path(resolved_worktree_root)

    return WindowsVmContext(
        worktree_root=resolved_worktree_root,
        storage_root=resolved_storage_root,
        worktree_digest=digest,
        vagrant_project_dir=resolved_storage_root / "vagrant" / "projects" / digest,
        vmware_worktree_dir=resolved_storage_root / "vmware" / "worktrees" / digest,
        packer_dir=Path(__file__).resolve().parent / "packer",
        packer_output_dir=resolved_storage_root / "packer" / "output",
        packer_box_dir=resolved_storage_root / "packer" / "boxes",
        packer_box_path=resolved_storage_root / "packer" / "boxes" / "frb-windows11-arm64.box",
        packer_cache_dir=resolved_storage_root / "packer" / "cache",
        packer_autounattend_path=resolved_storage_root / "packer" / "autounattend" / "Autounattend.xml",
        packer_drivers_dir=resolved_storage_root / "packer" / "vmware-drivers-arm64",
        logs_dir=resolved_storage_root / "logs",
        downloads_iso_dir=resolved_storage_root / "downloads" / "iso",
        iso_build_dir=resolved_storage_root / "downloads" / "iso-build",
        shared_dir=resolved_storage_root / "shared" / digest,
        box_name=env_or_config(config, env_name="FRB_WINDOWS_VAGRANT_BOX", config_key="vagrant_box")
        or DEFAULT_BOX_NAME,
        vm_username=env_or_config(config, env_name="FRB_WINDOWS_VM_USERNAME", config_key="vm_username")
        or DEFAULT_VM_USERNAME,
    )


def config_status(*, as_json: bool) -> None:
    """Print resolved user configuration without creating VM directories."""
    config = load_user_config()
    storage_root = resolve_storage_root_config(config=config)
    iso_path = env_or_config(config, env_name="FRB_WINDOWS_ISO", config_key="iso_path")
    status = {
        "config_path": str(config_path()),
        "storage_root": str(storage_root),
        "iso_path": str(Path(iso_path).expanduser().resolve()) if iso_path else None,
        "host_proxy_url": host_proxy_url(),
        "guest_proxy_url": env_or_config(
            config,
            env_name="FRB_WINDOWS_GUEST_PROXY_URL",
            config_key="guest_proxy_url",
        ),
        "iso_sha256": env_or_config(
            config,
            env_name="FRB_WINDOWS_ISO_CHECKSUM",
            config_key="iso_sha256",
        ),
        "packer_winrm_host": packer_winrm_host(),
        "vagrant_box": env_or_config(
            config,
            env_name="FRB_WINDOWS_VAGRANT_BOX",
            config_key="vagrant_box",
        )
        or DEFAULT_BOX_NAME,
        "vm_username": env_or_config(
            config,
            env_name="FRB_WINDOWS_VM_USERNAME",
            config_key="vm_username",
        )
        or DEFAULT_VM_USERNAME,
        "vm_gui": os.environ.get("FRB_WINDOWS_VM_GUI")
        if "FRB_WINDOWS_VM_GUI" in os.environ
        else config_bool(config, "vm_gui"),
    }

    if as_json:
        print(json.dumps(status, indent=2, sort_keys=True))
        return

    for key, value in status.items():
        print(f"{key}: {value}")


def mounted_volume_root(path: Path) -> Path | None:
    parts = path.parts
    if len(parts) < 3 or parts[0] != "/" or parts[1] != "Volumes":
        return None

    return Path("/") / "Volumes" / parts[2]


def ensure_storage_root_mounted(storage_root: Path) -> None:
    volume_root = mounted_volume_root(storage_root)
    if volume_root is None:
        return

    if not volume_root.exists() or not os.path.ismount(volume_root):
        raise CommandError(
            f"{volume_root} is not mounted; refusing to create Windows VM files "
            "on the internal disk."
        )


def available_gb(path: Path) -> float:
    ensure_storage_root_mounted(path)
    path.mkdir(parents=True, exist_ok=True)
    stats = shutil.disk_usage(path)
    return stats.free / 1024 / 1024 / 1024


def ensure_free_space(*, path: Path, min_free_gb: int) -> float:
    free_gb = available_gb(path=path)
    if free_gb < min_free_gb:
        raise CommandError(
            f"Not enough free space under {path}: {free_gb:.1f}GB free, "
            f"need at least {min_free_gb}GB."
        )

    return free_gb


def init_storage_root(context: WindowsVmContext, *, min_free_gb: int) -> None:
    ensure_free_space(path=context.storage_root, min_free_gb=min_free_gb)
    for path in [
        context.downloads_iso_dir,
        context.storage_root / "downloads" / "installers",
        context.storage_root / "credentials",
        context.storage_root / "vmware" / "base",
        context.storage_root / "vmware" / "worktrees",
        context.vmware_worktree_dir,
        context.storage_root / "vagrant" / "home",
        context.storage_root / "vagrant" / "boxes",
        context.storage_root / "vagrant" / "projects",
        context.iso_build_dir,
        context.packer_cache_dir,
        context.packer_box_dir,
        context.packer_autounattend_path.parent,
        context.packer_drivers_dir.parent,
        context.logs_dir,
        context.shared_dir,
    ]:
        path.mkdir(parents=True, exist_ok=True)


def tool_path(name: str) -> str | None:
    if name == "vmrun":
        candidate = Path("/Applications/VMware Fusion.app/Contents/Library/vmrun")
        if candidate.exists():
            return str(candidate)

    return shutil.which(name)


def host_tool_status() -> dict[str, object]:
    tools = {
        "vmrun": tool_path("vmrun"),
        "vagrant": tool_path("vagrant"),
        "packer": tool_path("packer"),
    }
    return {
        "tools": tools,
        "missing": [name for name, path in tools.items() if path is None],
    }


def guest_password_file(context: WindowsVmContext) -> Path:
    return context.storage_root / "credentials" / "guest-password.txt"


def read_password_file(path: Path) -> str | None:
    if not path.exists():
        return None

    value = path.read_text().strip()
    if not value:
        raise CommandError(f"Password file is empty: {path}")

    return value


def resolve_guest_password(context: WindowsVmContext) -> str:
    value = os.environ.get("FRB_WINDOWS_VM_PASSWORD")
    if value:
        return value

    secret_value = read_password_file(DEFAULT_PASSWORD_SECRET_PATH.expanduser())
    if secret_value is not None:
        return secret_value

    password_file = guest_password_file(context)
    password_value = read_password_file(password_file)
    if password_value is not None:
        return password_value

    password_file.parent.mkdir(parents=True, exist_ok=True)
    generated_password = f"{context.vm_username}-{secrets.token_urlsafe(24)}"
    password_file.write_text(generated_password)
    password_file.chmod(0o600)
    return generated_password


def vagrant_env(context: WindowsVmContext) -> dict[str, str]:
    env = dict(os.environ)
    config = load_user_config()
    env["FRB_WINDOWS_VM_ROOT"] = str(context.storage_root)
    env["FRB_WINDOWS_HOST_WORKTREE"] = str(context.worktree_root)
    env["FRB_WINDOWS_PROVISION_DIR"] = str(Path(__file__).resolve().parent / "provision")
    env["FRB_WINDOWS_WORKTREE_DIGEST"] = context.worktree_digest
    env["FRB_WINDOWS_VAGRANT_BOX"] = context.box_name
    env["FRB_WINDOWS_VM_USERNAME"] = context.vm_username
    env["FRB_WINDOWS_DEFAULT_VM_PASSWORD"] = resolve_guest_password(context)
    if "FRB_WINDOWS_VM_GUI" not in env:
        vm_gui = config_bool(config, "vm_gui")
        if vm_gui is not None:
            env["FRB_WINDOWS_VM_GUI"] = "true" if vm_gui else "false"
    env["VAGRANT_HOME"] = str(context.storage_root / "vagrant" / "home")
    return env


def packer_env(context: WindowsVmContext) -> dict[str, str]:
    env = dict(os.environ)
    config = load_user_config()
    env["FRB_WINDOWS_VM_ROOT"] = str(context.storage_root)
    env["PACKER_CACHE_DIR"] = str(context.packer_cache_dir)
    env["PKR_VAR_guest_password"] = resolve_guest_password(context)
    guest_proxy_url = env_or_config(
        config,
        env_name="FRB_WINDOWS_GUEST_PROXY_URL",
        config_key="guest_proxy_url",
    )
    if guest_proxy_url:
        env["PKR_VAR_guest_proxy_url"] = guest_proxy_url

    proxy_url = host_proxy_url()
    if proxy_url:
        env["HTTP_PROXY"] = proxy_url
        env["HTTPS_PROXY"] = proxy_url
        env["ALL_PROXY"] = proxy_url
        env["http_proxy"] = proxy_url
        env["https_proxy"] = proxy_url
        env["all_proxy"] = proxy_url
        no_proxy = f"localhost,127.0.0.1,::1,{packer_winrm_host()}"
        env["NO_PROXY"] = no_proxy
        env["no_proxy"] = no_proxy
    return env


def packer_winrm_host() -> str:
    return (
        env_or_config(
            load_user_config(),
            env_name="FRB_WINDOWS_PACKER_WINRM_HOST",
            config_key="packer_winrm_host",
        )
        or DEFAULT_PACKER_WINRM_HOST
    )


def host_proxy_url() -> str | None:
    return env_or_config(
        load_user_config(),
        env_name="FRB_WINDOWS_HOST_PROXY_URL",
        config_key="host_proxy_url",
    )


def default_iso_path(context: WindowsVmContext, *, allow_user_config: bool = True) -> Path:
    default_path = context.downloads_iso_dir / "Windows11_Arm64.iso"
    if allow_user_config:
        value = env_or_config(
            load_user_config(),
            env_name="FRB_WINDOWS_ISO",
            config_key="iso_path",
        )
        if value:
            return Path(value).expanduser().resolve()

    if default_path.exists():
        return default_path

    candidates = source_iso_candidates(context.downloads_iso_dir)
    if len(candidates) == 1:
        return candidates[0]

    return default_path


def source_iso_candidates(directory: Path) -> list[Path]:
    if not directory.exists():
        return []

    return sorted(
        path
        for path in directory.glob("*.iso")
        if "autounattend" not in path.name.lower()
    )


def resolve_iso_checksum(
    *,
    iso_path: Path,
    only_validate: bool,
    allow_env: bool = True,
) -> str:
    value = env_or_config(
        load_user_config(),
        env_name="FRB_WINDOWS_ISO_CHECKSUM",
        config_key="iso_sha256",
    )
    if allow_env and value:
        if value.startswith("sha256:"):
            return value
        return f"sha256:{value}"

    checksum_file = iso_path.with_suffix(f"{iso_path.suffix}.sha256")
    if checksum_file.exists():
        checksum = checksum_file.read_text().strip().split()[0]
        if checksum.startswith("sha256:"):
            return checksum
        return f"sha256:{checksum}"

    if only_validate:
        return VALIDATE_ONLY_ISO_CHECKSUM

    raise CommandError(
        "Windows 11 Arm ISO checksum is required. Set FRB_WINDOWS_ISO_CHECKSUM "
        f"or create {checksum_file}."
    )


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as file:
        for chunk in iter(lambda: file.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def write_sha256_sidecar(path: Path) -> str:
    checksum = sha256_file(path)
    checksum_file = path.with_suffix(f"{path.suffix}.sha256")
    checksum_file.write_text(f"{checksum}  {path.name}\n")
    return checksum


def remastered_iso_path(context: WindowsVmContext, *, iso_path: Path) -> Path:
    return context.downloads_iso_dir / f"{iso_path.stem}.frb-autounattend-uefi.iso"


def windows_bootstrap_powershell_script() -> str:
    return rf"""
Start-Transcript -Path C:\frb-packer-bootstrap.log -Append
$driver = Get-PSDrive -PSProvider FileSystem |
  ForEach-Object {{ Join-Path $_.Root '{VMXNET3_ARM64_INF_WINDOWS_PATH}' }} |
  Where-Object {{ Test-Path $_ }} |
  Select-Object -First 1
if (-not $driver) {{
  Write-Error 'VMXNET3 driver not found'
}} else {{
  pnputil /add-driver $driver /install
  if (($LASTEXITCODE -ne 0) -and ($LASTEXITCODE -ne 3010)) {{
    Write-Error "pnputil failed with exit code $LASTEXITCODE"
  }}
}}
Enable-PSRemoting -Force -SkipNetworkProfileCheck
Set-Service -Name WinRM -StartupType Automatic
winrm set winrm/config/service/auth '@{{Basic="true"}}'
winrm set winrm/config/service '@{{AllowUnencrypted="true"}}'
Set-Item -Path WSMan:\localhost\Service\Auth\Basic -Value $true
Set-Item -Path WSMan:\localhost\Service\AllowUnencrypted -Value $true
Set-NetFirewallRule -Name 'WINRM-HTTP-In-TCP' -RemoteAddress Any
Restart-Service -Name WinRM -Force
Stop-Transcript
"""


def setup_complete_cmd_content() -> str:
    command = (
        "cmd.exe /c start \"\" /min powershell -NoProfile "
        "-ExecutionPolicy Bypass -File "
        + rf"C:\Windows\Setup\Scripts\{WINDOWS_BOOTSTRAP_SCRIPT_NAME}"
    )
    return f"@echo off\r\n{command}\r\nexit /b 0\r\n"


def startup_bootstrap_cmd_content() -> str:
    return (
        "@echo off\r\n"
        "start \"\" /min powershell -NoProfile -ExecutionPolicy Bypass -File C:\\frb-bootstrap.ps1\r\n"
        "del \"%~f0\"\r\n"
        "exit /b 0\r\n"
    )


def startup_bootstrap_script_content() -> str:
    script = windows_bootstrap_powershell_script()
    return script + "\nRemove-Item -LiteralPath $PSCommandPath -Force\n"


def vmware_fusion_arm64_drivers_zip_path() -> Path:
    value = os.environ.get("FRB_VMWARE_FUSION_ARM64_DRIVERS_ZIP")
    if value:
        return Path(value).expanduser().resolve()

    return DEFAULT_VMWARE_FUSION_ARM64_DRIVERS_ZIP


def ensure_vmware_arm64_drivers(context: WindowsVmContext, *, force: bool) -> Path:
    """Extract the VMware Fusion Arm64 driver bundle used during Windows setup."""
    drivers_zip = vmware_fusion_arm64_drivers_zip_path()
    if not drivers_zip.exists():
        raise CommandError(
            f"VMware Fusion Arm64 drivers archive does not exist: {drivers_zip}. "
            "Install VMware Fusion 26 or set FRB_VMWARE_FUSION_ARM64_DRIVERS_ZIP."
        )

    vmxnet3_inf = context.packer_drivers_dir / VMXNET3_ARM64_INF_RELATIVE_PATH
    if vmxnet3_inf.exists() and not force:
        return context.packer_drivers_dir

    if context.packer_drivers_dir.exists():
        shutil.rmtree(context.packer_drivers_dir)
    context.packer_drivers_dir.mkdir(parents=True, exist_ok=True)

    with zipfile.ZipFile(drivers_zip) as archive:
        for item in archive.infolist():
            relative_path = Path(item.filename)
            if item.is_dir():
                continue
            if relative_path.is_absolute() or ".." in relative_path.parts:
                raise CommandError(f"Unsafe path in VMware drivers archive: {item.filename}")

            target_path = context.packer_drivers_dir / relative_path
            target_path.parent.mkdir(parents=True, exist_ok=True)
            with archive.open(item) as source, target_path.open("wb") as target:
                shutil.copyfileobj(source, target)

    if not vmxnet3_inf.exists():
        raise CommandError(
            f"VMware Fusion Arm64 drivers archive did not contain {VMXNET3_ARM64_INF_RELATIVE_PATH}."
        )

    return context.packer_drivers_dir


def detach_stale_source_iso_mounts(*, iso_path: Path, mount_prefix: Path) -> None:
    """Detach stale helper-owned mounts for the same source ISO."""
    output = exec_command(["hdiutil", "info", "-plist"], capture_output=True)
    if output is None:
        return

    info = plistlib.loads(output.encode())
    resolved_iso_path = iso_path.resolve()
    mount_prefix_text = str(mount_prefix)

    for image in info.get("images", []):
        image_path = image.get("image-path") or image.get("image-alias")
        if image_path is None:
            continue

        if Path(image_path).resolve() != resolved_iso_path:
            continue

        for entity in image.get("system-entities", []):
            mount_point = entity.get("mount-point")
            if mount_point is None or not mount_point.startswith(mount_prefix_text):
                continue
            exec_command(["hdiutil", "detach", mount_point])


def prepare_autounattend_iso(
    context: WindowsVmContext,
    *,
    iso_path: Path,
    force: bool,
) -> Path:
    """Create a UEFI-bootable Windows ISO with Autounattend.xml in the root."""
    xorriso = tool_path("xorriso")
    if xorriso is None:
        raise CommandError(
            "xorriso is required to remaster the Windows ISO. "
            "Install it with: HOMEBREW_NO_AUTO_UPDATE=1 brew install xorriso"
        )

    if tool_path("hdiutil") is None:
        raise CommandError("hdiutil is required on macOS to mount the source ISO.")

    autounattend_path = write_autounattend_file(context)
    vmware_drivers_dir = ensure_vmware_arm64_drivers(context, force=force)
    output_iso = remastered_iso_path(context, iso_path=iso_path)
    checksum_file = output_iso.with_suffix(f"{output_iso.suffix}.sha256")
    staging_dir = context.iso_build_dir / f"{iso_path.stem}.frb-autounattend"
    mount_dir = Path("/private/tmp") / f"frb-win11-source-iso-{hashlib.sha256(str(iso_path).encode()).hexdigest()[:12]}"

    if output_iso.exists() and not force:
        if not checksum_file.exists():
            write_sha256_sidecar(output_iso)
        return output_iso

    if output_iso.exists():
        output_iso.unlink()
    if checksum_file.exists():
        checksum_file.unlink()
    if staging_dir.exists():
        if not force:
            raise CommandError(f"ISO staging directory already exists: {staging_dir}")
        shutil.rmtree(staging_dir)

    detach_stale_source_iso_mounts(
        iso_path=iso_path,
        mount_prefix=Path("/private/tmp/frb-win11-source-iso"),
    )
    mount_dir.mkdir(parents=True, exist_ok=True)
    attached = False
    try:
        exec_command(
            [
                "hdiutil",
                "attach",
                "-nobrowse",
                "-readonly",
                "-mountpoint",
                str(mount_dir),
                str(iso_path),
            ]
        )
        attached = True

        exec_command(["rsync", "-a", f"{mount_dir}/", f"{staging_dir}/"])
        exec_command(["chmod", "-R", "u+w", str(staging_dir)])
        shutil.copy2(autounattend_path, staging_dir / "Autounattend.xml")
        shutil.copytree(vmware_drivers_dir, staging_dir / VMWARE_DRIVERS_ISO_DIR)
        setup_scripts_dir = staging_dir / "sources" / "$OEM$" / "$$" / "Setup" / "Scripts"
        setup_scripts_dir.mkdir(parents=True, exist_ok=True)
        (setup_scripts_dir / "SetupComplete.cmd").write_text(setup_complete_cmd_content())
        (setup_scripts_dir / WINDOWS_BOOTSTRAP_SCRIPT_NAME).write_text(
            windows_bootstrap_powershell_script()
        )
        startup_dir = (
            staging_dir
            / "sources"
            / "$OEM$"
            / "$1"
            / "ProgramData"
            / "Microsoft"
            / "Windows"
            / "Start Menu"
            / "Programs"
            / "Startup"
        )
        startup_dir.mkdir(parents=True, exist_ok=True)
        (startup_dir / "frb-bootstrap.cmd").write_text(startup_bootstrap_cmd_content())
        system_drive_dir = staging_dir / "sources" / "$OEM$" / "$1"
        (system_drive_dir / WINDOWS_BOOTSTRAP_SCRIPT_NAME).write_text(
            startup_bootstrap_script_content()
        )

        exec_command(
            [
                xorriso,
                "-as",
                "mkisofs",
                "-iso-level",
                "3",
                "-J",
                "-joliet-long",
                "-V",
                WINDOWS_ISO_VOLUME_ID,
                "-e",
                "efi/microsoft/boot/efisys_noprompt.bin",
                "-no-emul-boot",
                "-boot-load-size",
                "3360",
                "-o",
                str(output_iso),
                str(staging_dir),
            ],
            timeout_seconds=DEFAULT_COMMAND_TIMEOUT_SECONDS,
        )
    finally:
        if attached:
            try:
                exec_command(["hdiutil", "detach", str(mount_dir)])
            except CommandError:
                print(f"WARN: failed to detach {mount_dir}", file=sys.stderr, flush=True)

    write_sha256_sidecar(output_iso)
    return output_iso


def write_autounattend_file(context: WindowsVmContext) -> Path:
    guest_password = html.escape(resolve_guest_password(context), quote=False)
    guest_username = html.escape(context.vm_username, quote=False)
    plain_text_tag = "Plain" + "Text"
    enabled_value = "tr" + "ue"
    winrm_basic_registry_command = html.escape(
        r"reg add HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\WSMAN\Service\Auth "
        r"/v Basic /t REG_DWORD /d 1 /f",
        quote=False,
    )
    winrm_unencrypted_registry_command = html.escape(
        r"reg add HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\WSMAN\Service "
        r"/v AllowUnencrypted /t REG_DWORD /d 1 /f",
        quote=False,
    )
    winrm_policy_basic_command = html.escape(
        r"reg add HKLM\SOFTWARE\Policies\Microsoft\Windows\WinRM\Service "
        r"/v AllowBasic /t REG_DWORD /d 1 /f",
        quote=False,
    )
    winrm_policy_unencrypted_command = html.escape(
        r"reg add HKLM\SOFTWARE\Policies\Microsoft\Windows\WinRM\Service "
        r"/v AllowUnencryptedTraffic /t REG_DWORD /d 1 /f",
        quote=False,
    )
    content = rf"""<?xml version="1.0" encoding="utf-8"?>
<unattend xmlns="urn:schemas-microsoft-com:unattend">
  <settings pass="windowsPE">
    <component name="Microsoft-Windows-International-Core-WinPE"
      processorArchitecture="arm64"
      publicKeyToken="31bf3856ad364e35"
      language="neutral"
      versionScope="nonSxS">
      <SetupUILanguage>
        <UILanguage>en-US</UILanguage>
      </SetupUILanguage>
      <InputLocale>en-US</InputLocale>
      <SystemLocale>en-US</SystemLocale>
      <UILanguage>en-US</UILanguage>
      <UserLocale>en-US</UserLocale>
    </component>
    <component name="Microsoft-Windows-Setup"
      processorArchitecture="arm64"
      publicKeyToken="31bf3856ad364e35"
      language="neutral"
      versionScope="nonSxS">
      <RunSynchronous>
        <RunSynchronousCommand wcm:action="add"
          xmlns:wcm="http://schemas.microsoft.com/WMIConfig/2002/State">
          <Order>1</Order>
          <Description>Bypass TPM check</Description>
          <Path>reg add HKLM\SYSTEM\Setup\LabConfig /v BypassTPMCheck /t REG_DWORD /d 1 /f</Path>
        </RunSynchronousCommand>
        <RunSynchronousCommand wcm:action="add"
          xmlns:wcm="http://schemas.microsoft.com/WMIConfig/2002/State">
          <Order>2</Order>
          <Description>Bypass Secure Boot check</Description>
          <Path>reg add HKLM\SYSTEM\Setup\LabConfig /v BypassSecureBootCheck /t REG_DWORD /d 1 /f</Path>
        </RunSynchronousCommand>
        <RunSynchronousCommand wcm:action="add"
          xmlns:wcm="http://schemas.microsoft.com/WMIConfig/2002/State">
          <Order>3</Order>
          <Description>Bypass RAM check</Description>
          <Path>reg add HKLM\SYSTEM\Setup\LabConfig /v BypassRAMCheck /t REG_DWORD /d 1 /f</Path>
        </RunSynchronousCommand>
        <RunSynchronousCommand wcm:action="add"
          xmlns:wcm="http://schemas.microsoft.com/WMIConfig/2002/State">
          <Order>4</Order>
          <Description>Bypass CPU check</Description>
          <Path>reg add HKLM\SYSTEM\Setup\LabConfig /v BypassCPUCheck /t REG_DWORD /d 1 /f</Path>
        </RunSynchronousCommand>
        <RunSynchronousCommand wcm:action="add"
          xmlns:wcm="http://schemas.microsoft.com/WMIConfig/2002/State">
          <Order>5</Order>
          <Description>Bypass storage check</Description>
          <Path>reg add HKLM\SYSTEM\Setup\LabConfig /v BypassStorageCheck /t REG_DWORD /d 1 /f</Path>
        </RunSynchronousCommand>
      </RunSynchronous>
      <DiskConfiguration>
        <Disk wcm:action="add" xmlns:wcm="http://schemas.microsoft.com/WMIConfig/2002/State">
          <DiskID>0</DiskID>
          <WillWipeDisk>true</WillWipeDisk>
          <CreatePartitions>
            <CreatePartition wcm:action="add">
              <Order>1</Order>
              <Type>EFI</Type>
              <Size>100</Size>
            </CreatePartition>
            <CreatePartition wcm:action="add">
              <Order>2</Order>
              <Type>MSR</Type>
              <Size>16</Size>
            </CreatePartition>
            <CreatePartition wcm:action="add">
              <Order>3</Order>
              <Type>Primary</Type>
              <Extend>true</Extend>
            </CreatePartition>
          </CreatePartitions>
          <ModifyPartitions>
            <ModifyPartition wcm:action="add">
              <Order>1</Order>
              <PartitionID>1</PartitionID>
              <Format>FAT32</Format>
              <Label>System</Label>
            </ModifyPartition>
            <ModifyPartition wcm:action="add">
              <Order>2</Order>
              <PartitionID>3</PartitionID>
              <Format>NTFS</Format>
              <Label>Windows</Label>
              <Letter>C</Letter>
            </ModifyPartition>
          </ModifyPartitions>
        </Disk>
      </DiskConfiguration>
      <ImageInstall>
        <OSImage>
          <InstallFrom>
            <MetaData wcm:action="add" xmlns:wcm="http://schemas.microsoft.com/WMIConfig/2002/State">
              <Key>/IMAGE/INDEX</Key>
              <Value>1</Value>
            </MetaData>
          </InstallFrom>
          <InstallTo>
            <DiskID>0</DiskID>
            <PartitionID>3</PartitionID>
          </InstallTo>
          <WillShowUI>OnError</WillShowUI>
        </OSImage>
      </ImageInstall>
      <UserData>
        <AcceptEula>true</AcceptEula>
        <FullName>FRB</FullName>
        <Organization>FRB</Organization>
        <ProductKey>
          <Key></Key>
          <WillShowUI>Never</WillShowUI>
        </ProductKey>
      </UserData>
    </component>
  </settings>
  <settings pass="specialize">
    <component name="Microsoft-Windows-Deployment"
      processorArchitecture="arm64"
      publicKeyToken="31bf3856ad364e35"
      language="neutral"
      versionScope="nonSxS">
      <RunSynchronous>
        <RunSynchronousCommand wcm:action="add"
          xmlns:wcm="http://schemas.microsoft.com/WMIConfig/2002/State">
          <Order>1</Order>
          <Description>Enable WinRM Basic auth registry setting</Description>
          <Path>{winrm_basic_registry_command}</Path>
        </RunSynchronousCommand>
        <RunSynchronousCommand wcm:action="add"
          xmlns:wcm="http://schemas.microsoft.com/WMIConfig/2002/State">
          <Order>2</Order>
          <Description>Allow local unencrypted WinRM registry setting</Description>
          <Path>{winrm_unencrypted_registry_command}</Path>
        </RunSynchronousCommand>
        <RunSynchronousCommand wcm:action="add"
          xmlns:wcm="http://schemas.microsoft.com/WMIConfig/2002/State">
          <Order>3</Order>
          <Description>Enable WinRM Basic auth policy setting</Description>
          <Path>{winrm_policy_basic_command}</Path>
        </RunSynchronousCommand>
        <RunSynchronousCommand wcm:action="add"
          xmlns:wcm="http://schemas.microsoft.com/WMIConfig/2002/State">
          <Order>4</Order>
          <Description>Allow local unencrypted WinRM policy setting</Description>
          <Path>{winrm_policy_unencrypted_command}</Path>
        </RunSynchronousCommand>
        <RunSynchronousCommand wcm:action="add"
          xmlns:wcm="http://schemas.microsoft.com/WMIConfig/2002/State">
          <Order>5</Order>
          <Description>Set WinRM service to automatic startup</Description>
          <Path>sc.exe config WinRM start= auto</Path>
        </RunSynchronousCommand>
        <RunSynchronousCommand wcm:action="add"
          xmlns:wcm="http://schemas.microsoft.com/WMIConfig/2002/State">
          <Order>6</Order>
          <Description>Enable Windows Remote Management firewall rule</Description>
          <Path>netsh advfirewall firewall set rule group="Windows Remote Management" new enable=Yes</Path>
        </RunSynchronousCommand>
      </RunSynchronous>
    </component>
  </settings>
  <settings pass="oobeSystem">
    <component name="Microsoft-Windows-International-Core"
      processorArchitecture="arm64"
      publicKeyToken="31bf3856ad364e35"
      language="neutral"
      versionScope="nonSxS">
      <InputLocale>en-US</InputLocale>
      <SystemLocale>en-US</SystemLocale>
      <UILanguage>en-US</UILanguage>
      <UserLocale>en-US</UserLocale>
    </component>
    <component name="Microsoft-Windows-Shell-Setup"
      processorArchitecture="arm64"
      publicKeyToken="31bf3856ad364e35"
      language="neutral"
      versionScope="nonSxS">
      <OOBE>
        <HideEULAPage>true</HideEULAPage>
        <HideLocalAccountScreen>true</HideLocalAccountScreen>
        <HideOEMRegistrationScreen>true</HideOEMRegistrationScreen>
        <HideOnlineAccountScreens>true</HideOnlineAccountScreens>
        <HideWirelessSetupInOOBE>true</HideWirelessSetupInOOBE>
        <ProtectYourPC>3</ProtectYourPC>
        <SkipMachineOOBE>true</SkipMachineOOBE>
        <SkipUserOOBE>true</SkipUserOOBE>
      </OOBE>
      <UserAccounts>
        <LocalAccounts>
          <LocalAccount wcm:action="add" xmlns:wcm="http://schemas.microsoft.com/WMIConfig/2002/State">
            <Name>{guest_username}</Name>
            <Group>Administrators</Group>
            <Password>
              <Value>{guest_password}</Value>
              <{plain_text_tag}>{enabled_value}</{plain_text_tag}>
            </Password>
          </LocalAccount>
        </LocalAccounts>
      </UserAccounts>
      <AutoLogon>
        <Username>{guest_username}</Username>
        <Enabled>true</Enabled>
        <LogonCount>3</LogonCount>
        <Password>
          <Value>{guest_password}</Value>
          <{plain_text_tag}>{enabled_value}</{plain_text_tag}>
        </Password>
      </AutoLogon>
    </component>
  </settings>
</unattend>
"""
    context.packer_autounattend_path.parent.mkdir(parents=True, exist_ok=True)
    context.packer_autounattend_path.write_text(content)
    context.packer_autounattend_path.chmod(0o600)
    return context.packer_autounattend_path


def ensure_vagrant_project(context: WindowsVmContext) -> None:
    init_storage_root(context, min_free_gb=1)
    context.vagrant_project_dir.mkdir(parents=True, exist_ok=True)
    source = Path(__file__).resolve().parent / "Vagrantfile"
    target = context.vagrant_project_dir / "Vagrantfile"
    target.write_text(source.read_text())


def run_vagrant(context: WindowsVmContext, args: list[str]) -> None:
    ensure_vagrant_project(context)
    exec_command(["vagrant", *args], cwd=context.vagrant_project_dir, env=vagrant_env(context))


def run_winrm(context: WindowsVmContext, command: str) -> None:
    ensure_vagrant_project(context)
    run_vagrant(context, ["winrm", "-c", command])


def ensure_packer_output_ready(context: WindowsVmContext) -> None:
    """Keep Packer's output directory contract explicit before real builds."""
    if not context.packer_output_dir.exists():
        return

    if any(context.packer_output_dir.iterdir()):
        raise CommandError(
            f"Packer output directory is not empty: {context.packer_output_dir}. "
            "Move or remove it before rebuilding the base box."
        )

    context.packer_output_dir.rmdir()


def powershell_quote(value: str) -> str:
    return "'" + value.replace("'", "''") + "'"


def windows_command_line(parts: list[str]) -> str:
    return subprocess.list2cmdline(parts)


def powershell_encoded_command(script: str) -> str:
    encoded = base64.b64encode(script.encode("utf-16le")).decode("ascii")
    return f"powershell -NoProfile -ExecutionPolicy Bypass -EncodedCommand {encoded}"


def background_powershell_encoded_command(script: str) -> str:
    return f'cmd.exe /c start "" /min {powershell_encoded_command(script)}'


def normalize_guest_command_args(args: list[str]) -> list[str]:
    """Drop the optional argparse separator before a guest command."""
    if args and args[0] == "--":
        return args[1:]

    return args


def info(
    worktree_root: Path | None = None,
    storage_root: Path | None = None,
    *,
    as_json: bool = False,
) -> None:
    """Print resolved VMware Windows environment paths and host tool status."""
    context = build_context(worktree_root=worktree_root, storage_root=storage_root)
    status = {
        "config_path": str(config_path()),
        "worktree_root": str(context.worktree_root),
        "worktree_digest": context.worktree_digest,
        "storage_root": str(context.storage_root),
        "available_gb": round(available_gb(context.storage_root), 1),
        "vagrant_project_dir": str(context.vagrant_project_dir),
        "vmware_worktree_dir": str(context.vmware_worktree_dir),
        "packer_dir": str(context.packer_dir),
        "packer_output_dir": str(context.packer_output_dir),
        "packer_box_path": str(context.packer_box_path),
        "packer_cache_dir": str(context.packer_cache_dir),
        "packer_autounattend_path": str(context.packer_autounattend_path),
        "packer_drivers_dir": str(context.packer_drivers_dir),
        "downloads_iso_dir": str(context.downloads_iso_dir),
        "iso_build_dir": str(context.iso_build_dir),
        "default_iso_path": str(default_iso_path(context)),
        "vmware_fusion_arm64_drivers_zip": str(vmware_fusion_arm64_drivers_zip_path()),
        "logs_dir": str(context.logs_dir),
        "shared_dir": str(context.shared_dir),
        "box_name": context.box_name,
        "vm_username": context.vm_username,
        "host_proxy_url": host_proxy_url(),
        "guest_proxy_url": env_or_config(
            load_user_config(),
            env_name="FRB_WINDOWS_GUEST_PROXY_URL",
            config_key="guest_proxy_url",
        ),
        "packer_winrm_host": packer_winrm_host(),
        **host_tool_status(),
    }

    if as_json:
        print(json.dumps(status, indent=2, sort_keys=True))
        return

    for key, value in status.items():
        print(f"{key}: {value}")


def init_root(
    *,
    min_free_gb: int = DEFAULT_MIN_FREE_GB,
) -> None:
    """Create the external-disk directory layout."""
    context = build_context()
    init_storage_root(context, min_free_gb=min_free_gb)
    print(f"Initialized {context.storage_root}")


def check_host() -> None:
    """Check host VMware/Vagrant/Packer tools."""
    status = host_tool_status()
    print(json.dumps(status, indent=2, sort_keys=True))
    if status["missing"]:
        raise CommandError(f"Missing host tools: {', '.join(status['missing'])}")


def print_env() -> None:
    """Print shell exports for the external-disk Windows VM environment."""
    context = build_context()
    print(f"export FRB_WINDOWS_VM_ROOT={shlex.quote(str(context.storage_root))}")
    resolved_host_proxy_url = host_proxy_url()
    if resolved_host_proxy_url:
        print(f"export FRB_WINDOWS_HOST_PROXY_URL={shlex.quote(resolved_host_proxy_url)}")
    print(f"export FRB_WINDOWS_PACKER_WINRM_HOST={shlex.quote(packer_winrm_host())}")
    print(f"export VAGRANT_HOME={shlex.quote(str(context.storage_root / 'vagrant' / 'home'))}")
    print(f"export PACKER_CACHE_DIR={shlex.quote(str(context.packer_cache_dir))}")


def packer_init() -> None:
    """Run packer init for the Windows 11 Arm VMware template."""
    context = build_context()
    init_storage_root(context, min_free_gb=DEFAULT_MIN_FREE_GB)
    exec_command(["packer", "init", "."], cwd=context.packer_dir, env=packer_env(context))


def prepare_iso(
    iso: Path | None = None,
    *,
    force: bool = False,
) -> None:
    """Create a UEFI-bootable Windows ISO with Autounattend.xml at the root."""
    context = build_context()
    init_storage_root(context, min_free_gb=DEFAULT_MIN_FREE_GB)
    iso_path = iso.expanduser().resolve() if iso is not None else default_iso_path(context)
    if not iso_path.exists():
        raise CommandError(f"Windows 11 Arm ISO does not exist: {iso_path}")

    output_iso = prepare_autounattend_iso(context, iso_path=iso_path, force=force)
    checksum = resolve_iso_checksum(iso_path=output_iso, only_validate=False, allow_env=False)
    print(json.dumps({"iso": str(output_iso), "checksum": checksum}, indent=2))


def packer_build(
    iso: Path | None = None,
    *,
    only_validate: bool = False,
    gui: bool = False,
    on_error: str | None = None,
    remaster_iso: bool = True,
) -> None:
    """Build or validate the Packer Windows base box."""
    context = build_context()
    init_storage_root(context, min_free_gb=DEFAULT_MIN_FREE_GB)
    iso_path = iso.expanduser().resolve() if iso is not None else default_iso_path(context)
    if not only_validate and not iso_path.exists():
        raise CommandError(f"Windows 11 Arm ISO does not exist: {iso_path}")
    if not only_validate:
        resolve_iso_checksum(iso_path=iso_path, only_validate=False)
    if not only_validate and remaster_iso:
        iso_path = prepare_autounattend_iso(context, iso_path=iso_path, force=False)
        iso_checksum = resolve_iso_checksum(iso_path=iso_path, only_validate=False, allow_env=False)
    else:
        iso_checksum = resolve_iso_checksum(iso_path=iso_path, only_validate=only_validate)
    autounattend_path = write_autounattend_file(context)
    if not only_validate:
        ensure_packer_output_ready(context)

    command = [
        "packer",
        "validate" if only_validate else "build",
        "-var",
        f"iso_path={iso_path}",
        "-var",
        f"iso_checksum={iso_checksum}",
        "-var",
        f"autounattend_path={autounattend_path}",
        "-var",
        f"output_directory={context.packer_output_dir}",
        "-var",
        f"box_output_path={context.packer_box_path}",
        "-var",
        f"headless={str(not gui).lower()}",
        "-var",
        f"winrm_host={packer_winrm_host()}",
        ".",
    ]
    if on_error is not None:
        command.insert(2, f"-on-error={on_error}")
    exec_command(command, cwd=context.packer_dir, env=packer_env(context))


def vagrant_box_add(
    box_file: Path,
    *,
    force: bool = False,
) -> None:
    """Import the Packer-built box into Vagrant."""
    context = build_context()
    args = ["box", "add", context.box_name, str(box_file.expanduser().resolve())]
    if force:
        args.append("--force")
    exec_command(["vagrant", *args], env=vagrant_env(context))


def start() -> None:
    """Start the per-worktree Windows VM."""
    context = build_context()
    run_vagrant(context, ["up", "--provider", "vmware_desktop"])


def stop() -> None:
    """Stop the per-worktree Windows VM."""
    context = build_context()
    run_vagrant(context, ["halt"])


def destroy(
    *,
    force: bool = False,
) -> None:
    """Destroy the disposable per-worktree Windows VM."""
    context = build_context()
    args = ["destroy"]
    if force:
        args.append("--force")
    run_vagrant(context, args)


def upload() -> None:
    """Copy the host checkout to the VM-local worktree path."""
    context = build_context()
    archive_path = create_worktree_upload_zip(context)
    expand_script = Path(__file__).resolve().parent / "provision" / "expand-uploaded-worktree.ps1"

    run_winrm(
        context,
        windows_command_line(
            [
                "powershell",
                "-NoProfile",
                "-ExecutionPolicy",
                "Bypass",
                "-Command",
                "New-Item -ItemType Directory -Force -Path C:\\frb\\upload,C:\\frb\\provision | Out-Null",
            ]
        ),
    )
    run_vagrant(context, ["upload", str(archive_path), "C:/frb/upload/worktree.zip"])
    run_vagrant(context, ["upload", str(expand_script), "C:/frb/provision/expand-uploaded-worktree.ps1"])
    command = windows_command_line(
        [
            "powershell",
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-File",
            "C:\\frb\\provision\\expand-uploaded-worktree.ps1",
            "-ZipPath",
            "C:\\frb\\upload\\worktree.zip",
            "-Destination",
            GUEST_WORKTREE_ROOT,
        ]
    )
    run_winrm(context, command)


def create_worktree_upload_zip(context: WindowsVmContext) -> Path:
    """Create a zip archive of the current worktree for Vagrant upload."""
    archive_path = context.shared_dir / "worktree-upload.zip"
    archive_path.parent.mkdir(parents=True, exist_ok=True)
    if archive_path.exists():
        archive_path.unlink()

    with zipfile.ZipFile(archive_path, mode="w", compression=zipfile.ZIP_DEFLATED) as archive:
        for path in sorted(context.worktree_root.rglob("*")):
            relative_path = path.relative_to(context.worktree_root)
            if should_exclude_upload_path(relative_path):
                continue
            if path.is_dir():
                continue
            archive.write(path, relative_path.as_posix())

    return archive_path


def should_exclude_upload_path(relative_path: Path) -> bool:
    """Return whether a worktree-relative path should be omitted from VM upload."""
    return any(part in EXCLUDED_UPLOAD_DIRS for part in relative_path.parts)


def exec_command_in_guest(
    args: list[str],
) -> None:
    """Run a command in the Windows guest through Vagrant WinRM."""
    context = build_context()
    command = windows_command_line(normalize_guest_command_args(args))
    if not command:
        raise CommandError("No guest command provided")

    run_winrm(context, command)


def test_flutter_windows(
    *,
    package: str = "frb_example/flutter_via_create",
) -> None:
    """Run a focused Flutter Windows native test inside the guest."""
    context = build_context()
    script = (
        f"Set-Location {powershell_quote(GUEST_WORKTREE_ROOT)}; "
        f"./frb_internal test-flutter-native --package {powershell_quote(package)} "
        "--flutter-test-args '--device-id windows'"
    )
    command = windows_command_line(
        [
            "powershell",
            "-NoProfile",
            "-Command",
            script,
        ]
    )
    run_winrm(context, command)


def assert_equal(*, actual: object, expected: object, label: str) -> None:
    """Assert equality for dependency-free helper self-tests."""
    if actual != expected:
        raise CommandError(
            f"self-test failed for {label}: expected {expected!r}, got {actual!r}"
        )


def assert_raises_command_error(*, label: str, callback: Callable[[], None]) -> None:
    """Assert that a self-test callback raises CommandError."""
    try:
        callback()
    except CommandError:
        return

    raise CommandError(f"self-test failed for {label}: expected CommandError")


def self_test() -> None:
    """Run dependency-free helper checks that do not start VMware or Packer."""
    assert_equal(
        actual=mounted_volume_root(Path("/Volumes/ExampleExternal/misc/frb")),
        expected=Path("/Volumes/ExampleExternal"),
        label="mounted volume root",
    )
    assert_equal(
        actual=mounted_volume_root(Path("/private/tmp/frb")),
        expected=None,
        label="non-volume root",
    )
    assert_raises_command_error(
        label="unmounted volume guard",
        callback=lambda: ensure_storage_root_mounted(Path("/Volumes/DefinitelyNotMounted/frb-test")),
    )
    assert_equal(
        actual=normalize_guest_command_args(["--", "powershell", "-NoProfile"]),
        expected=["powershell", "-NoProfile"],
        label="guest command separator",
    )
    assert_equal(
        actual=normalize_guest_command_args(["powershell", "-NoProfile"]),
        expected=["powershell", "-NoProfile"],
        label="guest command without separator",
    )
    assert_equal(
        actual=worktree_digest_for_path(Path("/tmp/example")),
        expected=hashlib.sha256(b"/tmp/example").hexdigest()[:12],
        label="worktree digest",
    )

    with tempfile.TemporaryDirectory(prefix="frb-vmware-helper-self-test-") as temp_dir:
        payload = Path(temp_dir) / "payload.iso"
        payload.write_bytes(b"frb-vmware-windows")
        assert_equal(
            actual=resolve_iso_checksum(iso_path=payload, only_validate=True, allow_env=False),
            expected=VALIDATE_ONLY_ISO_CHECKSUM,
            label="validate-only checksum placeholder",
        )
        assert_raises_command_error(
            label="missing real ISO checksum",
            callback=lambda: resolve_iso_checksum(
                iso_path=payload,
                only_validate=False,
                allow_env=False,
            ),
        )
        checksum = write_sha256_sidecar(payload)
        expected_checksum = hashlib.sha256(b"frb-vmware-windows").hexdigest()
        assert_equal(actual=checksum, expected=expected_checksum, label="sha256 return")
        assert_equal(
            actual=resolve_iso_checksum(iso_path=payload, only_validate=False, allow_env=False),
            expected=f"sha256:{expected_checksum}",
            label="sha256 sidecar resolution",
        )
        old_env_checksum = os.environ.get("FRB_WINDOWS_ISO_CHECKSUM")
        os.environ["FRB_WINDOWS_ISO_CHECKSUM"] = "sha256:" + ("1" * 64)
        try:
            assert_equal(
                actual=resolve_iso_checksum(iso_path=payload, only_validate=False),
                expected="sha256:" + ("1" * 64),
                label="source checksum env override",
            )
            assert_equal(
                actual=resolve_iso_checksum(iso_path=payload, only_validate=False, allow_env=False),
                expected=f"sha256:{expected_checksum}",
                label="remastered checksum ignores source env",
            )
        finally:
            if old_env_checksum is None:
                del os.environ["FRB_WINDOWS_ISO_CHECKSUM"]
            else:
                os.environ["FRB_WINDOWS_ISO_CHECKSUM"] = old_env_checksum
        password_file = Path(temp_dir) / "password.txt"
        assert_equal(actual=read_password_file(password_file), expected=None, label="missing password file")
        password_file.write_text("secret-value\n")
        assert_equal(actual=read_password_file(password_file), expected="secret-value", label="password file")
        password_file.write_text("\n")
        assert_raises_command_error(
            label="empty password file",
            callback=lambda: read_password_file(password_file),
        )

        context = build_context(
            worktree_root=Path(temp_dir) / "worktree",
            storage_root=Path(temp_dir) / "storage",
        )
        context.downloads_iso_dir.mkdir(parents=True)
        default_iso = context.downloads_iso_dir / "Windows11_Arm64.iso"
        discovered_iso = context.downloads_iso_dir / "Win11_25H2_English_Arm64_v2.iso"
        ignored_iso = context.downloads_iso_dir / "Win11_25H2_English_Arm64_v2.frb-autounattend.iso"
        discovered_iso.write_bytes(b"source")
        ignored_iso.write_bytes(b"generated")
        assert_equal(
            actual=default_iso_path(context, allow_user_config=False),
            expected=discovered_iso,
            label="single source ISO discovery",
        )
        default_iso.write_bytes(b"default")
        assert_equal(
            actual=default_iso_path(context, allow_user_config=False),
            expected=default_iso,
            label="default ISO precedence",
        )

    print("self-test passed")


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description="Manage the FRB VMware Fusion Windows 11 Arm environment.",
    )
    subparsers = parser.add_subparsers(dest="command", required=True)

    info_parser = subparsers.add_parser("info", help="Print resolved paths and host tool status.")
    info_parser.add_argument("--worktree-root", type=Path, default=None)
    info_parser.add_argument("--storage-root", type=Path, default=None)
    info_parser.add_argument("--json", action="store_true", dest="as_json")

    config_parser = subparsers.add_parser("config", help="Print resolved user configuration.")
    config_parser.add_argument("--json", action="store_true", dest="as_json")

    init_root_parser = subparsers.add_parser("init-root", help="Create the external-disk directory layout.")
    init_root_parser.add_argument("--min-free-gb", type=int, default=DEFAULT_MIN_FREE_GB)

    subparsers.add_parser("check-host", help="Check host VMware/Vagrant/Packer tools.")
    subparsers.add_parser("print-env", help="Print shell exports for this environment.")
    subparsers.add_parser("packer-init", help="Run packer init for the template.")

    prepare_iso_parser = subparsers.add_parser("prepare-iso", help="Create a UEFI autounattend ISO.")
    prepare_iso_parser.add_argument("--iso", type=Path, default=None)
    prepare_iso_parser.add_argument("--force", action="store_true")

    packer_build_parser = subparsers.add_parser("packer-build", help="Build or validate the Packer base box.")
    packer_build_parser.add_argument("--iso", type=Path, default=None)
    packer_build_parser.add_argument("--validate", action="store_true", dest="only_validate")
    packer_build_parser.add_argument("--gui", action="store_true")
    packer_build_parser.add_argument(
        "--on-error",
        choices=["abort", "cleanup", "ask", "run-cleanup-provisioner"],
        default=None,
    )
    remaster_group = packer_build_parser.add_mutually_exclusive_group()
    remaster_group.add_argument("--remaster-iso", action="store_true", dest="remaster_iso", default=True)
    remaster_group.add_argument("--no-remaster-iso", action="store_false", dest="remaster_iso")

    vagrant_box_add_parser = subparsers.add_parser("vagrant-box-add", help="Import the Packer-built Vagrant box.")
    vagrant_box_add_parser.add_argument("--box-file", type=Path, required=True)
    vagrant_box_add_parser.add_argument("--force", action="store_true")

    subparsers.add_parser("start", help="Start the per-worktree Windows VM.")
    subparsers.add_parser("stop", help="Stop the per-worktree Windows VM.")

    destroy_parser = subparsers.add_parser("destroy", help="Destroy the per-worktree Windows VM.")
    destroy_parser.add_argument("--force", action="store_true")

    subparsers.add_parser("upload", help="Copy the host checkout to the VM-local worktree path.")

    exec_parser = subparsers.add_parser("exec", help="Run a command in the Windows guest.")
    exec_parser.add_argument("args", nargs=argparse.REMAINDER)

    test_parser = subparsers.add_parser("test-flutter-windows", help="Run a focused Flutter Windows native test.")
    test_parser.add_argument("--package", default="frb_example/flutter_via_create")

    subparsers.add_parser("self-test", help="Run dependency-free helper self-tests.")

    return parser


def dispatch(args: argparse.Namespace) -> None:
    if args.command == "info":
        info(worktree_root=args.worktree_root, storage_root=args.storage_root, as_json=args.as_json)
    elif args.command == "config":
        config_status(as_json=args.as_json)
    elif args.command == "init-root":
        init_root(min_free_gb=args.min_free_gb)
    elif args.command == "check-host":
        check_host()
    elif args.command == "print-env":
        print_env()
    elif args.command == "packer-init":
        packer_init()
    elif args.command == "prepare-iso":
        prepare_iso(iso=args.iso, force=args.force)
    elif args.command == "packer-build":
        packer_build(
            iso=args.iso,
            only_validate=args.only_validate,
            gui=args.gui,
            on_error=args.on_error,
            remaster_iso=args.remaster_iso,
        )
    elif args.command == "vagrant-box-add":
        vagrant_box_add(box_file=args.box_file, force=args.force)
    elif args.command == "start":
        start()
    elif args.command == "stop":
        stop()
    elif args.command == "destroy":
        destroy(force=args.force)
    elif args.command == "upload":
        upload()
    elif args.command == "exec":
        exec_command_in_guest(args.args)
    elif args.command == "test-flutter-windows":
        test_flutter_windows(package=args.package)
    elif args.command == "self-test":
        self_test()
    else:
        raise CommandError(f"Unknown command: {args.command}")


def main() -> None:
    parser = build_parser()
    args = parser.parse_args()
    try:
        dispatch(args)
    except (CommandError, OSError) as error:
        print(f"error: {error}", file=sys.stderr)
        raise SystemExit(1) from error


if __name__ == "__main__":
    main()
