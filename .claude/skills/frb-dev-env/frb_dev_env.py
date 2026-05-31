#!/usr/bin/env -S uv run --script
# /// script
# dependencies = ["typer>=0.12"]
# ///
from __future__ import annotations

import hashlib
import json
import os
import subprocess
import sys
import time
from pathlib import Path
from typing import Annotated

import typer


app = typer.Typer(no_args_is_help=True)
docker_app = typer.Typer(no_args_is_help=True)
tart_app = typer.Typer(no_args_is_help=True)
app.add_typer(docker_app, name="docker", help="Manage the per-worktree Docker development container.")
app.add_typer(tart_app, name="tart", help="Manage the per-worktree Tart macOS VM.")

DEFAULT_IMAGE = "fzyzcjy/flutter_rust_bridge_dev:latest"
DEFAULT_TART_BASE_VM = "frb-tart-base"
MIN_TART_FREE_GB = 150
REPO_LABEL = "frb.dev.repo"
WORKTREE_LABEL = "frb.dev.worktree"
REPO_LABEL_VALUE = "flutter_rust_bridge"
WORKSPACE_PATH = "/workspace"


# ========== Common ==========


class CommandError(RuntimeError):
    pass


def exec_command(cmd: list[str], *, capture_output: bool = False) -> str | None:
    print(f"EXEC: {' '.join(cmd)}", file=sys.stderr, flush=True)

    try:
        result = subprocess.run(
            cmd,
            check=True,
            capture_output=capture_output,
            text=capture_output,
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


def get_image(image: str | None) -> str:
    if image is not None:
        return image

    return os.environ.get("FRB_DOCKER_IMAGE", DEFAULT_IMAGE)


# ========== Tart ==========


def tart_vm_name_for_worktree(worktree_root: Path) -> str:
    digest = hashlib.sha256(str(worktree_root).encode()).hexdigest()[:12]
    return f"frb-tart-{digest}"


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


def start_tart_vm(*, vm_name: str, log_path: Path | None) -> None:
    if tart_vm_is_running(vm_name=vm_name):
        return

    resolved_log_path = log_path or Path(f"/tmp/{vm_name}.log")
    resolved_log_path.parent.mkdir(parents=True, exist_ok=True)
    typer.echo(f"Starting Tart VM {vm_name}; log: {resolved_log_path}", err=True)

    with resolved_log_path.open("ab") as output:
        subprocess.Popen(
            ["tart", "run", "--no-graphics", vm_name],
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

    if labels.get(REPO_LABEL) != REPO_LABEL_VALUE or labels.get(WORKTREE_LABEL) != expected_worktree_root:
        raise CommandError(
            "Existing container has unexpected labels: "
            f"{container_name}. Expected {REPO_LABEL}={REPO_LABEL_VALUE} "
            f"and {WORKTREE_LABEL}={expected_worktree_root}."
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
                "--volume",
                f"{worktree_root}:{WORKSPACE_PATH}",
                "--workdir",
                WORKSPACE_PATH,
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
        "workspace_path": WORKSPACE_PATH,
        "labels": {
            REPO_LABEL: REPO_LABEL_VALUE,
            WORKTREE_LABEL: str(worktree_root),
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
) -> None:
    """Ensure the container is running, then execute a command in /workspace."""

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

    exec_command(
        [
            "docker",
            "exec",
            *exec_flags,
            "--workdir",
            WORKSPACE_PATH,
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
    start_tart_vm(vm_name=vm_name, log_path=log_path)
    ip_address = wait_for_tart_ip(vm_name=vm_name, wait_seconds=wait)
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
) -> None:
    """Ensure the Tart VM is running, then execute a command inside it."""

    if not command:
        raise typer.BadParameter("exec requires a command, for example: exec -- sw_vers")

    resolved_worktree_root = resolve_worktree_root(worktree_root=worktree_root)
    vm_name = ensure_tart_vm(
        worktree_root=resolved_worktree_root,
        base_vm=get_tart_base_vm(base_vm=base_vm),
        min_free_gb=min_free_gb,
    )
    if not tart_vm_is_running(vm_name=vm_name):
        start_tart_vm(vm_name=vm_name, log_path=log_path)
        wait_for_tart_ip(vm_name=vm_name, wait_seconds=wait)

    exec_command(["tart", "exec", vm_name, *command])


if __name__ == "__main__":
    try:
        app()
    except CommandError as error:
        typer.echo(f"Error: {error}", err=True)
        raise typer.Exit(1) from error
