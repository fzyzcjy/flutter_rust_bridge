# Linking the project

Open `ios/Runner.xcodeproj` in Xcode, then add `$crate/$crate.xcodeproj` as a *subproject*
of the Runner project. It should look like this:

![proj-tree](ios-proj-tree.png)

Click on the `Runner` root project, then go to the **Build Phases** tab.
First, expand the **Dependencies** phase, and add **$crate-staticlib**
for iOS, or **$crate-cdylib** for MacOS.

![dep-phase](ios-dep-phase.png)

Then, expand the **Link Binary With Libraries** phase, and add **lib$crate_static.a**
for iOS, or **$crate.dylib** for MacOS.

![link-phase](ios-link-phase.png)
