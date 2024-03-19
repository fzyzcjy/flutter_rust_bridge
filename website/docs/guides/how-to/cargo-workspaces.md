# Cargo Workspaces

You can add flutter_rust_bridge to an existing [Cargo Workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) by using the `--rust-crate-dir` option.

To create a new Flutter project inside the workspace, run:

```sh
flutter_rust_bridge_codegen create my_flutter_project --rust-crate-dir ../my_flutter_bridge
```

Where:

- `my_flutter_project` is the name of the Flutter project
- `my_flutter_bridge` is the name of the Rust project

You can also apply the codegen to an existing Flutter project inside the workspace:

```sh
cd my_flutter_project
flutter_rust_bridge_codegen integrate --rust-crate-dir ../my_flutter_bridge
cd ..
```

The generated `my_flutter_bridge` project will also contain a `Cargo.lock` file. You'll need to remove this file before adding it to the workspace:

```sh
rm my_flutter_bridge/Cargo.lock
```

You then need to add the generated Rust project to your Cargo Workspace. Edit the `Cargo.toml` file at the root of your workspace:

```diff
-members = ["workspace_member_1", "workspace_member_2"]
+members = ["workspace_member_1", "workspace_member_2", "my_flutter_bridge"]
```

Finally, run the project:

```sh
cd my_flutter_project/
flutter run
```
