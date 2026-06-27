#!/usr/bin/env python3

import json
import re
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Optional


@dataclass(frozen=True)
class DeviceQuery:
    label: str
    name: str
    version: Optional[str]


@dataclass(frozen=True)
class DeviceMatch:
    name: str
    udid: str
    version: str


def main() -> int:
    device_label, simctl_json_path = _parse_args(args=sys.argv[1:])
    query = _parse_device_label(label=device_label)
    devices = _load_simctl_devices(simctl_json_path=simctl_json_path)
    matches = _find_matching_devices(query=query, devices=devices)

    if len(matches) != 1:
        print(
            f"Expected exactly one simulator for {query.label}, found {len(matches)}",
            file=sys.stderr,
        )
        return 1

    match = matches[0]
    if query.version is not None and query.version != match.version:
        print(
            f"Simulator {query.label} is unavailable; using {match.name} "
            f"Simulator ({match.version})",
            file=sys.stderr,
        )

    print(match.udid)
    return 0


def _parse_args(args: list[str]) -> tuple[str, Optional[Path]]:
    if not args or len(args) not in {1, 3}:
        _print_usage()
        raise SystemExit(2)

    if len(args) == 1:
        return args[0], None

    if args[1] != "--simctl-json":
        _print_usage()
        raise SystemExit(2)

    return args[0], Path(args[2])


def _print_usage() -> None:
    print(
        "Usage: find_ios_simulator_udid.py '<device label>' "
        "[--simctl-json path]",
        file=sys.stderr,
    )


def _parse_device_label(label: str) -> DeviceQuery:
    match = re.fullmatch(r"(.+) Simulator \(([^()]+)\)", label)
    if match is None:
        return DeviceQuery(label=label, name=label, version=None)

    return DeviceQuery(label=label, name=match.group(1), version=match.group(2))


def _load_simctl_devices(simctl_json_path: Optional[Path]) -> dict[str, Any]:
    if simctl_json_path is not None:
        return json.loads(simctl_json_path.read_text())

    output = subprocess.check_output(
        ["xcrun", "simctl", "list", "devices", "available", "-j"],
        text=True,
    )
    return json.loads(output)


def _find_matching_devices(
    query: DeviceQuery,
    devices: dict[str, Any],
) -> list[DeviceMatch]:
    devices = _parse_simulator_devices(devices=devices)
    name_matches = [device for device in devices if device.name == query.name]

    exact_matches = [
        match
        for match in name_matches
        if query.version is None or match.version == query.version
    ]
    if exact_matches or query.version is None:
        return exact_matches

    if name_matches:
        latest_version = max(_version_key(match.version) for match in name_matches)
        return [
            match
            for match in name_matches
            if _version_key(match.version) == latest_version
        ]

    fallback = _find_fallback_device(query=query, devices=devices)
    return [] if fallback is None else [fallback]


def _parse_simulator_devices(devices: dict[str, Any]) -> list[DeviceMatch]:
    result: list[DeviceMatch] = []
    for runtime, runtime_devices in devices.get("devices", {}).items():
        runtime_version = _parse_ios_runtime_version(runtime=runtime)
        if runtime_version is None:
            continue

        result.extend(
            DeviceMatch(
                name=runtime_device["name"],
                udid=runtime_device["udid"],
                version=runtime_version,
            )
            for runtime_device in runtime_devices
        )

    return result


def _find_fallback_device(
    query: DeviceQuery,
    devices: list[DeviceMatch],
) -> Optional[DeviceMatch]:
    family_pattern = _build_fallback_family_pattern(name=query.name)
    if family_pattern is None:
        return None

    candidates = [
        match
        for match in devices
        if re.fullmatch(family_pattern, match.name) is not None
    ]
    if not candidates:
        return None

    return max(candidates, key=lambda match: (_version_key(match.version), match.name))


def _build_fallback_family_pattern(name: str) -> Optional[str]:
    match = re.fullmatch(r"(iPhone) \d+(.*)", name)
    if match is None:
        return None

    return rf"{re.escape(match.group(1))} \d+{re.escape(match.group(2))}"


def _parse_ios_runtime_version(runtime: str) -> Optional[str]:
    prefix = "com.apple.CoreSimulator.SimRuntime.iOS-"
    if not runtime.startswith(prefix):
        return None

    return runtime.removeprefix(prefix).replace("-", ".")


def _version_key(version: str) -> tuple[int, ...]:
    return tuple(int(part) for part in version.split("."))


if __name__ == "__main__":
    raise SystemExit(main())
