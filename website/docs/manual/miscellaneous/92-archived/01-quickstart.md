# Quickstart

Write down Rust functions and types normally.

```rust,ignore
// A normal Rust function ...
pub fn draw_tree(root: TreeNode, mode: DrawMode) -> Result<Vec<u8>> { /* ... */ }

// ... with rich types
pub struct TreeNode { pub value: String, pub children: Vec<MyTreeNode> }
pub enum DrawMode { Colorful {palette: String}, Grayscale }
```

Install the code generator `flutter_rust_bridge_codegen`:

```bash
cargo install flutter_rust_bridge_codegen
# or with cargo-binstall
cargo binstall flutter_rust_bridge_codegen
# or with scoop (Windows)
scoop bucket add frb https://github.com/Desdaemon/scoop-repo
scoop install flutter_rust_bridge_codegen
# or with Homebrew
brew install desdaemon/repo/flutter_rust_bridge_codegen
```

<small>(Remark: Thanks @Desdaemon for scripts to publish to brew/scoop)</small>

Then run the code generator.

<small>Remark: It needs some installation steps. You may refer to `the tutorial`, `create new projects from a template` or `integrating with existing projects` for details.</small>

```bash
flutter_rust_bridge_codegen --rust-input path/to/api.rs \
                            --dart-output path/to/bridge_generated.dart
```

With bindings automatically generated, use it seamlessly in Flutter/Dart:

```dart
api.drawTree(TreeNode(value: "root", ...), Colorful(palette: "viridis"));
```

