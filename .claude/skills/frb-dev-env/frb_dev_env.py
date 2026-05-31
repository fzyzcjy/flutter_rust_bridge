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
from pathlib import Path
from typing import Annotated

import typer


app = typer.Typer(no_args_is_help=True)

DEFAULT_IMAGE = "fzyzcjy/flutter_rust_bridge_dev:latest"
REPO_LABEL = "frb.dev.repo"
WORKTREE_LABEL = "frb.dev.worktree"
REPO_LABEL_VALUE = "flutter_rust_bridge"
WORKSPACE_PATH = "/workspace"


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


def get_image(image: str | None) -> str:
    if image is not None:
        return image

    return os.environ.get("FRB_DOCKER_IMAGE", DEFAULT_IMAGE)


@app.command()
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


@app.command()
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


@app.command()
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


@app.command(name="exec")
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


if __name__ == "__main__":
    try:
        app()
    except CommandError as error:
        typer.echo(f"Error: {error}", err=True)
        raise typer.Exit(1) from error
