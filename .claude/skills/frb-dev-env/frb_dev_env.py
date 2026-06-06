#!/usr/bin/env -S uv run --script
# /// script
# dependencies = ["typer>=0.12"]
# ///
from __future__ import annotations

import hashlib
import json
import os
import shlex
import subprocess
import sys
import time
from pathlib import Path
from typing import Annotated

import typer


app = typer.Typer(no_args_is_help=True)
android_app = typer.Typer(no_args_is_help=True)
docker_app = typer.Typer(no_args_is_help=True)
tart_app = typer.Typer(no_args_is_help=True)
app.add_typer(android_app, name="android", help="Manage host Android emulator and ADB commands.")
app.add_typer(docker_app, name="docker", help="Manage the per-worktree Docker development container.")
app.add_typer(tart_app, name="tart", help="Manage the per-worktree Tart macOS VM.")

DEFAULT_IMAGE = "fzyzcjy/flutter_rust_bridge_dev:latest"
DEFAULT_TART_BASE_VM = "frb-tart-base"
DEFAULT_ANDROID_ADB_SERVER_HOST = "host.docker.internal"
DEFAULT_ANDROID_ADB_SERVER_PORT = 5037
DEFAULT_ANDROID_ADB_SERVER_BIND_ADDRESS = "0.0.0.0"
DEFAULT_ANDROID_EMULATOR_PORT = 5554
DEFAULT_MACOS_ANDROID_HOME = Path("~/Library/Android/sdk")
MIN_TART_FREE_GB = 150
REPO_LABEL = "frb.dev.repo"
WORKTREE_LABEL = "frb.dev.worktree"
GIT_COMMON_ROOT_LABEL = "frb.dev.git-common-root"
LAYOUT_VERSION_LABEL = "frb.dev.layout-version"
REPO_LABEL_VALUE = "flutter_rust_bridge"
LAYOUT_VERSION_VALUE = "3"
TART_WORKSPACE_SHARE_NAME = "workspace"
TART_HOST_MAIN_SHARE_NAME = "main"
TART_LOCAL_COPY_ROOT = Path("/Users/admin/frb-dev-env-local-copies")
TART_LOCAL_COPY_EXCLUDES = [
    ".dart_tool",
    ".git",
    ".idea",
    ".vscode",
    "build",
    "target",
]


# ========== Common ==========


class CommandError(RuntimeError):
    pass


def exec_command(
    cmd: list[str],
    *,
    capture_output: bool = False,
    env: dict[str, str] | None = None,
) -> str | None:
    print(f"EXEC: {' '.join(cmd)}", file=sys.stderr, flush=True)

    try:
        result = subprocess.run(
            cmd,
            check=True,
            capture_output=capture_output,
            text=capture_output,
            env=env,
        )
    except subprocess.CalledProcessError as error:
        if capture_output:
            print(
                f"FAILED: stdout={error.stdout} stderr={error.stderr}",
                file=sys.stderr,
                flush=True,
            )
        raise CommandError(f"Command failed: {' '.join(cmd)}") from error

    if capture_output:
        return result.stdout.strip()

    return None


def try_exec_command(cmd: list[str], *, capture_output: bool = False) -> str | None:
    try:
        return exec_command(cmd, capture_output=capture_output)
    except CommandError:
        return None


def resolve_worktree_root(worktree_root: Path | None) -> Path:
    if worktree_root is not None:
        return worktree_root.expanduser().resolve()

    output = exec_command(
        ["git", "rev-parse", "--show-toplevel"],
        capture_output=True,
    )
    if output is None:
        raise CommandError("Unable to resolve git repository root")

    return Path(output).resolve()


def container_name_for_worktree(worktree_root: Path) -> str:
    digest = hashlib.sha256(str(worktree_root).encode()).hexdigest()[:12]
    return f"frb-{digest}"


def docker_volume_args(*, worktree_root: Path) -> list[str]:
    git_common_root = resolve_git_common_root(worktree_root=worktree_root)
    volumes = [
        "--volume",
        f"{worktree_root}:{worktree_root}",
    ]
    if git_common_root != worktree_root:
        volumes.extend(
            [
                "--volume",
                f"{git_common_root}:{git_common_root}",
            ]
        )

    return volumes


def get_image(image: str | None) -> str:
    if image is not None:
        return image

    return os.environ.get("FRB_DOCKER_IMAGE", DEFAULT_IMAGE)


def get_android_adb_server_host(host: str | None) -> str:
    if host is not None:
        return host

    return os.environ.get("FRB_ANDROID_ADB_SERVER_HOST", DEFAULT_ANDROID_ADB_SERVER_HOST)


def get_android_adb_server_port(port: int | None) -> int:
    if port is not None:
        return port

    value = os.environ.get("FRB_ANDROID_ADB_SERVER_PORT")
    if value is None:
        return DEFAULT_ANDROID_ADB_SERVER_PORT

    try:
        return int(value)
    except ValueError as error:
        raise CommandError(f"FRB_ANDROID_ADB_SERVER_PORT must be an integer: {value}") from error


