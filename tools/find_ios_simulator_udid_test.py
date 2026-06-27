import json
import sys

import tools.find_ios_simulator_udid as subject


def test_find_matching_devices_returns_exact_runtime_match() -> None:
    """Exact simulator name and runtime match is used when present."""
    devices = {
        "devices": {
            "com.apple.CoreSimulator.SimRuntime.iOS-18-6": [
                {"name": "iPhone 16 Pro Max", "udid": "exact"}
            ],
            "com.apple.CoreSimulator.SimRuntime.iOS-26-5": [
                {"name": "iPhone 17 Pro Max", "udid": "fallback"}
            ],
        }
    }

    actual = subject._find_matching_devices(
        query=subject.DeviceQuery(
            label="iPhone 16 Pro Max Simulator (18.6)",
            name="iPhone 16 Pro Max",
            version="18.6",
        ),
        devices=devices,
    )

    assert actual == [
        subject.DeviceMatch(name="iPhone 16 Pro Max", udid="exact", version="18.6")
    ]


def test_find_matching_devices_falls_back_to_latest_same_named_runtime() -> None:
    """Missing iOS runtime falls back to newest runtime for the exact device name."""
    devices = {
        "devices": {
            "com.apple.CoreSimulator.SimRuntime.iOS-26-2": [
                {"name": "iPhone 16 Pro Max", "udid": "older"}
            ],
            "com.apple.CoreSimulator.SimRuntime.iOS-26-5": [
                {"name": "iPhone 16 Pro Max", "udid": "newer"}
            ],
        }
    }

    actual = subject._find_matching_devices(
        query=subject.DeviceQuery(
            label="iPhone 16 Pro Max Simulator (18.6)",
            name="iPhone 16 Pro Max",
            version="18.6",
        ),
        devices=devices,
    )

    assert actual == [
        subject.DeviceMatch(name="iPhone 16 Pro Max", udid="newer", version="26.5")
    ]


def test_find_matching_devices_falls_back_to_latest_same_iphone_family() -> None:
    """Missing iOS runtime falls back to the newest matching iPhone family."""
    devices = {
        "devices": {
            "com.apple.CoreSimulator.SimRuntime.iOS-26-2": [
                {"name": "iPhone 17 Pro Max", "udid": "older"}
            ],
            "com.apple.CoreSimulator.SimRuntime.iOS-26-5": [
                {"name": "iPhone 17 Pro Max", "udid": "newer"},
                {"name": "iPhone 17 Pro", "udid": "wrong-family"},
            ],
        }
    }

    actual = subject._find_matching_devices(
        query=subject.DeviceQuery(
            label="iPhone 16 Pro Max Simulator (18.6)",
            name="iPhone 16 Pro Max",
            version="18.6",
        ),
        devices=devices,
    )

    assert actual == [
        subject.DeviceMatch(name="iPhone 17 Pro Max", udid="newer", version="26.5")
    ]


def test_find_matching_devices_does_not_fallback_for_unversioned_query() -> None:
    """Unversioned labels keep exact matching semantics."""
    devices = {
        "devices": {
            "com.apple.CoreSimulator.SimRuntime.iOS-26-5": [
                {"name": "iPhone 17 Pro Max", "udid": "fallback"}
            ],
        }
    }

    actual = subject._find_matching_devices(
        query=subject.DeviceQuery(
            label="iPhone 16 Pro Max",
            name="iPhone 16 Pro Max",
            version=None,
        ),
        devices=devices,
    )

    assert actual == []


def test_main_reports_actual_fallback_device_name(tmp_path, capsys, monkeypatch) -> None:
    """CLI warning reports the concrete fallback simulator name."""
    simctl_json = tmp_path / "simctl.json"
    simctl_json.write_text(
        json.dumps(
            {
                "devices": {
                    "com.apple.CoreSimulator.SimRuntime.iOS-26-5": [
                        {"name": "iPhone 17 Pro Max", "udid": "newer"}
                    ],
                }
            }
        )
    )
    monkeypatch.setattr(
        sys,
        "argv",
        [
            "find_ios_simulator_udid.py",
            "iPhone 16 Pro Max Simulator (18.6)",
            "--simctl-json",
            str(simctl_json),
        ],
    )

    actual = subject.main()

    captured = capsys.readouterr()
    assert actual == 0
    assert captured.out == "newer\n"
    assert "using iPhone 17 Pro Max Simulator (26.5)" in captured.err
