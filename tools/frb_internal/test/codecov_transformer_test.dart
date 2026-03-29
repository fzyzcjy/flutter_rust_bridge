import 'package:flutter_rust_bridge_internal/src/utils/codecov_transformer.dart';
import 'package:test/test.dart';

class _CoverageFixture {
  final List<String> fileLines;
  final Map<String, dynamic> rawCoverage;
  final Map<String, dynamic> expectedCoverage;

  const _CoverageFixture({
    required this.fileLines,
    required this.rawCoverage,
    required this.expectedCoverage,
  });
}

void main() {
  test('kIgnoreLineRegex', () {
    expect(shouldKeepLine('hello'), true);

    expect(shouldKeepLine('#[derive(Debug)]'), false);
    expect(shouldKeepLine('   #[derive(Copy, Debug, Eq)]   '), false);

    expect(shouldKeepLine('    )?;'), false);
    expect(shouldKeepLine('  )?,'), false);
    expect(shouldKeepLine('    }?;'), false);

    expect(shouldKeepLine('// hello'), false);
    expect(shouldKeepLine('    // hello'), false);

    expect(shouldKeepLine('}'), false);
    expect(shouldKeepLine('   };'), false);
    expect(shouldKeepLine('{'), false);
    expect(shouldKeepLine('    )*'), false);
    expect(shouldKeepLine('#[enum_dispatch]'), false);
  });

  test(
    'computeFormatCallNoiseLines detects format call body by matching parentheses',
    () {
      final fileLines = _parseCoverageFixture([
        '[_,_] let maybe_deref_mut_code = if rw == ReadWrite::Write {',
        '[_,_]     format!(',
        '[_,_]         "',
        "[_,_]         impl std::ops::DerefMut for {enum_name}<'_> {{",
        '[_,_]             fn deref_mut(&mut self) -> &mut Self::Target {',
        '[_,_]                 {body}',
        '[_,_]             }',
        '[_,_]         }',
        '[_,_]         "',
        '[_,_]     )',
        '[_,_] } else {',
        '[_,_]     "".to_owned()',
        '[_,_] };',
      ]).fileLines;

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
      _expectFixtureTransformation([
        '[0,0] let maybe_deref_mut_code = if rw == ReadWrite::Write {',
        '[0,null]     format!(',
        '[0,null]         "',
        "[0,null]         impl std::ops::DerefMut for {enum_name}<'_> {{",
        '[0,null]             fn deref_mut(&mut self) -> &mut Self::Target {',
        '[0,null]                 {body}',
        '[0,null]             }',
        '[0,null]         }',
        '[0,null]         "',
        '[0,null]     )',
        '[0,0] } else {',
        '[0,0]     "".to_owned()',
        '[0,null] };',
      ]);
    },
  );

  test('transformCodecovFileCoverageForTest ignores uncovered struct fields', () {
    _expectFixtureTransformation([
      '[_,_] pub(crate) struct Cli {',
      '[_,_]     /// Show debug messages.',
      '[_,_]     #[arg(short, long)]',
      '[0,null]     pub verbose: bool,',
      '[_,_] ',
      '[_,_]     #[command(subcommand)]',
      '[0,null]     pub(crate) command: Commands,',
      '[_,_] }',
    ]);
  });

  test(
    'transformCodecovFileCoverageForTest ignores uncovered multiline destructure noise',
    () {
      _expectFixtureTransformation([
        '[_,_] attrs',
        '[_,_]     .iter()',
        '[_,_]     .filter_map(|attr| match &attr.meta {',
        '[0,null]         Meta::NameValue(MetaNameValue {',
        '[1,1]             path,',
        '[0,null]             value:',
        '[0,null]                 Expr::Lit(ExprLit {',
        '[1,1]                     lit: Lit::Str(lit), ..',
        '[0,null]                 }),',
        '[0,null]             ..',
        '[1,1]         }) if path.is_ident("doc") => Some(parse_comment(&lit.value())),',
        '[_,_]         _ => None,',
        '[_,_]     })',
      ]);
    },
  );

  test(
    'transformCodecovFileCoverageForTest ignores uncovered let destructure noise',
    () {
      _expectFixtureTransformation([
        '[_,_]     fn generate_class(&self) -> Option<ApiDartGeneratedClass> {',
        '[0,null]         let Info {',
        '[0,null]             dart_api_type,',
        '[0,null]             methods,',
        '[0,null]         } = self.compute_info(',
        '[1,1]             &GenerateApiMethodConfig {',
        '[_,_]                 mode_static: GenerateApiMethodMode::DeclAndImpl,',
        '[_,_]                 mode_non_static: GenerateApiMethodMode::DeclOnly,',
        '[_,_]             },',
        '[_,_]             "",',
        '[_,_]         );',
        '[2,2]         let rust_api_type = self.mir.rust_api_type();',
        '[_,_]     }',
      ]);
    },
  );

  test(
    'transformCodecovFileCoverageForTest ignores uncovered constructor scaffolding and call continuations',
    () {
      _expectFixtureTransformation([
        '[_,_] fn demo() {',
        '[0,0]     let value = Some(',
        '[0,null]         DemoStruct {',
        '[0,null]             field: Some(',
        '[1,1]                 other.call()',
        '[0,null]                     .status',
        '[0,null]             ),',
        '[0,null]         },',
        '[0,null]     );',
        '[_,_] }',
      ]);
    },
  );

  test(
    'transformCodecovFileCoverageForTest ignores uncovered multiline derive and static ref noise',
    () {
      _expectFixtureTransformation([
        '[0,null] #[derive(',
        '[0,null]     Debug, Clone, Copy,',
        '[0,null] )]',
        '[0,null] lazy_static! {',
        '[0,null]     static ref FILTER: Regex =',
        '[0,null]         Regex::new("abc")',
        '[0,null]             .unwrap();',
        '[0,null] }',
      ]);
    },
  );

  test(
    'transformCodecovFileCoverageForTest ignores uncovered scalar argument and field prefix noise',
    () {
      _expectFixtureTransformation([
        '[_,_] fn demo() {',
        '[0,0]     info!(',
        '[0,null]         "hello {}",',
        '[1,1]         name,',
        '[0,null]     );',
        '[1,1]     build(false, 42, SomeConfig {',
        '[0,null]         arguments:',
        '[1,1]             value,',
        '[0,null]     });',
        '[_,_] }',
      ]);
    },
  );

  test('computeFormatCallNoiseLines does not ignore ordinary multiline strings', () {
    final fileLines = _parseCoverageFixture([
      '[_,_] let sql = "',
      '[_,_] SELECT *',
      '[_,_] FROM demo',
      '[_,_] ";',
    ]).fileLines;

    expect(computeFormatCallNoiseLines(fileLines), isEmpty);
  });

  test(
    'computeFormatCallNoiseLines handles nested parentheses inside format call',
    () {
      final fileLines = _parseCoverageFixture([
        '[_,_] let content = format!(',
        '[_,_]     "{} {}",',
        '[_,_]     compute_name(foo(bar)),',
        '[_,_]     other_call(),',
        '[_,_] );',
      ]).fileLines;

      expect(computeFormatCallNoiseLines(fileLines), {1, 2, 3, 4, 5});
    },
  );

  test(
    'computeFormatCallNoiseLines ignores single-line format call on one line',
    () {
      final fileLines = _parseCoverageFixture([
        '[_,_] let content = format!("hello {}", name);',
        '[_,_] let untouched = 1;',
      ]).fileLines;

      expect(computeFormatCallNoiseLines(fileLines), {1});
    },
  );

  test('computeFormatCallNoiseLines ignores multiple format calls in one file', () {
    final fileLines = _parseCoverageFixture([
      '[_,_] let first = format!(',
      '[_,_]     "{} {}",',
      '[_,_]     one,',
      '[_,_]     two,',
      '[_,_] );',
      '[_,_] let keep = do_work();',
      '[_,_] let second = format!("value={}", three);',
    ]).fileLines;

    expect(computeFormatCallNoiseLines(fileLines), {1, 2, 3, 4, 5, 7});
  });

  test(
    'computeFormatCallNoiseLines ignores format call with escaped quotes in strings',
    () {
      final fileLines = _parseCoverageFixture([
        '[_,_] let content = format!(',
        r'[_,_]     "say \"hello\" to {}",',
        '[_,_]     name,',
        '[_,_] );',
      ]).fileLines;

      expect(computeFormatCallNoiseLines(fileLines), {1, 2, 3, 4});
    },
  );

  test(
    'computeFormatCallNoiseLines ignores nested format call inside format arguments',
    () {
      final fileLines = _parseCoverageFixture([
        '[_,_] let content = format!(',
        '[_,_]     "{} {}",',
        '[_,_]     format!("inner {}", value),',
        '[_,_]     other,',
        '[_,_] );',
      ]).fileLines;

      expect(computeFormatCallNoiseLines(fileLines), {1, 2, 3, 4, 5});
    },
  );

  test(
    'computeFormatCallNoiseLines ignores format call when opening parenthesis is on next line',
    () {
      final fileLines = _parseCoverageFixture([
        '[_,_] let content = format!',
        '[_,_] (',
        '[_,_]     "{} {}",',
        '[_,_]     one,',
        '[_,_]     two,',
        '[_,_] );',
      ]).fileLines;

      expect(computeFormatCallNoiseLines(fileLines), {2, 3, 4, 5, 6});
    },
  );

  test('computeFormatCallNoiseLines does not match format inside comments', () {
    final fileLines = _parseCoverageFixture([
      '[_,_] // format!("hello {}", name)',
      '[_,_] let untouched = 1;',
    ]).fileLines;

    expect(computeFormatCallNoiseLines(fileLines), isEmpty);
  });

  test(
    'computeFormatCallNoiseLines does not match format inside ordinary strings',
    () {
      final fileLines = _parseCoverageFixture([
        '[_,_] let text = "call format!(\\"hello {}\\", name) later";',
        '[_,_] let untouched = 1;',
      ]).fileLines;

      expect(computeFormatCallNoiseLines(fileLines), isEmpty);
    },
  );

  test(
    'computeFormatCallNoiseLines ignores misleading format text and unmatched parentheses inside format strings',
    () {
      final fileLines = _parseCoverageFixture([
        '[_,_] let content = format!(',
        r'[_,_]     "literal format!( not a call, stray ) ( braces {} and text",',
        '[_,_]     value,',
        '[_,_] );',
        '[_,_] let untouched = 1;',
      ]).fileLines;

      expect(computeFormatCallNoiseLines(fileLines), {1, 2, 3, 4});
    },
  );

  test(
    'computeFormatCallNoiseLines ignores multiline template text containing format markers and stray parentheses',
    () {
      final fileLines = _parseCoverageFixture([
        '[_,_] let content = format!(',
        '[_,_]     "',
        '[_,_]     here is text: format!( definitely not real',
        '[_,_]     and unmatched ) ( inside generated code text',
        '[_,_]     still same template block',
        '[_,_]     ",',
        '[_,_]     value,',
        '[_,_] );',
      ]).fileLines;

      expect(computeFormatCallNoiseLines(fileLines), {1, 2, 3, 4, 5, 6, 7, 8});
    },
  );

  test(
    'computeFormatCallNoiseLines ignores line comments that mention format and unmatched parentheses inside string args',
    () {
      final fileLines = _parseCoverageFixture([
        '[_,_] let content = format!(',
        r'[_,_]     "value // format!( not code ) ( {}",',
        '[_,_]     value, // comment says format!( )(',
        '[_,_] );',
      ]).fileLines;

      expect(computeFormatCallNoiseLines(fileLines), {1, 2, 3, 4});
    },
  );

  test('computeFormatCallNoiseLines stops at line comments inside format call', () {
    final fileLines = _parseCoverageFixture([
      '[_,_] let content = format!(',
      '[_,_]     "{} {}", // comment with ) and format!(',
      '[_,_]     one,',
      '[_,_]     two,',
      '[_,_] );',
    ]).fileLines;

    expect(computeFormatCallNoiseLines(fileLines), {1, 2, 3, 4, 5});
  });

  test(
    'transformCodecovFileCoverageForTest preserves hit lines even inside format call',
    () {
      _expectFixtureTransformation([
        '[0,null] let content = format!(',
        '[3,3]     "{} {}",',
        '[0,null]     one,',
        '[0,null]     two,',
        '[0,null] );',
      ]);
    },
  );

  test(
    'transformCodecovFileCoverageForTest keeps uncovered semantic control-flow lines',
    () {
      _expectFixtureTransformation([
        '[_,_] fn demo(raw: *mut c_void, latest: Output, dart_coverage: bool) -> Result<Self> {',
        '[_,_]     if condition {',
        '[_,_]         do_work()?;',
        '[0,0]     } else {',
        '[0,0]         check_exit_code(&latest)?;',
        '[_,_]     }',
        '[_,_]     if dart_coverage {',
        '[0,0]         return Self(raw);',
        '[_,_]     }',
        '[0,0]     Ok(())',
        '[_,_] }',
      ]);
    },
  );

  test(
    'transformCodecovFileCoverageForTest keeps uncovered unsafe and error-handling lines',
    () {
      _expectFixtureTransformation([
        '[_,_] fn demo(thread_result: Result<(), Error>, raw: *mut c_void) {',
        '[0,0]     unsafe {',
        '[_,_]         call_ffi(raw);',
        '[_,_]     }',
        '[0,0]     if let Err(error) = thread_result {',
        '[0,0]         handle_non_sync_panic_error::<Rust2DartCodec>(error);',
        '[_,_]     }',
        '[_,_] }',
      ]);
    },
  );

  test(
    'transformCodecovFileCoverageForTest keeps uncovered semantic method calls while filtering pure argument scaffolding',
    () {
      _expectFixtureTransformation([
        '[_,_] fn demo(manifest_path: &Path) -> Result<()> {',
        '[0,0]     run(',
        '[0,null]         "cargo",',
        '[0,null]         false,',
        '[0,0]         manifest_path.parent(),',
        '[0,null]     )?;',
        '[0,0]     Some(_) => None,',
        '[_,_] }',
      ]);
    },
  );

  test(
    'transformCodecovFileCoverageForTest keeps uncovered semantic bool argument from cargo_expand real code',
    () {
      _expectFixtureTransformation([
        '[0,0]     Ok(decode_macro_frb_encoded_comments(&run_raw(',
        '[0,0]         rust_crate_dir,',
        '[0,0]         interest_crate_name,',
        '[0,null]         "--cfg frb_expand",',
        '[0,null]         true,',
        '[0,0]         features,',
        '[0,null]     )?)',
        '[0,0]     .into_owned())',
      ]);
    },
  );

  test(
    'transformCodecovFileCoverageForTest keeps uncovered build_web coverage branch lines',
    () {
      _expectFixtureTransformation([
        '[0,0]     if dart_coverage {',
        '[0,0]         let res = command_run!(',
        '[0,0]             call_shell[Some(current_dir), None],',
        '[0,null]             "dart",',
        '[0,null]             "pub",',
        '[0,null]             "global",',
        '[0,null]             "run",',
        '[0,null]             "coverage:collect_coverage",',
        '[0,null]             "--wait-paused",',
        '[0,null]             "--uri=http://127.0.0.1:8181/",',
        '[0,null]             "-o",',
        '[0,null]             "coverage/coverage.json",',
        '[0,null]             "--resume-isolates",',
        '[_,_]             // TODO this scope-output?',
        '[0,null]             "--scope-output=foo",',
        '[0,null]         )?;',
        '[0,0]         check_exit_code(&res)?;',
        '[0,null]     }',
      ]);
    },
  );
}

