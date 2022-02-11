# Retrieving build scripts

To expedite the process, we will use build scripts that can detect the
necessary targets as well as the build profile (debug, profile or release).
These are the same files used by the template, so go ahead and download them
to the *root of your Flutter app*, i.e. the folder above `ios`:

- [`create-library.sh`](https://raw.githubusercontent.com/Desdaemon/flutter_rust_bridge_template/main/create-library.sh)
- [`remove-library.sh`](https://github.com/Desdaemon/flutter_rust_bridge_template/blob/main/remove-library.sh)

Now we need to modify these scripts to match our setup.

In `create-library.sh`:

```diff
 # the resulting library in the build products
 #
 # Change this to your crate name.
-crate="native"
+crate="$crate"
 #
 # The $PATH used by Xcode likely won't contain Cargo, fix that.
 # In addition, the $PATH used by XCode has lots of Apple-specific
 # developer tools that your Cargo isn't expecting to use, fix that.
```

In `remove-library.sh`:

```diff
-rm -fv ${DERIVED_FILES_DIR}/libnative.a
+rm -fv ${DERIVED_FILES_DIR}/lib$crate.a
```
