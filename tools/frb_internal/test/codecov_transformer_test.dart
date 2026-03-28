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

  test('computeFormatMultilineStringNoiseLines detects multiline template body', () {
    final fileLines = [
      'let maybe_deref_mut_code = if rw == ReadWrite::Write {',
      '    format!(',
      '        "',
      "        impl std::ops::DerefMut for {enum_name}<'_> {{",
      '            fn deref_mut(&mut self) -> &mut Self::Target {',
      '                {body}',
      '            }',
      '        }',
      '        "',
      '    )',
      '} else {',
      '    "".to_owned()',
      '};',
    ];

    expect(computeFormatMultilineStringNoiseLines(fileLines), {3, 4, 5, 6, 7, 8, 9});
  });

  test('transformCodecovFileCoverageForTest ignores uncovered multiline template body', () {
    final fileLines = [
      'let maybe_deref_mut_code = if rw == ReadWrite::Write {',
      '    format!(',
      '        "',
      "        impl std::ops::DerefMut for {enum_name}<'_> {{",
      '            fn deref_mut(&mut self) -> &mut Self::Target {',
      '                {body}',
      '            }',
      '        }',
      '        "',
      '    )',
      '} else {',
      '    "".to_owned()',
      '};',
    ];
    final transformed = transformCodecovFileCoverageForTest(fileLines, {
      '1': 0,
      '2': 0,
      '3': 0,
      '4': 0,
      '5': 0,
      '6': 0,
      '7': 0,
      '8': 0,
      '9': 0,
      '10': 0,
      '11': 0,
      '12': 0,
      '13': 0,
    });

    expect(transformed['1'], 0);
    expect(transformed['2'], 0);
    expect(transformed['3'], null);
    expect(transformed['4'], null);
    expect(transformed['5'], null);
    expect(transformed['6'], null);
    expect(transformed['7'], null);
    expect(transformed['8'], null);
    expect(transformed['9'], null);
    expect(transformed['10'], 0);
    expect(transformed['11'], 0);
    expect(transformed['12'], 0);
    expect(transformed['13'], null);
  });

  test('computeFormatMultilineStringNoiseLines does not ignore ordinary multiline strings', () {
    final fileLines = [
      'let sql = "',
      'SELECT *',
      'FROM demo',
      '";',
    ];

    expect(computeFormatMultilineStringNoiseLines(fileLines), isEmpty);
  });
}
