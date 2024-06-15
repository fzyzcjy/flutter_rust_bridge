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
