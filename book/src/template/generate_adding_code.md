# Adding new code

Let's say we need to change `Platform` such that we don't really care about whether it
is running on Intel or Apple Silicon, but we would like to keep this information so
downlevel code can act on it. We would like to merge `MacApple` and `MacIntel` into a
single `MacOs(String)` that contains the current CPU architecture. Go ahead and update
`native/src/api.rs`:

```diff
 pub enum Platform {
     ..
-    MacIntel,
-    MacApple,
+    MacOs(String),
     ..
 }
```

Now run `just` and see that your binding code now has changed.

## Troubleshooting: "Please supply one or more path/to/llvm..."

A common issue with `ffigen` is that its detection of the LLVM installation is not reliable
across platforms. Especially for MacOS and the split between x86-64 and arm64 binaries,
you might have to modify `justfile` to explicitly point to its location:

```
llvm_path := if os() == "macos" {
    "--llvm-path /opt/homebrew/opt/llvm"
} else {
    ""
}
```