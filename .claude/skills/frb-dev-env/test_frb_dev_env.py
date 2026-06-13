from __future__ import annotations

import importlib.util
import sys
from pathlib import Path
from typing import Any

import pytest


def load_frb_dev_env_module() -> Any:
    module_path = Path(__file__).with_name("frb_dev_env.py")
    spec = importlib.util.spec_from_file_location("frb_dev_env", module_path)
    assert spec is not None
    assert spec.loader is not None
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


frb_dev_env = load_frb_dev_env_module()


def test_validate_publish_credentials_rejects_missing_credentials(tmp_path: Path) -> None:
    """Publish preflight fails before Docker when required credential files are absent."""

    paths = frb_dev_env.PublishCredentialPaths(
        cargo_home=tmp_path / "cargo",
        gh_config_dir=tmp_path / "gh",
        git_config=None,
        git_config_dir=None,
        pub_cache_dir=None,
        dart_config_dir=None,
    )

    with pytest.raises(frb_dev_env.CommandError) as error:
        frb_dev_env.validate_publish_credentials(paths)

    message = str(error.value)
    assert "GitHub CLI config directory" in message
    assert "Cargo credentials file" in message
    assert "Dart pub credentials file" in message


def test_build_publish_container_command_mounts_credentials(tmp_path: Path) -> None:
    """Publish mode runs a temporary container with read-only credential mounts."""

    cargo_home = tmp_path / "cargo"
    cargo_home.mkdir()
    (cargo_home / "credentials").write_text("token = 'dummy'\n")
    gh_config_dir = tmp_path / "gh"
    gh_config_dir.mkdir()
    pub_cache_dir = tmp_path / "pub-cache"
    pub_cache_dir.mkdir()
    (pub_cache_dir / "credentials.json").write_text("{}\n")
    git_config = tmp_path / ".gitconfig"
    git_config.write_text("[user]\n  name = Test\n")

    paths = frb_dev_env.PublishCredentialPaths(
        cargo_home=cargo_home,
        gh_config_dir=gh_config_dir,
        git_config=git_config,
        git_config_dir=None,
        pub_cache_dir=pub_cache_dir,
        dart_config_dir=None,
    )

    command = frb_dev_env.build_publish_container_command(
        worktree_root=Path("/repo/flutter_rust_bridge"),
        git_common_root=Path("/repo/flutter_rust_bridge"),
        image="example/frb-dev:latest",
        command=["./frb_internal", "release"],
        preflight_only=False,
        paths=paths,
    )

    assert command[:4] == ["docker", "run", "--rm", "-i"]
    assert "example/frb-dev:latest" in command
    assert "./frb_internal" in command
    assert "release" in command
    assert f"{cargo_home}:/frb-publish-host-credentials/cargo:ro" in command
    assert f"{gh_config_dir}:/frb-publish-host-credentials/gh:ro" in command
    assert f"{pub_cache_dir}:/frb-publish-host-credentials/pub-cache:ro" in command
    assert f"{git_config}:/frb-publish-host-credentials/gitconfig:ro" in command

    script = command[command.index("bash") + 2]
    assert "gh auth status --hostname github.com" in script
    assert "gh auth setup-git --hostname github.com" in script
    assert 'test -s "$CARGO_HOME/credentials.toml"' in script
    assert 'test -s "$PUB_CACHE/credentials.json"' in script


def test_build_publish_preflight_command_does_not_append_release_command(
    tmp_path: Path,
) -> None:
    """Publish preflight only checks credentials and exits without release work."""

    paths = frb_dev_env.PublishCredentialPaths(
        cargo_home=tmp_path / "cargo",
        gh_config_dir=tmp_path / "gh",
        git_config=None,
        git_config_dir=None,
        pub_cache_dir=None,
        dart_config_dir=None,
    )

    command = frb_dev_env.build_publish_container_command(
        worktree_root=Path("/repo/flutter_rust_bridge"),
        git_common_root=Path("/repo/flutter_rust_bridge"),
        image="example/frb-dev:latest",
        command=["./frb_internal", "release"],
        preflight_only=True,
        paths=paths,
    )

    assert command[-1] == "frb-publish-container"
    assert "./frb_internal" not in command
    assert "release" not in command
    script = command[command.index("bash") + 2]
    assert script.endswith("exit 0")
