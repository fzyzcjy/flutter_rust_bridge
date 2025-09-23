# Dart Workspace

`flutter_rust_bridge_codegen` supports Dart workspaces. With workspace support, you can use tools like [Melos](https://melos.invertase.dev/) to manage multiple apps and packages
in one repository, organized how you like. This also alows side-by-side organization with
your rust libraries too, meaning your glue code live alongside other rust libraries for
your project.

See an example repository [AadamZ5/flutter_rust_bridge_workspace_example](https://github.com/AadamZ5/flutter_rust_bridge_workspace_example) for more details.
The codegen tool should still be ran from the sub-application's folder.
