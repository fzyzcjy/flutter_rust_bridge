# Integrating with MacOS

The steps required to integrate with MacOS are identical to those of iOS, save for some minor details.

## Adding the dummy header

Flutter on MacOS does not inject headers by default, so you will notice the absence of `macos/Runner/Runner-Bridging-Header.h`
in the MacOS project config. To fix this, in **Build Settings**, change **Objective-C Bridging Header** to be
**Runner/bridge_generated.h**.

## Using the dummy header

Likewise, you will notice that in `macos/Runner/AppDelegate.swift` there is no `func application` anymore.
It is sufficient to use `dummy_method_to_enfore_binding` anywhere in this file, as long as XCode does
not consider it dead code.
