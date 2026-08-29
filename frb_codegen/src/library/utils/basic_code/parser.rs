use crate::utils::basic_code::dart_header_code::DartHeaderCode;
use crate::utils::basic_code::general_code::GeneralDartCode;

pub(crate) fn parse_dart_code(raw: &str) -> GeneralDartCode {
    let (mut imports, mut body) = (Vec::new(), Vec::new());
    for line in raw.split('\n') {
        (if line.trim_start().starts_with("import ") {
            &mut imports
        } else {
            &mut body
        })
        .push(line);
    }
    GeneralDartCode {
        header: DartHeaderCode {
            import: imports.join("\n"),
            ..Default::default()
        },
        body: body.join("\n"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Separates imports from all remaining Dart source lines.
    fn separates_imports_from_the_body() {
        let code = parse_dart_code(
            "import 'first.dart';\n  import 'second.dart';\n\nclass Example {}\nexport 'not-an-import.dart';",
        );

        assert_eq!(
            code.header.import,
            "import 'first.dart';\n  import 'second.dart';"
        );
        assert_eq!(
            code.body,
            "\nclass Example {}\nexport 'not-an-import.dart';"
        );
    }

    #[test]
    /// Keeps line order and whitespace inside the import and body sections.
    fn preserves_each_section_verbatim() {
        let code = parse_dart_code("// preamble\n\timport 'indented.dart';\nfinal value = 1;\n");

        assert_eq!(code.header.import, "\timport 'indented.dart';");
        assert_eq!(code.body, "// preamble\nfinal value = 1;\n");
    }

    #[test]
    /// Keeps import-like text outside a true import directive in the body.
    fn keeps_import_like_text_outside_true_import_directives_in_the_body() {
        let code = parse_dart_code(
            "importx 'not-a-directive';\n// import 'comment.dart';\nfinal text = 'import value';\n  import 'actual.dart';",
        );

        assert_eq!(code.header.import, "  import 'actual.dart';");
        assert_eq!(
            code.body,
            "importx 'not-a-directive';\n// import 'comment.dart';\nfinal text = 'import value';"
        );
    }

    #[test]
    /// Leaves source without imports entirely in the body.
    fn leaves_source_without_imports_in_the_body() {
        let code = parse_dart_code("class Example {}\n");

        assert!(code.header.import.is_empty());
        assert_eq!(code.body, "class Example {}\n");
    }
}
