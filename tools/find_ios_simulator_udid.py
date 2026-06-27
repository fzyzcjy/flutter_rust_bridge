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
class SimulatorDevice:
    name: str
    version: str
    udid: str


def main() -> int:
    device_label, simctl_json_path = _parse_args(args=sys.argv[1:])
    query = _parse_device_label(label=device_label)
    devices = _load_simctl_devices(simctl_json_path=simctl_json_path)
    matches = _find_matching_udids(query=query, devices=devices)

    if len(matches) != 1:
        print(
            f"Expected exactly one simulator for {query.label}, found {len(matches)}",
            file=sys.stderr,
        )
        return 1

    print(matches[0])
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


def _find_matching_udids(
    query: DeviceQuery,
    devices: dict[str, Any],
) -> list[str]:
    simulator_devices = _parse_simulator_devices(devices=devices)
    matches = [
        device.udid
        for device in simulator_devices
        if device.name == query.name
        and (query.version is None or device.version == query.version)
    ]
    if matches or query.version is None:
        return matches

    fallback = _find_fallback_device(query=query, devices=simulator_devices)
    return [] if fallback is None else [fallback.udid]


def _parse_simulator_devices(devices: dict[str, Any]) -> list[SimulatorDevice]:
    result: list[SimulatorDevice] = []
    for runtime, runtime_devices in devices.get("devices", {}).items():
        version = _parse_ios_runtime_version(runtime=runtime)
        if version is None:
            continue

        for runtime_device in runtime_devices:
            result.append(
                SimulatorDevice(
                    name=runtime_device["name"],
                    version=version,
                    udid=runtime_device["udid"],
                )
            )

    return result


def _parse_ios_runtime_version(runtime: str) -> Optional[str]:
    match = re.search(r"iOS-([0-9-]+)$", runtime)
    if match is None:
        return None

    return match.group(1).replace("-", ".")


def _find_fallback_device(
    query: DeviceQuery,
    devices: list[SimulatorDevice],
) -> Optional[SimulatorDevice]:
    family_pattern = _build_fallback_family_pattern(name=query.name)
    if family_pattern is None:
        return None

    candidates = [
        device
        for device in devices
        if re.fullmatch(family_pattern, device.name) is not None
    ]
    if not candidates:
        return None

    return max(candidates, key=lambda device: (_version_key(device.version), device.name))


def _build_fallback_family_pattern(name: str) -> Optional[str]:
    match = re.fullmatch(r"(iPhone) \d+(.*)", name)
    if match is None:
        return None

    return rf"{re.escape(match.group(1))} \d+{re.escape(match.group(2))}"


def _version_key(version: str) -> tuple[int, ...]:
    return tuple(int(part) for part in version.split("."))


if __name__ == "__main__":
    raise SystemExit(main())
