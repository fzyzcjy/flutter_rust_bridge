## Changes

_Please list issues fixed by this PR here, using format "Fixes #the-issue-number"._

## Checklist

- [ ] An issue to be fixed by this PR is listed above.
- [ ] New tests are added to ensure new features are working. End-to-end tests are usually in the `./frb_example/pure_dart` example, more specifically, `rust/src/api/whatever.rs` and `test/api/whatever_test.dart`.
- [ ] `./frb_internal precommit --mode slow` (or `fast`) is run (it internal runs code generator, does auto formatting, etc).
- [ ] If this PR adds/changes features, documentations (in the `./website` folder) are updated.
- [ ] CI is passing. (Operations are reproducible using corresponding `./frb_internal something` commands shown in CI.)

## Remark for PR creator

- `./frb_internal --help` shows utilities for development.
- If fzyzcjy does not reply for a few days, maybe he just did not see it, so please ping him.
