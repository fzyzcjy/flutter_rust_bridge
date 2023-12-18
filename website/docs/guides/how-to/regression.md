# Avoid regressions

Every code change will have to pass the CI with various checks and tons of tests (see the CI part of [this page](../miscellaneous/safety)),
so things are usually stable.

However, if you want to be 100% sure that your own use case will work forever,
feel free to make a PR to add some tests for your scenario
(mainly located in `frb_example/pure_dart/`).

This is in similar spirit as [rfw](https://pub.dev/packages/rfw),
a package published by Flutter itself,
quoted below:

> We plan to keep the format and supported widget set backwards compatible,
> so that once a file works, it will keep working.
> However, this is best-effort only.
> To guarantee that files keep working as you expect,
> submit tests to this package (e.g. the binary file and the corresponding screenshot, as a golden test).
