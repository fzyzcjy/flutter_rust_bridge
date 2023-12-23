import 'package:flutter_rust_bridge_internal/src/utils/codecov_transformer.dart';
import 'package:test/test.dart';

void main() {
  test('kIgnoreLineRegex', () {
    expect(shouldKeepLine('hello'), true);

    // Ignore since this is by Rust compiler and is surely correct
    expect(shouldKeepLine('#[derive(Debug)]'), false);
    expect(shouldKeepLine('   #[derive(Copy, Debug, Eq)]   '), false);

    // Also ignore since in Dart/Java/...,
    // such error handling is implicit and will not even appear in code coverage
    expect(shouldKeepLine('    )?;'), false);
    expect(shouldKeepLine('  )?,'), false);
    expect(shouldKeepLine('    }?;'), false);

    // Also ignore pure comments - they are not real code
    expect(shouldKeepLine('// hello'), false);
    expect(shouldKeepLine('    // hello'), false);

    // Also ignore pure brackets, since they are not real code
    // If some branch is not covered, the body itself will be red
    // and we will know it. I see sometimes a bracket is marked red,
    // thus ignore it by default.
    expect(shouldKeepLine('}'), false);
    expect(shouldKeepLine('   };'), false);
  });
}
