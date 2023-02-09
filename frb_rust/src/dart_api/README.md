This folder contains a copy of Dart SDK [include/](https://github.com/dart-lang/sdk/tree/master/runtime/include) folder.

Current version of Dart API is `2.0`. On breaking changes the major version is increased. Minor versions are allowed to be
different. If the DartVM has a higher minor version, it will provide more symbols than we initialize here.

Note that you might need to update if Dart SDK makes an incompatible change to its DL C API.