def resolve_android_home() -> Path:
    resolved_android_home = env_path("ANDROID_HOME")
    if resolved_android_home is None:
        resolved_android_home = DEFAULT_MACOS_ANDROID_HOME.expanduser().resolve()

    if not resolved_android_home.exists():
        raise CommandError(
            f"Android SDK root does not exist: {resolved_android_home}. "
            "Install Android Studio or set ANDROID_HOME to the host Android SDK root."
        )

    return resolved_android_home


def android_host_env() -> dict[str, str]:
    resolved_android_home = resolve_android_home()

    env = dict(os.environ)
    env["ANDROID_HOME"] = str(resolved_android_home)
    env["ANDROID_SDK_ROOT"] = str(resolved_android_home)

    path_prefix = [
        resolved_android_home / "emulator",
        resolved_android_home / "platform-tools",
        resolved_android_home / "cmdline-tools" / "latest" / "bin",
    ]
    env["PATH"] = os.pathsep.join([*(str(path) for path in path_prefix), env.get("PATH", "")])

    return env


def env_path(name: str) -> Path | None:
    value = os.environ.get(name)
    if value is None or value == "":
        return None

    return Path(value).expanduser().resolve()


def print_android_host_env() -> None:
    android_home = resolve_android_home()
    typer.echo(f"export ANDROID_HOME={shlex.quote(str(android_home))}")
    typer.echo(f"export ANDROID_SDK_ROOT={shlex.quote(str(android_home))}")
    path_prefix = (
        f"{android_home / 'emulator'}:"
        f"{android_home / 'platform-tools'}:"
        f"{android_home / 'cmdline-tools' / 'latest' / 'bin'}"
    )
    typer.echo(
        f"export PATH={shlex.quote(path_prefix)}:" + '"$PATH"'
    )


# ========== Android commands ==========


@android_app.command(name="env")
def android_env() -> None:
    """Print shell exports for the host Android SDK and emulator environment."""

    print_android_host_env()


@android_app.command(name="emulator")
def android_emulator(
    avd: Annotated[
        str,
        typer.Option("--avd", help="AVD name to boot on the host."),
    ],
    port: Annotated[
        int,
        typer.Option("--port", help="Even emulator console port; the ADB serial becomes emulator-<port>."),
    ] = DEFAULT_ANDROID_EMULATOR_PORT,
    emulator_args: Annotated[
        list[str] | None,
        typer.Argument(help="Additional arguments passed to emulator after the generated -avd/-port flags."),
    ] = None,
) -> None:
    """Start the host Android Emulator with the standard Android SDK locations."""

    exec_command(
        [
            "emulator",
            "-avd",
            avd,
            "-port",
            str(port),
            *(emulator_args or []),
        ],
        env=android_host_env(),
    )


@android_app.command(name="adb-server")
def android_adb_server(
    bind_address: Annotated[
        str,
        typer.Option("--bind-address", help="Host address where the ADB server listens for Docker clients."),
    ] = DEFAULT_ANDROID_ADB_SERVER_BIND_ADDRESS,
    port: Annotated[
        int,
        typer.Option("--port", help="Host ADB server port for Docker clients."),
    ] = DEFAULT_ANDROID_ADB_SERVER_PORT,
    kill_existing: Annotated[
        bool,
        typer.Option("--kill-existing/--no-kill-existing", help="Kill any existing ADB server before starting this one."),
    ] = True,
) -> None:
    """Start a foreground host ADB server that Docker can reach."""

    env = android_host_env()
    if kill_existing:
        exec_command(["adb", "kill-server"], env=env)

    exec_command(
        [
            "adb",
            "-a",
            "-L",
            f"tcp:{bind_address}:{port}",
            "server",
            "nodaemon",
        ],
        env=env,
    )


# ========== Tart ==========


def tart_vm_name_for_worktree(worktree_root: Path) -> str:
    return f"frb-tart-{tart_worktree_digest(worktree_root=worktree_root)}"


def tart_worktree_digest(worktree_root: Path) -> str:
    return hashlib.sha256(str(worktree_root).encode()).hexdigest()[:12]


def tart_local_copy_path_for_worktree(worktree_root: Path) -> Path:
    return TART_LOCAL_COPY_ROOT / tart_worktree_digest(worktree_root=worktree_root)


def validate_tart_local_copy_root(local_copy_root: Path) -> Path:
    if not local_copy_root.is_absolute():
        raise CommandError(f"Tart local copy root must be an absolute VM path: {local_copy_root}")

    return local_copy_root


def tart_storage_root() -> Path:
    return Path(os.environ.get("FRB_TART_STORAGE_ROOT", "~/.tart")).expanduser()


def available_gb(path: Path) -> float:
    path.mkdir(parents=True, exist_ok=True)
    stats = os.statvfs(path)
    return stats.f_bavail * stats.f_frsize / 1024 / 1024 / 1024


