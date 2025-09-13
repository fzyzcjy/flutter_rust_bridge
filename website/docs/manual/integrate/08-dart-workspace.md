# Dart Workspace

`flutter_rust_bridge_codegen` supports Dart workspaces, but the codegen tool must still be ran from
the sub-application's folder.

With workspace support, you can use tools like [Melos](https://melos.invertase.dev/) to manage multiple apps and packages
in one repository, organized how you like. This also alows side-by-side organization with
your rust libraries too, meaning your glue code can live alongside other rust libraries for
your project.

See an example repository [AadamZ5/flutter_rust_bridge_workspace_example](https://github.com/AadamZ5/flutter_rust_bridge_workspace_example) for more details
