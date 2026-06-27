import tools.find_ios_simulator_udid as subject


def test_find_matching_udids_returns_exact_runtime_match() -> None:
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

    actual = subject._find_matching_udids(
        query=subject.DeviceQuery(
            label="iPhone 16 Pro Max Simulator (18.6)",
            name="iPhone 16 Pro Max",
            version="18.6",
        ),
        devices=devices,
    )

    assert actual == ["exact"]


def test_find_matching_udids_falls_back_to_latest_same_iphone_family() -> None:
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

    actual = subject._find_matching_udids(
        query=subject.DeviceQuery(
            label="iPhone 16 Pro Max Simulator (18.6)",
            name="iPhone 16 Pro Max",
            version="18.6",
        ),
        devices=devices,
    )

    assert actual == ["newer"]


def test_find_matching_udids_does_not_fallback_for_unversioned_query() -> None:
    """Unversioned labels keep exact matching semantics."""
    devices = {
        "devices": {
            "com.apple.CoreSimulator.SimRuntime.iOS-26-5": [
                {"name": "iPhone 17 Pro Max", "udid": "fallback"}
            ],
        }
    }

    actual = subject._find_matching_udids(
        query=subject.DeviceQuery(
            label="iPhone 16 Pro Max",
            name="iPhone 16 Pro Max",
            version=None,
        ),
        devices=devices,
    )

    assert actual == []
