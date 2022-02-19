# Quickstart

Write down Rust functions and types normally.

```rust,ignore
// A normal Rust function ...
pub fn draw_tree(root: TreeNode, mode: DrawMode) -> Result<Vec<u8>> { /* ... */ }

// ... with rich types
pub struct TreeNode { pub value: String, pub children: Vec<MyTreeNode> }
pub enum DrawMode { Colorful {palette: String}, Grayscale }
```

Run the code generator. (It needs some installation steps. You may refer to [the tutorial](tutorial_with_flutter.md), [create new projects from a template](template.md) or [integrating with existing projects](integrate.md) for details.)

```
flutter_rust_bridge_codegen --rust-input path/to/api.rs --dart-output path/to/bridge_generated.dart
```

With bindings automatically generated, use it seamlessly in Flutter/Dart:

```dart
api.drawTree(TreeNode(value: "root", ...), Colorful(palette: "viridis"));
```