def get_tart_base_vm(base_vm: str | None) -> str:
    if base_vm is not None:
        return base_vm

    return os.environ.get("FRB_TART_BASE_VM", DEFAULT_TART_BASE_VM)


def resolve_git_common_root(worktree_root: Path) -> Path:
    output = exec_command(
        ["git", "-C", str(worktree_root), "rev-parse", "--git-common-dir"],
        capture_output=True,
    )
    if output is None:
        raise CommandError(f"Unable to resolve git common directory for {worktree_root}")

    git_common_dir = Path(output)
    if not git_common_dir.is_absolute():
        git_common_dir = worktree_root / git_common_dir

    return git_common_dir.resolve().parent


def tart_get(vm_name: str) -> dict[str, object] | None:
    output = try_exec_command(
        ["tart", "get", vm_name, "--format", "json"],
        capture_output=True,
    )
    if output is None:
        return None

    return json.loads(output)


def tart_vm_exists(vm_name: str) -> bool:
    return tart_get(vm_name=vm_name) is not None


def tart_vm_is_running(vm_name: str) -> bool:
    details = tart_get(vm_name=vm_name)
    return bool(details and details.get("Running"))


def ensure_tart_free_space(*, storage_root: Path, min_free_gb: int) -> float:
    free_gb = available_gb(path=storage_root)
    if free_gb < min_free_gb:
        raise CommandError(
            f"Not enough free space for Tart VM creation: {free_gb:.1f}GB free "
            f"under {storage_root}, need at least {min_free_gb}GB."
        )

    return free_gb


def build_tart_info(*, worktree_root: Path, base_vm: str, min_free_gb: int) -> dict[str, object]:
    vm_name = tart_vm_name_for_worktree(worktree_root=worktree_root)
    storage_root = tart_storage_root()
    tart_available = True
    tart_error: str | None = None
    details: dict[str, object] | None = None
    base_details: dict[str, object] | None = None

    try:
        details = tart_get(vm_name=vm_name)
        base_details = tart_get(vm_name=base_vm)
    except CommandError as error:
        tart_available = False
        tart_error = str(error)

    return {
        "worktree_root": str(worktree_root),
        "vm_name": vm_name,
        "base_vm": base_vm,
        "storage_root": str(storage_root),
        "available_gb": round(available_gb(path=storage_root), 1),
        "min_create_free_gb": min_free_gb,
        "tart_available": tart_available,
        "tart_error": tart_error,
        "exists": details is not None,
        "running": bool(details and details.get("Running")),
        "details": details,
        "base_exists": base_details is not None,
        "base_details": base_details,
        "local_copy_path": str(tart_local_copy_path_for_worktree(worktree_root=worktree_root)),
        "local_copy_excludes": TART_LOCAL_COPY_EXCLUDES,
    }


def ensure_tart_vm(*, worktree_root: Path, base_vm: str, min_free_gb: int) -> str:
    vm_name = tart_vm_name_for_worktree(worktree_root=worktree_root)

    if tart_vm_exists(vm_name=vm_name):
        return vm_name

    if not tart_vm_exists(vm_name=base_vm):
        raise CommandError(
            f'Tart base VM "{base_vm}" does not exist. Create or clone it first, '
            "or set FRB_TART_BASE_VM / --base-vm to another prepared macOS VM."
        )

    storage_root = tart_storage_root()
    free_gb = ensure_tart_free_space(
        storage_root=storage_root,
        min_free_gb=min_free_gb,
    )
    typer.echo(
        f"Creating Tart VM {vm_name} from {base_vm} "
        f"({free_gb:.1f}GB free under {storage_root})",
        err=True,
    )

    exec_command(["tart", "clone", base_vm, vm_name])
    return vm_name


def tart_shell_command(command: list[str], *, worktree_root: Path) -> list[str]:
    quoted_command = " ".join(shlex.quote(part) for part in command)
    return [
        "/bin/zsh",
        "-lc",
        f"cd {shlex.quote(str(worktree_root))} && exec {quoted_command}",
    ]


def ensure_tart_host_path_links(*, vm_name: str, worktree_root: Path) -> None:
    git_common_root = resolve_git_common_root(worktree_root=worktree_root)
    link_command = "\n".join(
        [
            "set -euo pipefail",
            "ensure_mount() {",
            "  local tag=$1",
            "  local mount_point=$2",
            '  if [ -L "$mount_point" ]; then',
            '    local archive_root="${TMPDIR:-/tmp}/frb-dev-env-replaced-mount-points"',
            '    local archive_name="$(date +%Y%m%d-%H%M%S)-$(echo "$mount_point" | sed "s#[ /]#_#g")"',
            '    sudo -n mkdir -p "$archive_root"',
            '    sudo -n mv "$mount_point" "$archive_root/$archive_name"',
            "  fi",
            '  sudo -n mkdir -p "$mount_point"',
            '  if ! mount | grep -F " on $mount_point " >/dev/null; then',
            '    sudo -n mount_virtiofs "$tag" "$mount_point"',
            "  fi",
            "}",
            f"ensure_mount {shlex.quote(TART_HOST_MAIN_SHARE_NAME)} {shlex.quote(str(git_common_root))}",
            f"ensure_mount {shlex.quote(TART_WORKSPACE_SHARE_NAME)} {shlex.quote(str(worktree_root))}",
        ]
    )
    exec_command(["tart", "exec", vm_name, "/bin/zsh", "-lc", link_command])


