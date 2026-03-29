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
    expect(shouldKeepLine('{'), false);
    expect(shouldKeepLine('    )*'), false);
    expect(shouldKeepLine('#[enum_dispatch]'), false);
  });

  test(
    'computeFormatCallNoiseLines detects format call body by matching parentheses',
    () {
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

      expect(computeFormatCallNoiseLines(fileLines), {
        2,
        3,
        4,
        5,
        6,
        7,
        8,
        9,
        10,
      });
    },
  );

  test(
    'transformCodecovFileCoverageForTest ignores uncovered format call body',
    () {
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
      expect(transformed['10'], null);
      expect(transformed['11'], 0);
      expect(transformed['12'], 0);
      expect(transformed['13'], null);
    },
  );

  test(
    'transformCodecovFileCoverageForTest ignores uncovered struct fields',
    () {
      final fileLines = [
        'pub(crate) struct Cli {',
        '    /// Show debug messages.',
        '    #[arg(short, long)]',
        '    pub verbose: bool,',
        '',
        '    #[command(subcommand)]',
        '    pub(crate) command: Commands,',
        '}',
      ];

      final transformed = transformCodecovFileCoverageForTest(fileLines, {
        '4': 0,
        '7': 0,
      });

      expect(transformed['4'], null);
      expect(transformed['7'], null);
    },
  );

  test(
    'transformCodecovFileCoverageForTest ignores uncovered multiline destructure noise',
    () {
      final fileLines = [
        'attrs',
        '    .iter()',
        '    .filter_map(|attr| match &attr.meta {',
        '        Meta::NameValue(MetaNameValue {',
        '            path,',
        '            value:',
        '                Expr::Lit(ExprLit {',
        '                    lit: Lit::Str(lit), ..',
        '                }),',
        '            ..',
        '        }) if path.is_ident("doc") => Some(parse_comment(&lit.value())),',
        '        _ => None,',
        '    })',
      ];

      final transformed = transformCodecovFileCoverageForTest(fileLines, {
        '4': 0,
        '5': 1,
        '6': 0,
        '7': 0,
        '8': 1,
        '9': 0,
        '10': 0,
        '11': 1,
      });

      expect(transformed['4'], null);
      expect(transformed['5'], 1);
      expect(transformed['6'], null);
      expect(transformed['7'], null);
      expect(transformed['8'], 1);
      expect(transformed['9'], null);
      expect(transformed['10'], null);
      expect(transformed['11'], 1);
    },
  );

  test(
    'transformCodecovFileCoverageForTest ignores uncovered let destructure noise',
    () {
      final fileLines = [
        '    fn generate_class(&self) -> Option<ApiDartGeneratedClass> {',
        '        let Info {',
        '            dart_api_type,',
        '            methods,',
        '        } = self.compute_info(',
        '            &GenerateApiMethodConfig {',
        '                mode_static: GenerateApiMethodMode::DeclAndImpl,',
        '                mode_non_static: GenerateApiMethodMode::DeclOnly,',
        '            },',
        '            "",',
        '        );',
        '        let rust_api_type = self.mir.rust_api_type();',
        '    }',
      ];

      final transformed = transformCodecovFileCoverageForTest(fileLines, {
        '2': 0,
        '3': 0,
        '4': 0,
        '5': 0,
        '6': 1,
        '12': 2,
      });

      expect(transformed['2'], null);
      expect(transformed['3'], null);
      expect(transformed['4'], null);
      expect(transformed['5'], null);
      expect(transformed['6'], 1);
      expect(transformed['12'], 2);
    },
  );

  test(
    'transformCodecovFileCoverageForTest ignores uncovered constructor scaffolding and call continuations',
    () {
      final fileLines = [
        'fn demo() {',
        '    let value = Some(',
        '        DemoStruct {',
        '            field: Some(',
        '                other.call()',
        '                    .status',
        '            ),',
        '        },',
        '    );',
        '}',
      ];

      final transformed = transformCodecovFileCoverageForTest(fileLines, {
        '2': 0,
        '3': 0,
        '4': 0,
        '5': 1,
        '6': 0,
        '7': 0,
        '8': 0,
        '9': 0,
      });

      expect(transformed['2'], null);
      expect(transformed['3'], null);
      expect(transformed['4'], null);
      expect(transformed['5'], 1);
      expect(transformed['6'], null);
      expect(transformed['7'], null);
      expect(transformed['8'], null);
      expect(transformed['9'], null);
    },
  );

  test(
    'transformCodecovFileCoverageForTest ignores uncovered multiline derive and static ref noise',
    () {
      final fileLines = [
        '#[derive(',
        '    Debug, Clone, Copy,',
        ')]',
        'lazy_static! {',
        '    static ref FILTER: Regex =',
        '        Regex::new("abc")',
        '            .unwrap();',
        '}',
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
      });

      expect(transformed['1'], null);
      expect(transformed['2'], null);
      expect(transformed['3'], null);
      expect(transformed['4'], null);
      expect(transformed['5'], null);
      expect(transformed['6'], null);
      expect(transformed['7'], null);
      expect(transformed['8'], null);
    },
  );

  test(
    'computeFormatCallNoiseLines does not ignore ordinary multiline strings',
    () {
      final fileLines = ['let sql = "', 'SELECT *', 'FROM demo', '";'];

      expect(computeFormatCallNoiseLines(fileLines), isEmpty);
    },
  );

  test(
    'computeFormatCallNoiseLines handles nested parentheses inside format call',
    () {
      final fileLines = [
        'let content = format!(',
        '    "{} {}",',
        '    compute_name(foo(bar)),',
        '    other_call(),',
        ');',
      ];

      expect(computeFormatCallNoiseLines(fileLines), {1, 2, 3, 4, 5});
    },
  );

  test(
    'computeFormatCallNoiseLines ignores single-line format call on one line',
    () {
      final fileLines = [
        'let content = format!("hello {}", name);',
        'let untouched = 1;',
      ];

      expect(computeFormatCallNoiseLines(fileLines), {1});
    },
  );

  test(
    'computeFormatCallNoiseLines ignores multiple format calls in one file',
    () {
      final fileLines = [
        'let first = format!(',
        '    "{} {}",',
        '    one,',
        '    two,',
        ');',
        'let keep = do_work();',
        'let second = format!("value={}", three);',
      ];

      expect(computeFormatCallNoiseLines(fileLines), {1, 2, 3, 4, 5, 7});
    },
  );

  test(
    'computeFormatCallNoiseLines ignores format call with escaped quotes in strings',
    () {
      final fileLines = [
        'let content = format!(',
        r'    "say \"hello\" to {}",',
        '    name,',
        ');',
      ];

      expect(computeFormatCallNoiseLines(fileLines), {1, 2, 3, 4});
    },
  );

  test(
    'computeFormatCallNoiseLines ignores nested format call inside format arguments',
    () {
      final fileLines = [
        'let content = format!(',
        '    "{} {}",',
        '    format!("inner {}", value),',
        '    other,',
        ');',
      ];

      expect(computeFormatCallNoiseLines(fileLines), {1, 2, 3, 4, 5});
    },
  );

  test(
    'computeFormatCallNoiseLines ignores format call when opening parenthesis is on next line',
    () {
      final fileLines = [
        'let content = format!',
        '(',
        '    "{} {}",',
        '    one,',
        '    two,',
        ');',
      ];

      expect(computeFormatCallNoiseLines(fileLines), {2, 3, 4, 5, 6});
    },
  );

  test('computeFormatCallNoiseLines does not match format inside comments', () {
    final fileLines = ['// format!("hello {}", name)', 'let untouched = 1;'];

    expect(computeFormatCallNoiseLines(fileLines), isEmpty);
  });

  test(
    'computeFormatCallNoiseLines does not match format inside ordinary strings',
    () {
      final fileLines = [
        'let text = "call format!(\\"hello {}\\", name) later";',
        'let untouched = 1;',
      ];

      expect(computeFormatCallNoiseLines(fileLines), isEmpty);
    },
  );

  test(
    'computeFormatCallNoiseLines ignores misleading format text and unmatched parentheses inside format strings',
    () {
      final fileLines = [
        'let content = format!(',
        r'    "literal format!( not a call, stray ) ( braces {} and text",',
        '    value,',
        ');',
        'let untouched = 1;',
      ];

      expect(computeFormatCallNoiseLines(fileLines), {1, 2, 3, 4});
    },
  );

  test(
    'computeFormatCallNoiseLines ignores multiline template text containing format markers and stray parentheses',
    () {
      final fileLines = [
        'let content = format!(',
        '    "',
        '    here is text: format!( definitely not real',
        '    and unmatched ) ( inside generated code text',
        '    still same template block',
        '    ",',
        '    value,',
        ');',
      ];

      expect(computeFormatCallNoiseLines(fileLines), {1, 2, 3, 4, 5, 6, 7, 8});
    },
  );

  test(
    'computeFormatCallNoiseLines ignores line comments that mention format and unmatched parentheses inside string args',
    () {
      final fileLines = [
        'let content = format!(',
        r'    "value // format!( not code ) ( {}",',
        '    value, // comment says format!( )(',
        ');',
      ];

      expect(computeFormatCallNoiseLines(fileLines), {1, 2, 3, 4});
    },
  );

  test(
    'computeFormatCallNoiseLines stops at line comments inside format call',
    () {
      final fileLines = [
        'let content = format!(',
        '    "{} {}", // comment with ) and format!(',
        '    one,',
        '    two,',
        ');',
      ];

      expect(computeFormatCallNoiseLines(fileLines), {1, 2, 3, 4, 5});
    },
  );

  test(
    'transformCodecovFileCoverageForTest preserves hit lines even inside format call',
    () {
      final fileLines = [
        'let content = format!(',
        '    "{} {}",',
        '    one,',
        '    two,',
        ');',
      ];
      final transformed = transformCodecovFileCoverageForTest(fileLines, {
        '1': 0,
        '2': 3,
        '3': 0,
        '4': 0,
        '5': 0,
      });

      expect(transformed['1'], null);
      expect(transformed['2'], 3);
      expect(transformed['3'], null);
      expect(transformed['4'], null);
      expect(transformed['5'], null);
    },
  );
}
