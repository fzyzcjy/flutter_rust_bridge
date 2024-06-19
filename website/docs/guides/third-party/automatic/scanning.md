# Scanning

The first step is to configure to scan the third-party crate.
This is fairly simple - just modify `flutter_rust_bridge.yaml` and change to something like:

```yaml
rust_input: crate::api,interesting_third_party_crate_name
```

The line above means we want to both scan `src/api` folder in our crate and scan the `interesting_third_party_crate_name` crate.

For crate with `-` in the name, we can write `interesting-third-party-crate-name`

Please refer to [this page](../../misc-features/multi-input) for more details of the configuration.