def start_tart_vm(
    *,
    vm_name: str,
    worktree_root: Path,
    log_path: Path | None,
) -> None:
    if tart_vm_is_running(vm_name=vm_name):
        return

    resolved_log_path = log_path or Path(f"/tmp/{vm_name}.log")
    resolved_log_path.parent.mkdir(parents=True, exist_ok=True)
    typer.echo(f"Starting Tart VM {vm_name}; log: {resolved_log_path}", err=True)

    with resolved_log_path.open("ab") as output:
        subprocess.Popen(
            [
                "tart",
                "run",
                "--no-graphics",
                f"--dir={worktree_root}:tag={TART_WORKSPACE_SHARE_NAME}",
                f"--dir={resolve_git_common_root(worktree_root=worktree_root)}:tag={TART_HOST_MAIN_SHARE_NAME}",
                vm_name,
            ],
            stdout=output,
            stderr=subprocess.STDOUT,
            start_new_session=True,
        )


def wait_for_tart_ip(*, vm_name: str, wait_seconds: int) -> str:
    start_time = time.monotonic()
    output = exec_command(
        ["tart", "ip", vm_name, "--wait", str(wait_seconds)],
        capture_output=True,
    )
    if output is None:
        raise CommandError(f'Unable to resolve IP for Tart VM "{vm_name}"')

    elapsed = time.monotonic() - start_time
    typer.echo(f"Resolved {vm_name} IP in {elapsed:.1f}s", err=True)
    return output


def wait_for_tart_running(*, vm_name: str, wait_seconds: int) -> None:
    deadline = time.monotonic() + wait_seconds
    while time.monotonic() < deadline:
        if tart_vm_is_running(vm_name=vm_name):
            return
        time.sleep(1)

    raise CommandError(f'Tart VM "{vm_name}" did not become running within {wait_seconds}s')


def ensure_tart_vm_running(
    *,
    worktree_root: Path,
    base_vm: str,
    min_free_gb: int,
    log_path: Path | None,
    wait: int,
) -> str:
    vm_name = ensure_tart_vm(
        worktree_root=worktree_root,
        base_vm=base_vm,
        min_free_gb=min_free_gb,
    )
    if not tart_vm_is_running(vm_name=vm_name):
        start_tart_vm(
            vm_name=vm_name,
            worktree_root=worktree_root,
            log_path=log_path,
        )
        wait_for_tart_running(vm_name=vm_name, wait_seconds=wait)
        wait_for_tart_ip(vm_name=vm_name, wait_seconds=wait)
    ensure_tart_host_path_links(vm_name=vm_name, worktree_root=worktree_root)

    return vm_name


def upload_tart_local_copy(
    *,
    vm_name: str,
    worktree_root: Path,
    local_copy_root: Path,
) -> Path:
    local_copy_path = local_copy_root / tart_worktree_digest(worktree_root=worktree_root)
    exclude_args = " ".join(
        f"--exclude {shlex.quote(exclude)}"
        for exclude in TART_LOCAL_COPY_EXCLUDES
    )
    upload_command = "\n".join(
        [
            "set -euo pipefail",
            f"source_dir={shlex.quote(str(worktree_root))}",
            f"copy_root={shlex.quote(str(local_copy_root))}",
            f"copy_dir={shlex.quote(str(local_copy_path))}",
            'if [ -e "$copy_dir" ] && [ ! -d "$copy_dir" ]; then',
            '  archive_root="${TMPDIR:-/tmp}/frb-dev-env-replaced-local-copies"',
            '  archive_name="$(date +%Y%m%d-%H%M%S)-$(basename "$copy_dir")"',
            '  mkdir -p "$archive_root"',
            '  mv "$copy_dir" "$archive_root/$archive_name"',
            "fi",
            'mkdir -p "$copy_root" "$copy_dir"',
            f'rsync -a {exclude_args} "$source_dir/" "$copy_dir/"',
        ]
    )
    exec_command(["tart", "exec", vm_name, "/bin/zsh", "-lc", upload_command])

    return local_copy_path


# ========== Docker ==========


def container_id_for_name(container_name: str) -> str | None:
    output = exec_command(
        ["docker", "ps", "-aq", "--filter", f"name=^/{container_name}$"],
        capture_output=True,
    )
    if not output:
        return None

    return output.splitlines()[0]


