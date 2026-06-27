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
    if query.label == "iPhone Simulator":
        return _find_available_iphone_udids(devices=devices)

    matches: list[str] = []
    for runtime, runtime_devices in devices.get("devices", {}).items():
        if query.version is not None and not runtime.endswith(
            f"iOS-{query.version.replace('.', '-')}"
        ):
            continue

        for runtime_device in runtime_devices:
            if runtime_device.get("name") == query.name:
                matches.append(runtime_device["udid"])

    return matches


def _find_available_iphone_udids(devices: dict[str, Any]) -> list[str]:
    pro_max_matches: list[str] = []
    iphone_matches: list[str] = []
    for runtime_devices in devices.get("devices", {}).values():
        for runtime_device in runtime_devices:
            name = runtime_device.get("name")
            udid = runtime_device.get("udid")
            if not isinstance(name, str) or not isinstance(udid, str):
                continue
            if not name.startswith("iPhone "):
                continue

            iphone_matches.append(udid)
            if " Pro Max" in name:
                pro_max_matches.append(udid)

    if pro_max_matches:
        return pro_max_matches[:1]

    return iphone_matches[:1]


if __name__ == "__main__":
    raise SystemExit(main())
