# Overview

:::info
The API here may subject to change and does not follow semver that is trying to be guaranteed throughout the package
(since it is more like by-product when developing flutter_rust_bridge itself),
especially during the first several versions of 2.x,
but I will try to keep it unchanged.
In addition, even if it is changed, it should highly probably be easily migratable (e.g. simple rename).
For example, maybe we should make it namespaced, instead of all flattened at the top namespace.
:::

The flutter_rust_bridge is cross-platform and support all six platforms - 
Android, iOS, Windows, MacOS, Linux, and Web.
The web platform does have some speciality while the other five platforms are quite similar.
In this section, we show uniformed APIs that can be directly used regardless whether you are on web
or on native (i.e. other 5 platforms).

This is somehow by-product when developing flutter_rust_bridge,
but we make it public because the functionalities are quite commonly used.