def container_is_running(container_name: str) -> bool:
    output = exec_command(
        ["docker", "inspect", "-f", "{{.State.Running}}", container_name],
        capture_output=True,
    )
    return output == "true"


def container_labels(container_name: str) -> dict[str, str]:
    output = exec_command(
        ["docker", "inspect", "-f", "{{json .Config.Labels}}", container_name],
        capture_output=True,
    )
    if output is None or output == "null":
        return {}

    labels = json.loads(output)
    return {str(key): str(value) for key, value in labels.items()}


def validate_existing_container(*, container_name: str, worktree_root: Path) -> None:
    labels = container_labels(container_name=container_name)
    expected_worktree_root = str(worktree_root)
    expected_git_common_root = str(resolve_git_common_root(worktree_root=worktree_root))

    if (
        labels.get(REPO_LABEL) != REPO_LABEL_VALUE
        or labels.get(WORKTREE_LABEL) != expected_worktree_root
        or labels.get(GIT_COMMON_ROOT_LABEL) != expected_git_common_root
        or labels.get(LAYOUT_VERSION_LABEL) != LAYOUT_VERSION_VALUE
    ):
        raise CommandError(
            "Existing container has unexpected labels: "
            f"{container_name}. Expected {REPO_LABEL}={REPO_LABEL_VALUE} "
            f"and {WORKTREE_LABEL}={expected_worktree_root} "
            f"and {GIT_COMMON_ROOT_LABEL}={expected_git_common_root} "
            f"and {LAYOUT_VERSION_LABEL}={LAYOUT_VERSION_VALUE}."
        )


def ensure_container(*, worktree_root: Path, image: str) -> str:
    container_name = container_name_for_worktree(worktree_root=worktree_root)

    if container_id_for_name(container_name=container_name) is None:
        exec_command(
            [
                "docker",
                "run",
                "--detach",
                "--name",
                container_name,
                "--label",
                f"{REPO_LABEL}={REPO_LABEL_VALUE}",
                "--label",
                f"{WORKTREE_LABEL}={worktree_root}",
                "--label",
                f"{GIT_COMMON_ROOT_LABEL}={resolve_git_common_root(worktree_root=worktree_root)}",
                "--label",
                f"{LAYOUT_VERSION_LABEL}={LAYOUT_VERSION_VALUE}",
                *docker_volume_args(worktree_root=worktree_root),
                "--workdir",
                str(worktree_root),
                image,
                "bash",
                "-lc",
                "sleep infinity",
            ]
        )
    else:
        validate_existing_container(
            container_name=container_name,
            worktree_root=worktree_root,
        )

        if not container_is_running(container_name=container_name):
            exec_command(["docker", "start", container_name])

    return container_name


def android_host_adb_env(*, host: str, port: int) -> dict[str, str]:
    return {
        "ADB_SERVER_SOCKET": f"tcp:{host}:{port}",
        "ANDROID_ADB_SERVER_ADDRESS": host,
        "ANDROID_ADB_SERVER_PORT": str(port),
    }


def docker_exec_env_args(env: dict[str, str]) -> list[str]:
    result: list[str] = []
    for key, value in env.items():
        result.extend(["--env", f"{key}={value}"])

    return result


def delete_container(*, worktree_root: Path, force: bool) -> str:
    container_name = container_name_for_worktree(worktree_root=worktree_root)

    if container_id_for_name(container_name=container_name) is None:
        return container_name

    if not force:
        validate_existing_container(
            container_name=container_name,
            worktree_root=worktree_root,
        )

    exec_command(["docker", "rm", "-f", container_name])
    return container_name


def build_info(*, worktree_root: Path, image: str) -> dict[str, object]:
    container_name = container_name_for_worktree(worktree_root=worktree_root)
    docker_available = True
    docker_error: str | None = None
    container_id: str | None = None
    exists = False
    running = False
    actual_labels: dict[str, str] = {}

    try:
        container_id = container_id_for_name(container_name=container_name)
        exists = container_id is not None
        running = container_is_running(container_name=container_name) if exists else False
        actual_labels = container_labels(container_name=container_name) if exists else {}
    except CommandError as error:
        docker_available = False
        docker_error = str(error)

    return {
        "worktree_root": str(worktree_root),
        "container_name": container_name,
        "container_id": container_id,
        "docker_available": docker_available,
        "docker_error": docker_error,
        "exists": exists,
        "running": running,
        "image": image,
        "command_workdir": str(worktree_root),
        "git_common_root": str(resolve_git_common_root(worktree_root=worktree_root)),
        "labels": {
            REPO_LABEL: REPO_LABEL_VALUE,
            WORKTREE_LABEL: str(worktree_root),
            GIT_COMMON_ROOT_LABEL: str(resolve_git_common_root(worktree_root=worktree_root)),
            LAYOUT_VERSION_LABEL: LAYOUT_VERSION_VALUE,
        },
        "actual_labels": actual_labels,
    }


