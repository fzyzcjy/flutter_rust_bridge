# Adding build phases

Next, we will add new build phases to the build process.

## Create Phase

From the **Build Phases** tab, click the plus icon to the top left of the panel
to add a new phase. Rearrange this phase to go *before* the **Link Binary with Libraries**
phase. Rename it the **Create Library** phase.

Expand the phase, and replace its contents with:

```bash
source "${PROJECT_DIR}/../create-library.sh"
```

so that it runs the `create-library.sh` script we downloaded earlier.

## Remove Phase

While still on the **Build Phase** tab, create another phase and
rearrange it such that it lies at the very bottom of the process.
Rename it the **Remove Library** phase.

Similar to the Create Phase, replace its contents with:

```bash
source "${PROJECT_DIR}/../remove-library.sh"
```

## Why is this necessary?

The `create-library.sh` script relies on environment variables injected
by XCode, so running it standalone will not work as expected. A better
answer is that [`cargo-lipo`](https://lib.rs/crates/cargo-lipo) was previously
used for this task, but the maintainer has since [declared the utility
unnecessary](https://github.com/timnn/cargo-lipo#maintenance-status).

The `remove-library.sh` script removes the static library (\*.a) files,
since they are unmanaged by XCode and should be disposed of
to prevent stale files.
