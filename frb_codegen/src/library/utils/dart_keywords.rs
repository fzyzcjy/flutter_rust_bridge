pub(crate) fn escape(input: String) -> String {
    if DART_KEYWORDS.contains(&input.as_str()) {
        format!("{input}_")
    } else {
        input
    }
}

// https://dart.dev/guides/language/language-tour#keywords
const DART_KEYWORDS: [&str; 63] = [
    "abstract",
    "else",
    "import",
    "show",
    "as",
    "enum",
    "in",
    "static",
    "assert",
    "export",
    "interface",
    "super",
    "async",
    "extends",
    "is",
    "switch",
    "await",
    "extension",
    "late",
    "sync",
    "break",
    "external",
    "library",
    "this",
    "case",
    "factory",
    "mixin",
    "throw",
    "catch",
    "false",
    "new",
    "true",
    "class",
    "final",
    "null",
    "try",
    "const",
    "finally",
    "on",
    "typedef",
    "continue",
    "for",
    "operator",
    "var",
    "covariant",
    "Function",
    "part",
    "void",
    "default",
    "get",
    "required",
    "while",
    "deferred",
    "hide",
    "rethrow",
    "with",
    "do",
    "if",
    "return",
    "yield",
    "dynamic",
    "implements",
    "set",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Escapes every reserved Dart identifier.
    fn escapes_every_dart_keyword() {
        for keyword in DART_KEYWORDS {
            assert_eq!(escape(keyword.to_owned()), format!("{keyword}_"));
        }
    }

    #[test]
    /// Preserves identifiers that are not reserved Dart keywords.
    fn preserves_non_keywords_and_keyword_prefixes() {
        for identifier in [
            "identifier",
            "Abstract",
            "class_name",
            "className",
            "class_",
        ] {
            assert_eq!(escape(identifier.to_owned()), identifier);
        }
    }
}