# ========== Docker commands ==========


@docker_app.command()
def info(
    worktree_root: Annotated[
        Path | None,
        typer.Option("--worktree-root", help="FRB worktree root. Defaults to git root."),
    ] = None,
    image: Annotated[
        str | None,
        typer.Option("--image", help="Docker image. Defaults to FRB_DOCKER_IMAGE or the published FRB dev image."),
    ] = None,
    json_output: Annotated[
        bool,
        typer.Option("--json", help="Print machine-readable JSON."),
    ] = False,
) -> None:
    """Show the worktree container that this environment will use."""

    resolved_worktree_root = resolve_worktree_root(worktree_root=worktree_root)
    details = build_info(
        worktree_root=resolved_worktree_root,
        image=get_image(image=image),
    )

    if json_output:
        typer.echo(json.dumps(details, indent=2, sort_keys=True))
        return

    for key, value in details.items():
        if isinstance(value, dict):
            typer.echo(f"{key}:")
            for nested_key, nested_value in value.items():
                typer.echo(f"  {nested_key}: {nested_value}")
        else:
            typer.echo(f"{key}: {value}")


@docker_app.command()
def create(
    worktree_root: Annotated[
        Path | None,
        typer.Option("--worktree-root", help="FRB worktree root. Defaults to git root."),
    ] = None,
    image: Annotated[
        str | None,
        typer.Option("--image", help="Docker image. Defaults to FRB_DOCKER_IMAGE or the published FRB dev image."),
    ] = None,
) -> None:
    """Ensure the per-worktree Docker container exists and is running."""

    resolved_worktree_root = resolve_worktree_root(worktree_root=worktree_root)
    container_name = ensure_container(
        worktree_root=resolved_worktree_root,
        image=get_image(image=image),
    )
    typer.echo(container_name)


@docker_app.command()
def delete(
    worktree_root: Annotated[
        Path | None,
        typer.Option("--worktree-root", help="FRB worktree root. Defaults to git root."),
    ] = None,
    force: Annotated[
        bool,
        typer.Option("--force", help="Delete even if the existing container labels do not match."),
    ] = False,
) -> None:
    """Delete the per-worktree Docker container to free local resources."""

    resolved_worktree_root = resolve_worktree_root(worktree_root=worktree_root)
    container_name = delete_container(
        worktree_root=resolved_worktree_root,
        force=force,
    )
    typer.echo(container_name)


@docker_app.command(name="exec")
def exec_in_container(
    command: Annotated[
        list[str],
        typer.Argument(help="Command to execute inside the worktree container."),
    ],
    worktree_root: Annotated[
        Path | None,
        typer.Option("--worktree-root", help="FRB worktree root. Defaults to git root."),
    ] = None,
    image: Annotated[
        str | None,
        typer.Option("--image", help="Docker image. Defaults to FRB_DOCKER_IMAGE or the published FRB dev image."),
    ] = None,
    android_host_adb: Annotated[
        bool,
        typer.Option("--android-host-adb", help="Connect Docker-side ADB clients to the host Android ADB server."),
    ] = False,
    android_adb_server_host: Annotated[
        str | None,
        typer.Option("--android-adb-server-host", help="Host ADB server address for --android-host-adb."),
    ] = None,
    android_adb_server_port: Annotated[
        int | None,
        typer.Option("--android-adb-server-port", help="Host ADB server port for --android-host-adb."),
    ] = None,
) -> None:
    """Ensure the container is running, then execute a command from the host-like worktree path."""

    if not command:
        raise typer.BadParameter("exec requires a command, for example: exec -- bash -lc './frb_internal --help'")

    resolved_worktree_root = resolve_worktree_root(worktree_root=worktree_root)
    container_name = ensure_container(
        worktree_root=resolved_worktree_root,
        image=get_image(image=image),
    )

    exec_flags = ["-i"]
    if sys.stdin.isatty() and sys.stdout.isatty():
        exec_flags = ["-it"]

    env: dict[str, str] = {}
    if android_host_adb:
        env.update(
            android_host_adb_env(
                host=get_android_adb_server_host(host=android_adb_server_host),
                port=get_android_adb_server_port(port=android_adb_server_port),
            )
        )

    exec_command(
        [
            "docker",
            "exec",
            *exec_flags,
            *docker_exec_env_args(env),
            "--workdir",
            str(resolved_worktree_root),
            container_name,
            *command,
        ]
    )


# ========== Tart commands ==========


