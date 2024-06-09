# Scanning

The first step is to configure to scan the third-party crate.
This is fairly simple - just modify `flutter_rust_bridge.yaml` and change to something like:

```yaml
rust_input: crate::api,web-audio-api
```

The line above means we want to both scan `src/api` folder in our crate and scan the `web-audio-api` crate.

Please refer to [this page](../miscellaneous/multi-input) for more details of the configuration.
