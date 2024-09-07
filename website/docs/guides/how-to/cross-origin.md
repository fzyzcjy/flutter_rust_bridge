# Cross-origin in Web

:::info
This page only applies to the web platform,
and other platforms (Android, iOS, Linux, MacOS, Windows) does not have this issue.
:::

If you want to enjoy features such as multi-threading (which by default is enabled),
the [cross-origin headers](../../manual/miscellaneous/web-cross-origin) needs to be specified.

On the other hand, if you cannot enable those headers, then the other parts of flutter_rust_bridge can still work.
Currently, you may need to disable the default thread
pool ([link](https://github.com/fzyzcjy/flutter_rust_bridge/issues/2215)).
However, if this is needed frequently, then maybe we can make it simpler to disable (e.g. create a new flag).