@tart_app.command(name="info")
def tart_info(
    worktree_root: Annotated[
        Path | None,
        typer.Option("--worktree-root", help="FRB worktree root. Defaults to git root."),
    ] = None,
    base_vm: Annotated[
        str | None,
        typer.Option("--base-vm", help="Prepared Tart base VM. Defaults to FRB_TART_BASE_VM or frb-tart-base."),
    ] = None,
    min_free_gb: Annotated[
        int,
        typer.Option("--min-free-gb", help="Minimum free space required before creating a Tart VM."),
    ] = MIN_TART_FREE_GB,
    json_output: Annotated[
        bool,
        typer.Option("--json", help="Print machine-readable JSON."),
    ] = False,
) -> None:
    """Show the worktree Tart VM that this environment will use."""

    resolved_worktree_root = resolve_worktree_root(worktree_root=worktree_root)
    details = build_tart_info(
        worktree_root=resolved_worktree_root,
        base_vm=get_tart_base_vm(base_vm=base_vm),
        min_free_gb=min_free_gb,
    )

    if json_output:
        typer.echo(json.dumps(details, indent=2, sort_keys=True))
        return

    for key, value in details.items():
        if isinstance(value, dict):
            typer.echo(f"{key}:")
            for nested_key, nested_value in value.items():
                typer.echo(f"  {nested_key}: {nested_value}")
        else:
            typer.echo(f"{key}: {value}")


@tart_app.command(name="create")
def tart_create(
    worktree_root: Annotated[
        Path | None,
        typer.Option("--worktree-root", help="FRB worktree root. Defaults to git root."),
    ] = None,
    base_vm: Annotated[
        str | None,
        typer.Option("--base-vm", help="Prepared Tart base VM. Defaults to FRB_TART_BASE_VM or frb-tart-base."),
    ] = None,
    min_free_gb: Annotated[
        int,
        typer.Option("--min-free-gb", help="Minimum free space required before creating a Tart VM."),
    ] = MIN_TART_FREE_GB,
) -> None:
    """Ensure the per-worktree Tart VM exists."""

    resolved_worktree_root = resolve_worktree_root(worktree_root=worktree_root)
    vm_name = ensure_tart_vm(
        worktree_root=resolved_worktree_root,
        base_vm=get_tart_base_vm(base_vm=base_vm),
        min_free_gb=min_free_gb,
    )
    typer.echo(vm_name)


@tart_app.command(name="start")
def tart_start(
    worktree_root: Annotated[
        Path | None,
        typer.Option("--worktree-root", help="FRB worktree root. Defaults to git root."),
    ] = None,
    base_vm: Annotated[
        str | None,
        typer.Option("--base-vm", help="Prepared Tart base VM. Defaults to FRB_TART_BASE_VM or frb-tart-base."),
    ] = None,
    min_free_gb: Annotated[
        int,
        typer.Option("--min-free-gb", help="Minimum free space required before creating a Tart VM."),
    ] = MIN_TART_FREE_GB,
    log_path: Annotated[
        Path | None,
        typer.Option("--log-path", help="Where to write tart run output."),
    ] = None,
    wait: Annotated[
        int,
        typer.Option("--wait", help="Seconds to wait for the VM IP after starting."),
    ] = 180,
) -> None:
    """Ensure the per-worktree Tart VM is running."""

    resolved_worktree_root = resolve_worktree_root(worktree_root=worktree_root)
    vm_name = ensure_tart_vm(
        worktree_root=resolved_worktree_root,
        base_vm=get_tart_base_vm(base_vm=base_vm),
        min_free_gb=min_free_gb,
    )
    start_tart_vm(
        vm_name=vm_name,
        worktree_root=resolved_worktree_root,
        log_path=log_path,
    )
    wait_for_tart_running(vm_name=vm_name, wait_seconds=wait)
    ip_address = wait_for_tart_ip(vm_name=vm_name, wait_seconds=wait)
    ensure_tart_host_path_links(vm_name=vm_name, worktree_root=resolved_worktree_root)
    typer.echo(ip_address)


@tart_app.command(name="ip")
def tart_ip(
    worktree_root: Annotated[
        Path | None,
        typer.Option("--worktree-root", help="FRB worktree root. Defaults to git root."),
    ] = None,
    wait: Annotated[
        int,
        typer.Option("--wait", help="Seconds to wait for the VM IP."),
    ] = 0,
) -> None:
    """Print the per-worktree Tart VM IP address."""

    resolved_worktree_root = resolve_worktree_root(worktree_root=worktree_root)
    vm_name = tart_vm_name_for_worktree(worktree_root=resolved_worktree_root)
    typer.echo(wait_for_tart_ip(vm_name=vm_name, wait_seconds=wait))


@tart_app.command(name="stop")
def tart_stop(
    worktree_root: Annotated[
        Path | None,
        typer.Option("--worktree-root", help="FRB worktree root. Defaults to git root."),
    ] = None,
) -> None:
    """Stop the per-worktree Tart VM if it exists."""

    resolved_worktree_root = resolve_worktree_root(worktree_root=worktree_root)
    vm_name = tart_vm_name_for_worktree(worktree_root=resolved_worktree_root)
    if tart_vm_exists(vm_name=vm_name):
        exec_command(["tart", "stop", vm_name])
    typer.echo(vm_name)


