# Using underlying data

You might wonder, since the Dart side is just a "pointer",
how to use the underlying data in Dart?
There are two answers.

## Through properties

For each public field, corresponding getters and setters will be auto generated, thus can be directy used.
Please refer to [this page](properties) for more details.

## Through functions/methods

In short,
just imagine those fields are private (to Rust) - the **"private fields" concept** you use everyday
to do encapsulation,
then write standard code.

For example, suppose we want to manipulate with the temporary directory object in the sample above,
then the rough code looks like:

```rust
pub struct MyTempDir {
    dir: tempdir::TempDir,
}

impl MyTempDir {
    pub fn new() -> Self { ... }

    pub fn directory_path(&self) -> String {
        self.dir.path()
    }

    pub fn read_text(&self, filename: String) -> String {
        fs::read_to_string(self.dir.path().join(filename))
    }

    // ...
}
```

These methods can be called in Dart as if normal Dart functions (code sketch as below):

```dart
var d = await MyTempDir.newMyTempDir();
print(await d.directoryPath());
print(await d.readText('a.txt'));
```
