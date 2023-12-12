## Changes

_Please list issues fixed by this PR here, using format "Fixes #the-issue-number"._

## Checklist

- [ ] An issue to be fixed by this PR is listed above.
- [ ] New tests are added to ensure new features are working. End-to-end tests are usually in the `./frb_example/pure_dart` example, more specifically, `rust/src/api/whatever.rs` and `test/api/whatever_test.dart`.
- [ ] `./frb_internal precommit` is run (it internal runs code generator, does auto formatting, etc).
- [ ] If this PR adds/changes features, documentations (in the `./website` folder) are updated.
- [ ] CI is passing.

## Remark for PR creator

- Justfile is a task runner for the command line interface that allows you to run shell commands from a file named `justfile` in your project directory. You can use Justfile after [installing it](https://github.com/casey/just). Note that commands written in `justfile` of this repository are expected to be run in `bash`, not `cmd` or `powershell`. Running `just ...` commands in `cmd` or `powershell` will produce errors as the syntax is not compatible. On Windows, you can use `git bash` if you have Git installed.
- If fzyzcjy does not reply for a few days, maybe he just did not see it, so please ping him.
