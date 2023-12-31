# Customizing the Rust Folder Location

By default, your Rust code goes into the '/rust' folder, but you can customize this location. 
Let's call the root folder '/frb', thus the default Rust folder would be '/frb/rust'.  
With the following steps, you can locate it anywhere else. As an example, we will use '/app_core', parallel to '/frb', as the new location. 
For a complete example, see [this project](https://github.com/patmuk/flutter-rust-bridge_crux_style/tree/flutter-rust-bridge_codegen):

## Steps:

### 1. Delete the 'rust' folder:
Remove the existing '/frb/rust' folder from your Flutter-Rust-Bridge project as you will be replacing it with your custom folder.

### 2. Create your custom folder:
Create a new folder in the project and name it 'app_core' (or your preferred name).

### 3. Update the Code Generation Configuration:
Generate the 'glue code' by updating the configuration in `flutter_rust_bridge.yaml` located at the root of your project (or equivalent cli arguments):

```yaml
rust_input: ../app_core/src/api.rs
rust_root: ../app_core
rust_output: ../app_core/src/bridge/generated/mod.rs
```
(`rust_root` might not be needed, as it can be derived from `rust_input`).  
Note that in this example, there is a file `app_core/src/bridge/mod.rs` containing `pub mod generated;`. 
If you want to implement a similar (optional) structure, be sure to surpress codegen adding a module with `add_mod_to_lib: false`.  

### 4. Update the Build Configuration (Cargokit):
In order for the flutter compiler to find the rust library at the new location conduct the changes. 
The compiling is done in the cargokit folder, all needed files reside in `/frb/rust_builder`. 
For each of the different plattfrom targets, the following changes are needed.  
As mentioned, `/app_core` is the location of our Rust project, the name of the project (reffered to as `libname` in the configuration files) is `app_core` as well.

#### Android Configuration:

Update the file `build.gradle` located at `frb/rust_builder/android/build.gradle`:

```diff
android {
  // ... other configurations ...

  apply from: "../cargokit/gradle/plugin.gradle"
  cargokit {
-     manifestDir = "../../rust"
-     libname = "rust_lib"
+     manifestDir = "../../../app_core"
+     libname = "app_core"
  }
}
```

#### iOS/MacOS Configuration:

The same changes are needed, thus update the file `rust_builder.podspec` located at `frb/rust_builder/ios/rust_builder.podspec` and `frb/rust_builder/macos/rust_builder.podspec` respectively:

```diff 
  // ... other configurations ...

   s.script_phase = {
     :name => 'Build Rust library',
     # First argument is relative path to the `rust` folder, second is name of rust library
-    :script => 'sh "$PODS_TARGET_SRCROOT/../cargokit/build_pod.sh" ../../rust rust_lib',
+    :script => 'sh "$PODS_TARGET_SRCROOT/../cargokit/build_pod.sh" ../../../app_core app_core',
     :execution_position => :before_compile,
     :input_files => ['${BUILT_PRODUCTS_DIR}/cargokit_phony'],
     # Let XCode know that the static library referenced in -force_load below is
     # created by this build step.
-    :output_files => ["${BUILT_PRODUCTS_DIR}/librust_lib.a"],
+    :output_files => ["${BUILT_PRODUCTS_DIR}/libapp_core.a"],
   s.pod_target_xcconfig = {
     'DEFINES_MODULE' => 'YES',
     # Flutter.framework does not contain a i386 slice.
     'EXCLUDED_ARCHS[sdk=iphonesimulator*]' => 'i386',
-    'OTHER_LDFLAGS' => '-force_load ${BUILT_PRODUCTS_DIR}/librust_lib.a',
+    'OTHER_LDFLAGS' => '-force_load ${BUILT_PRODUCTS_DIR}/libapp_core.a',
  }
end
```

### Linux/Windows Confuguration:

The same changes are needed, thus update the file `CMakeList.txt` located at `frb/rust_builder/linux/CMakeList.txt` and `frb/rust_builder/windows/CMakeList.txt` respectively:

```diff
  // ... other configurations ...

include("../cargokit/cmake/cargokit.cmake")
- apply_cargokit(${PROJECT_NAME} ../../../../../../rust rust_lib "")
+ apply_cargokit(${PROJECT_NAME} ../../../../../../../app_core app_core "")

# List of absolute paths to libraries that should be bundled with the plugin.
# This list could contain prebuilt libraries, or libraries created by an
```
