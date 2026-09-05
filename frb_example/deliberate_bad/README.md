## Deliberately bad code to test sanitizers

Please visit the main documentation or open an issue for more information.

- The Dart-to-Rust data-race sentinel uses volatile writes so optimized native builds retain both conflicting accesses.
- Both threads wait at a barrier before writing the same location; the writes remain unsynchronized with each other and must produce a TSAN data-race report.
