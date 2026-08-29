use crate::impl_add_by_add_assign;
use itertools::Itertools;
use serde::Serialize;
use std::ops::AddAssign;

#[derive(Default, Clone, Debug, Serialize)]
pub(crate) struct DartHeaderCode {
    pub file_top: String,
    pub import: String,
    pub part: String,
}

impl_add_by_add_assign!(DartHeaderCode);

impl AddAssign for DartHeaderCode {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.file_top += &rhs.file_top;
        self.import += &rhs.import;
        self.part += &rhs.part;
    }
}

impl DartHeaderCode {
    pub(crate) fn all_code(&self) -> String {
        let import = optimize_imports(&self.import);
        format!("{}\n{}\n{}", self.file_top, import, self.part)
    }
}

fn optimize_imports(raw: &str) -> String {
    raw.split_inclusive(&['\n', ';'][..])
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .sorted()
        .dedup()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Sorts and deduplicates imports separated by whitespace and newlines.
    fn sorts_and_deduplicates_imports() {
        assert_eq!(
            optimize_imports(
                "
                import 'orange.dart';
                import 'apple.dart';
                import 'orange.dart';
                "
            ),
            "import 'apple.dart';\nimport 'orange.dart';"
        );
    }

    #[test]
    /// Recognizes semicolon-delimited imports without trailing newlines.
    fn recognizes_semicolon_delimited_imports() {
        assert_eq!(
            optimize_imports(" import 'orange.dart'; import 'apple.dart';import 'orange.dart';"),
            "import 'apple.dart';\nimport 'orange.dart';"
        );
    }

    #[test]
    /// Drops blank fragments while preserving a final import without a semicolon.
    fn drops_blank_fragments_and_keeps_an_unterminated_import() {
        assert_eq!(
            optimize_imports("\nimport 'b.dart';\n\nimport 'a.dart'"),
            "import 'a.dart'\nimport 'b.dart';"
        );
    }

    #[test]
    /// Renders file sections around normalized imports.
    fn renders_file_sections_around_normalized_imports() {
        let header = DartHeaderCode {
            file_top: "// generated".to_owned(),
            import: "import 'z.dart';\nimport 'a.dart';\nimport 'z.dart';".to_owned(),
            part: "part 'output.dart';".to_owned(),
        };

        assert_eq!(
            header.all_code(),
            "// generated\nimport 'a.dart';\nimport 'z.dart';\npart 'output.dart';"
        );
    }

    #[test]
    /// Concatenates every header section during addition.
    fn concatenates_every_header_section_during_addition() {
        let mut header = DartHeaderCode {
            file_top: "first".to_owned(),
            import: "one".to_owned(),
            part: "alpha".to_owned(),
        };
        header += DartHeaderCode {
            file_top: "second".to_owned(),
            import: "two".to_owned(),
            part: "beta".to_owned(),
        };

        assert_eq!(header.file_top, "firstsecond");
        assert_eq!(header.import, "onetwo");
        assert_eq!(header.part, "alphabeta");
    }
}
