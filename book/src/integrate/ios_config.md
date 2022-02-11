# Configuring build settings

Next, open XCode, select the option to open an existing project and choose
`ios/Runner.xcworkspace` if it exists, or `ios/Runner.xcodeproj/project.pbxproj`.

Click on `Runner` at the top of the left column, then navigate to the Build Settings
tab. We need to change three settings here:

- **Enable Bitcode** to **No**
- **Strip Style** to **Debugging Symbols**
- Add to **Library Search Paths** a new row with the value `${DERIVED_FILE_DIR}`

<video muted autoplay loop controls>
    <source src="xcode_config.webm" type="video/webm">
    Your browser does not support WEBM.
</video>

You can read more about iOS bitcode [here](https://developer.apple.com/forums/thread/3991).
As for the library search path, we will be soon compiling our Rust static library and copying
it to the folder we just specified. If you prefer to directly edit the project file,
add in `ios/Runner.xcodeproj/project.pbxproj` these two settings:

```diff
 .. /* Release */ = {
     ..
     buildSettings = {
         ..
+        LIBRARY_SEARCH_PATHS = "${DERIVED_FILE_DIR}";
+        STRIP_STYLE = debugging;
     }
 }
```

Repeat for the three profiles: debug, profile and release.
