# Overview

There are roughly two components, let's call them "core" and "compilation":

* **Core**:
  By using the core code generator (`flutter_rust_bridge_codegen generate`) and support library,
  the Rust code can be seamlessly called from/to Flutter (Dart) code.
  That's what we mainly talk about in the [guides](../../guides) chapter.
* **Compilation**:
  This component makes *any* Rust code be compiled/bundled with *any* Flutter (Dart) code.

In this chapter, we focus on the *compilation* component.
Since it is not specific or tied to the flutter_rust_bridge core,
we demonstrate some approaches to do so as follows.

Each approach has its strengths and weaknesses depending on what feature you want.
For example, some developers want high automation level and abstract away how the code is compiled,
while some others may want to avoid complexity embedded in automation and prefer fine-grained control to compilation process.
As another example, when developing a library (instead of an app), 
some people may prefer prebuilt binaries (e.g. no need for user to have a full Rust compilation environment),
while some people may prefer the opposite (e.g. user may want to build all binaries by themselves).
A (very brief) overview of the approaches are as follows:

* [`flutter_rust_bridge_codegen create/integrate` command](builtin): Currently based on Cargokit, thus high automation level, and (for library instead of app) no prebuilt binaries.
* [Native assets](native-assets): Future Dart feature. High automation level, possibly the preferred way in the future.
* [Cargokit](cargokit): High automation level, (for library instead of app) no prebuilt binaries.
* [flutter_rust_bridge_template](template): Fine-grained control to compilation process.
* [BrickHub brick](existing): Similar to the line above.
* [Create libraries](library): Fine-grained control to compilation process, create CI and prebuilt binaries.
* ...
