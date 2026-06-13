# URL and URI

Codegen supports URL-like types from the [`url`](https://docs.rs/url) and [`uriparse`](https://docs.rs/uriparse) crates.

| :crab: Rust             | :dart: Dart |
| ----------------------- | ----------- |
| `url::Url`              | `Uri`       |
| `uriparse::URI<'a>`     | `Uri`       |

The values are serialized as strings across the FFI boundary. On the Dart side, generated code uses `Uri.parse` and `Uri.toString`.

## Setup

Add the crates to your Rust dependencies:

```toml
[dependencies]
url = "2"
uriparse = "0.6"
```

## Example

```rust
pub fn echo_url(url: url::Url) -> url::Url {
    url
}

pub fn echo_uri(uri: uriparse::URI<'static>) -> uriparse::URI<'static> {
    uri
}
```

```dart
final url = await echoUrl(url: Uri.parse('https://example.com/path?q=1'));
final uri = await echoUri(uri: Uri.parse('urn:isbn:0451450523'));
```
