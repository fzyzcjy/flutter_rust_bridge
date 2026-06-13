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


def test_validate_publish_credentials_rejects_missing_credentials(
    tmp_path: Path,
    monkeypatch: pytest.MonkeyPatch,
) -> None:
    """Publish preflight fails before Docker when required credential files are absent."""

    monkeypatch.delenv("GH_TOKEN", raising=False)
    monkeypatch.delenv("GITHUB_TOKEN", raising=False)

    paths = frb_dev_env.PublishCredentialPaths(
        cargo_home=tmp_path / "cargo",
        gh_config_dir=None,
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


def test_resolve_publish_credential_paths_respects_pub_cache(
    tmp_path: Path,
    monkeypatch: pytest.MonkeyPatch,
) -> None:
    """Credential discovery respects a custom PUB_CACHE directory."""

    custom_pub_cache = tmp_path / "custom-pub-cache"
    custom_pub_cache.mkdir()
    monkeypatch.setenv("PUB_CACHE", str(custom_pub_cache))

    paths = frb_dev_env.resolve_publish_credential_paths(home=tmp_path)

    assert paths.pub_cache_dir == custom_pub_cache


def test_validate_publish_credentials_accepts_github_token(
    tmp_path: Path,
    monkeypatch: pytest.MonkeyPatch,
) -> None:
    """GitHub token env vars can replace a local gh config directory."""

    cargo_home = tmp_path / "cargo"
    cargo_home.mkdir()
    (cargo_home / "credentials").write_text("token = 'dummy'\n")
    pub_cache_dir = tmp_path / "pub-cache"
    pub_cache_dir.mkdir()
    (pub_cache_dir / "credentials.json").write_text("{}\n")
    monkeypatch.setenv("GH_TOKEN", "dummy-token")

    paths = frb_dev_env.PublishCredentialPaths(
        cargo_home=cargo_home,
        gh_config_dir=None,
        git_config=None,
        git_config_dir=None,
        pub_cache_dir=pub_cache_dir,
        dart_config_dir=None,
    )

    frb_dev_env.validate_publish_credentials(paths)


def test_build_docker_run_rm_command_mounts_publish_credentials(
    tmp_path: Path,
    monkeypatch: pytest.MonkeyPatch,
) -> None:
    """Publish credential mode adds read-only credential mounts."""

    monkeypatch.setenv("GH_TOKEN", "dummy-token")
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

    command = frb_dev_env.build_docker_run_rm_command(
        worktree_root=Path("/repo/flutter_rust_bridge"),
        git_common_root=Path("/repo/flutter_rust_bridge"),
        image="example/frb-dev:latest",
        command=["./frb_internal", "release"],
        with_publish_credentials=True,
        publish_credential_paths=paths,
    )

    assert command[:4] == ["docker", "run", "--rm", "-i"]
    assert "example/frb-dev:latest" in command
    assert "./frb_internal" in command
    assert "release" in command
    assert f"{cargo_home}:/frb-publish-host-credentials/cargo:ro" in command
    assert f"{gh_config_dir}:/frb-publish-host-credentials/gh:ro" in command
    assert f"{pub_cache_dir}:/frb-publish-host-credentials/pub-cache:ro" in command
    assert f"{git_config}:/frb-publish-host-credentials/gitconfig:ro" in command
    assert "--env" in command
    assert "GH_TOKEN" in command
    assert "dummy-token" not in command

    script = command[command.index("bash") + 2]
    assert "GitHub CLI (gh) is required in the Docker image" in script
    assert "gh auth status --hostname github.com" in script
    assert "gh auth setup-git --hostname github.com" in script
    assert 'test -s "$CARGO_HOME/credentials.toml"' in script
    assert 'test -s "$PUB_CACHE/credentials.json"' in script


def test_build_docker_run_rm_command_without_credentials_is_plain() -> None:
    """Plain run-rm mode does not add publish credential bootstrap."""

    command = frb_dev_env.build_docker_run_rm_command(
        worktree_root=Path("/repo/flutter_rust_bridge"),
        git_common_root=Path("/repo/flutter_rust_bridge"),
        image="example/frb-dev:latest",
        command=["./frb_internal", "--help"],
        with_publish_credentials=False,
        publish_credential_paths=None,
    )

    assert command == [
        "docker",
        "run",
        "--rm",
        "-i",
        "--volume",
        "/repo/flutter_rust_bridge:/repo/flutter_rust_bridge",
        "--workdir",
        "/repo/flutter_rust_bridge",
        "example/frb-dev:latest",
        "./frb_internal",
        "--help",
    ]
