*Please list issues fixed by this PR here, using format "Fixes #the-issue-number".*

## Checklist

- [ ] An issue to be fixed by this PR is listed above.
- [ ] New tests are added to ensure new features are working. End-to-end tests are usually in the `./frb_example/pure_dart` example, more specifically, `rust/src/api.rs` and `dart/lib/main.dart`.
- [ ] The code generator is run and the code is formatted (e.g. via `just refresh_all`).
- [ ] If this PR adds/changes features, documentations (in the `./book` folder) are updated.
- [ ] CI is passing.

## Remark for PR creator

* New contributors will be blocked by GitHub from running CI, but you can use [a trick](https://github.com/fzyzcjy/flutter_rust_bridge/pull/663#discussion_r962150638) to workaround this, and verify your code using CI by yourself.
* If fzyzcjy does not reply for a few days, maybe he just did not see it, so please ping him.
