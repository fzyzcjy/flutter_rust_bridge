# Flutter wrapper
On this page, we will start creating the Flutter wrapper around our Dart-only library package.
We start with the plugin_ffi template since it is somewhat similar to what we need,
but we will need to modify it significantly.
Configuring the build processes for each supported platform is also a bit involved,
so those are covered indivdually in the coming pages.

Run `flutter create --help` to see all the available options; you may want to set some (like `--org`).

Finally, in the `packages` folder, run the following, adding any other options you choose
and replacing `library_name` with your library name:
```bash
flutter create --template=plugin_ffi -platforms=android,ios,macos,linux,windows library_name
```

# TODO other steps
- modify pubspec?
- example?
- integration test?
- other files from setup.md
