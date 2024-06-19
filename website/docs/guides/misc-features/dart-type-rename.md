# Rename Dart types

The automatically generated type names can be customized.
For example, `Box<dyn Any + Send + Sync + 'static>` has corresponding Dart name as `BoxAny` by default.
Suppose we want to change it to `MyFancyName`, then we can configure as follows in `flutter_rust_bridge.yaml`:

```yaml
dart_type_rename:
    Box<dyn Any + Send + Sync + 'static>: MyFancyName
```