@tart_app.command(name="delete")
def tart_delete(
    worktree_root: Annotated[
        Path | None,
        typer.Option("--worktree-root", help="FRB worktree root. Defaults to git root."),
    ] = None,
) -> None:
    """Delete the per-worktree Tart VM if it exists."""

    resolved_worktree_root = resolve_worktree_root(worktree_root=worktree_root)
    vm_name = tart_vm_name_for_worktree(worktree_root=resolved_worktree_root)
    if tart_vm_exists(vm_name=vm_name):
        if tart_vm_is_running(vm_name=vm_name):
            exec_command(["tart", "stop", vm_name])
        exec_command(["tart", "delete", vm_name])
    typer.echo(vm_name)


@tart_app.command(name="upload")
def tart_upload(
    worktree_root: Annotated[
        Path | None,
        typer.Option("--worktree-root", help="FRB worktree root. Defaults to git root."),
    ] = None,
    base_vm: Annotated[
        str | None,
        typer.Option("--base-vm", help="Prepared Tart base VM. Defaults to FRB_TART_BASE_VM or frb-tart-base."),
    ] = None,
    min_free_gb: Annotated[
        int,
        typer.Option("--min-free-gb", help="Minimum free space required before creating a Tart VM."),
    ] = MIN_TART_FREE_GB,
    log_path: Annotated[
        Path | None,
        typer.Option("--log-path", help="Where to write tart run output if the VM needs to start."),
    ] = None,
    wait: Annotated[
        int,
        typer.Option("--wait", help="Seconds to wait for the VM IP if the VM needs to start."),
    ] = 180,
    local_copy_root: Annotated[
        Path,
        typer.Option("--local-copy-root", help="VM-local root for uploaded worktree copies."),
    ] = TART_LOCAL_COPY_ROOT,
) -> None:
    """Upload the current worktree into a VM-local copy for heavy build/test commands."""

    resolved_worktree_root = resolve_worktree_root(worktree_root=worktree_root)
    vm_name = ensure_tart_vm_running(
        worktree_root=resolved_worktree_root,
        base_vm=get_tart_base_vm(base_vm=base_vm),
        min_free_gb=min_free_gb,
        log_path=log_path,
        wait=wait,
    )
    local_copy_path = upload_tart_local_copy(
        vm_name=vm_name,
        worktree_root=resolved_worktree_root,
        local_copy_root=validate_tart_local_copy_root(local_copy_root=local_copy_root),
    )
    typer.echo(local_copy_path)


@tart_app.command(name="exec")
def tart_exec(
    command: Annotated[
        list[str],
        typer.Argument(help="Command to execute inside the worktree Tart VM."),
    ],
    worktree_root: Annotated[
        Path | None,
        typer.Option("--worktree-root", help="FRB worktree root. Defaults to git root."),
    ] = None,
    base_vm: Annotated[
        str | None,
        typer.Option("--base-vm", help="Prepared Tart base VM. Defaults to FRB_TART_BASE_VM or frb-tart-base."),
    ] = None,
    min_free_gb: Annotated[
        int,
        typer.Option("--min-free-gb", help="Minimum free space required before creating a Tart VM."),
    ] = MIN_TART_FREE_GB,
    log_path: Annotated[
        Path | None,
        typer.Option("--log-path", help="Where to write tart run output if the VM needs to start."),
    ] = None,
    wait: Annotated[
        int,
        typer.Option("--wait", help="Seconds to wait for the VM IP if the VM needs to start."),
    ] = 180,
    sync_code: Annotated[
        bool,
        typer.Option("--sync-code", help="Upload the worktree to a VM-local copy before executing."),
    ] = False,
    local_copy_root: Annotated[
        Path,
        typer.Option("--local-copy-root", help="VM-local root for uploaded worktree copies."),
    ] = TART_LOCAL_COPY_ROOT,
) -> None:
    """Ensure the Tart VM is running, then execute a command inside it."""

    if not command:
        raise typer.BadParameter("exec requires a command, for example: exec -- sw_vers")

    resolved_worktree_root = resolve_worktree_root(worktree_root=worktree_root)
    vm_name = ensure_tart_vm_running(
        worktree_root=resolved_worktree_root,
        base_vm=get_tart_base_vm(base_vm=base_vm),
        min_free_gb=min_free_gb,
        log_path=log_path,
        wait=wait,
    )
    command_worktree_root = resolved_worktree_root
    if sync_code:
        command_worktree_root = upload_tart_local_copy(
            vm_name=vm_name,
            worktree_root=resolved_worktree_root,
            local_copy_root=validate_tart_local_copy_root(local_copy_root=local_copy_root),
        )

    exec_command(
        [
            "tart",
            "exec",
            vm_name,
            *tart_shell_command(command, worktree_root=command_worktree_root),
        ]
    )


if __name__ == "__main__":
    try:
        app()
    except CommandError as error:
        typer.echo(f"Error: {error}", err=True)
        raise typer.Exit(1) from error
