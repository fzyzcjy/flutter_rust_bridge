# Creating the libraries
In this section, we will create our Dart-only base library and then
a Flutter wrapper library built on top of the Dart-only base.

The Flutter library can add additional Flutter-specific functionality
to your Dart-only base; however, it does not need to.
The main purpose of the Flutter wrapper is to bundle the Rust binaries
alongside your Dart library and to re-export the Dart library.