void _expectFixtureTransformation(List<String> fixtureLines) {
  final fixture = _parseCoverageFixture(fixtureLines);
  final transformed = transformCodecovFileCoverageForTest(
    fixture.fileLines,
    fixture.rawCoverage,
  );

  for (final entry in fixture.expectedCoverage.entries) {
    expect(transformed[entry.key], entry.value, reason: 'line ${entry.key}');
  }
}

_CoverageFixture _parseCoverageFixture(List<String> fixtureLines) {
  final fileLines = <String>[];
  final rawCoverage = <String, dynamic>{};
  final expectedCoverage = <String, dynamic>{};

  for (var i = 0; i < fixtureLines.length; i++) {
    final lineNumber = (i + 1).toString();
    final match = RegExp(r'^\[([^,]+),([^\]]+)\]\s?(.*)$').firstMatch(
      fixtureLines[i],
    );
    if (match == null) {
      throw ArgumentError('Invalid fixture line: ${fixtureLines[i]}');
    }

    final rawToken = match.group(1)!;
    final expectedToken = match.group(2)!;
    final code = match.group(3)!;

    fileLines.add(code);

    final rawValue = _parseCoverageToken(rawToken);
    if (rawValue != _kCoverageOmitted) {
      rawCoverage[lineNumber] = rawValue;
    }

    final expectedValue = _parseCoverageToken(expectedToken);
    if (expectedValue != _kCoverageOmitted) {
      expectedCoverage[lineNumber] = expectedValue;
    }
  }

  return _CoverageFixture(
    fileLines: fileLines,
    rawCoverage: rawCoverage,
    expectedCoverage: expectedCoverage,
  );
}

const _kCoverageOmitted = Object();

dynamic _parseCoverageToken(String token) {
  final trimmed = token.trim();
  if (trimmed == '_') return _kCoverageOmitted;
  if (trimmed == 'null') return null;
  return int.parse(trimmed);
}
