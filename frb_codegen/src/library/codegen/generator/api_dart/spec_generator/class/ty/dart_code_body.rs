//! Classifies the members of a `dart_code` class body.
//!
//! The text is already the inside of the generated class, so declarations are
//! read at brace depth zero. Comments, strings, and the bodies of factories or
//! methods are ignored.

const RESERVED_OPAQUE_MEMBERS: &[&str] = &[
    "dispose",
    "isDisposed",
    "move",
    "frbInternalCstEncode",
    "frbInternalSseEncode",
];

pub(crate) fn has_instance_member(body: &str) -> bool {
    declarations(body).iter().any(|decl| decl.instance)
}

pub(crate) fn reserved_instance_member(body: &str) -> Option<&'static str> {
    declarations(body).iter().find_map(|decl| {
        if !decl.instance {
            return None;
        }
        RESERVED_OPAQUE_MEMBERS
            .iter()
            .copied()
            .find(|name| *name == decl.name)
    })
}

pub(crate) fn reserved_opaque_member_list() -> &'static str {
    "dispose, isDisposed, move, frbInternalCstEncode, frbInternalSseEncode"
}

struct Decl {
    name: String,
    instance: bool,
}

fn declarations(body: &str) -> Vec<Decl> {
    headers(body)
        .into_iter()
        .filter_map(|header| classify(&header))
        .collect()
}

fn classify(header: &str) -> Option<Decl> {
    let text = strip_annotations(header);
    let text = text.trim();
    if text.is_empty() || is_directive(text) {
        return None;
    }
    let (is_static, is_factory, rest) = peel_modifiers(text);
    let name = member_name(rest)?;
    let instance = !is_static && !is_factory && !is_constructor(rest);
    Some(Decl { name, instance })
}

fn is_directive(text: &str) -> bool {
    let text = text.trim();
    strip_word(text, "import").is_some()
        || strip_word(text, "export").is_some()
        || strip_word(text, "part").is_some()
        || strip_word(text, "library").is_some()
}

fn peel_modifiers(text: &str) -> (bool, bool, &str) {
    let mut rest = text.trim();
    let mut is_static = false;
    let mut is_factory = false;
    loop {
        if let Some(next) = strip_word(rest, "static") {
            is_static = true;
            rest = next;
            continue;
        }
        if let Some(next) = strip_word(rest, "factory") {
            is_factory = true;
            rest = next;
            continue;
        }
        if let Some(next) = strip_word(rest, "const")
            .or_else(|| strip_word(rest, "external"))
            .or_else(|| strip_word(rest, "final"))
            .or_else(|| strip_word(rest, "late"))
            .or_else(|| strip_word(rest, "covariant"))
        {
            rest = next;
            continue;
        }
        break;
    }
    (is_static, is_factory, rest)
}

fn strip_word<'a>(text: &'a str, word: &str) -> Option<&'a str> {
    let rest = text.strip_prefix(word)?;
    if rest
        .chars()
        .next()
        .is_some_and(|ch| ch.is_ascii_alphanumeric() || ch == '_')
    {
        return None;
    }
    Some(rest.trim_start())
}

fn is_constructor(text: &str) -> bool {
    let head = text.split('(').next().unwrap_or(text).trim();
    let ident = head.split('.').next().unwrap_or(head);
    !ident.is_empty()
        && ident.starts_with(|ch: char| ch.is_ascii_uppercase())
        && ident
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
        && !head.contains(' ')
}

fn member_name(text: &str) -> Option<String> {
    let text = text.trim();
    if let Some(rest) = find_word(text, "set") {
        return first_ident(rest);
    }
    if let Some(rest) = find_word(text, "get") {
        return first_ident(rest);
    }
    if find_word(text, "operator").is_some() {
        return Some("operator".to_owned());
    }
    if let Some(index) = text.find('(') {
        return ident_before(&text[..index]);
    }
    last_ident(text)
}

fn find_word<'a>(text: &'a str, word: &str) -> Option<&'a str> {
    let bytes = text.as_bytes();
    let word_bytes = word.as_bytes();
    let mut start = 0;
    while start + word_bytes.len() <= bytes.len() {
        if bytes[start..].starts_with(word_bytes) {
            let before_ok = start == 0 || bytes[start - 1].is_ascii_whitespace();
            let after = start + word_bytes.len();
            let after_ok = after == bytes.len()
                || !(bytes[after].is_ascii_alphanumeric() || bytes[after] == b'_');
            if before_ok && after_ok {
                return Some(text[after..].trim_start());
            }
        }
        start += 1;
    }
    None
}

fn first_ident(text: &str) -> Option<String> {
    let ident: String = text
        .trim_start()
        .chars()
        .take_while(|ch| ch.is_ascii_alphanumeric() || *ch == '_')
        .collect();
    (!ident.is_empty()).then_some(ident)
}

fn ident_before(text: &str) -> Option<String> {
    let trimmed = text.trim_end();
    let ident: String = trimmed
        .chars()
        .rev()
        .take_while(|ch| ch.is_ascii_alphanumeric() || *ch == '_')
        .collect::<String>()
        .chars()
        .rev()
        .collect();
    (!ident.is_empty()).then_some(ident)
}

fn last_ident(text: &str) -> Option<String> {
    text.split(|ch: char| !(ch.is_ascii_alphanumeric() || ch == '_'))
        .rfind(|part| !part.is_empty())
        .map(str::to_owned)
}

fn strip_annotations(text: &str) -> String {
    let mut rest = text.trim_start();
    while rest.starts_with('@') {
        if let Some(open) = rest.find('(') {
            let name = &rest[1..open];
            if !name.contains(char::is_whitespace) {
                if let Some(end) = matching_paren(rest, open) {
                    rest = rest[end + 1..].trim_start();
                    continue;
                }
            }
        }
        if let Some(end) = rest.find(char::is_whitespace) {
            rest = rest[end..].trim_start();
        } else {
            return String::new();
        }
    }
    rest.to_owned()
}

fn matching_paren(text: &str, open: usize) -> Option<usize> {
    let mut depth = 0i32;
    for (index, ch) in text[open..].char_indices() {
        match ch {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(open + index);
                }
            }
            _ => {}
        }
    }
    None
}

fn headers(body: &str) -> Vec<String> {
    let chars: Vec<char> = body.chars().collect();
    let mut headers = Vec::new();
    let mut buffer = String::new();
    let mut depth = 0i32;
    let mut index = 0;
    while index < chars.len() {
        let ch = chars[index];
        if ch == '/' && chars.get(index + 1) == Some(&'/') {
            index += 2;
            while index < chars.len() && chars[index] != '\n' {
                index += 1;
            }
            continue;
        }
        if ch == '/' && chars.get(index + 1) == Some(&'*') {
            index += 2;
            while index + 1 < chars.len() && !(chars[index] == '*' && chars[index + 1] == '/') {
                index += 1;
            }
            index = (index + 2).min(chars.len());
            continue;
        }
        if ch == '\'' || ch == '"' {
            index = skip_string(&chars, index);
            if depth == 0 {
                buffer.push(' ');
            }
            continue;
        }
        if ch == '{' {
            if depth == 0 {
                headers.push(std::mem::take(&mut buffer));
            }
            depth += 1;
            index += 1;
            continue;
        }
        if ch == '}' {
            depth = (depth - 1).max(0);
            index += 1;
            continue;
        }
        if ch == ';' && depth == 0 {
            headers.push(std::mem::take(&mut buffer));
            index += 1;
            continue;
        }
        if depth == 0 {
            buffer.push(ch);
        }
        index += 1;
    }
    if !buffer.trim().is_empty() {
        headers.push(buffer);
    }
    headers
}

fn skip_string(chars: &[char], start: usize) -> usize {
    let quote = chars[start];
    let triple = chars.get(start + 1) == Some(&quote) && chars.get(start + 2) == Some(&quote);
    if triple {
        let mut index = start + 3;
        while index + 2 < chars.len()
            && !(chars[index] == quote && chars[index + 1] == quote && chars[index + 2] == quote)
        {
            index += 1;
        }
        return (index + 3).min(chars.len());
    }
    let mut index = start + 1;
    while index < chars.len() && chars[index] != quote {
        if chars[index] == '\\' {
            index += 2;
            continue;
        }
        index += 1;
    }
    (index + 1).min(chars.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instance_method_counts_and_static_factory_and_comments_do_not() {
        assert!(has_instance_member("int extra() => 1;"));
        assert!(has_instance_member(
            "@override\n  bool operator ==(Object other) => true;"
        ));
        assert!(has_instance_member("int get hashCode => 7;"));
        assert!(!has_instance_member(""));
        assert!(!has_instance_member("// only a comment\n/* and a block */"));
        assert!(!has_instance_member(
            "static int get dartCodeGetter => 123;"
        ));
        assert!(!has_instance_member(
            "factory BaseRustState({required void Function() onMutate}) {\n  return BaseRustState.empty();\n}"
        ));
        assert!(!has_instance_member("const Foo._();"));
        assert!(!has_instance_member("import 'dart:math';"));
    }

    #[test]
    fn reserved_names_are_instance_declarations_only() {
        assert_eq!(
            reserved_instance_member("void dispose() { print('x'); }"),
            Some("dispose")
        );
        assert_eq!(
            reserved_instance_member("bool get isDisposed => true;"),
            Some("isDisposed")
        );
        assert_eq!(
            reserved_instance_member("set move(bool? value) {}"),
            Some("move")
        );
        assert_eq!(
            reserved_instance_member("int frbInternalCstEncode({bool? move}) => 0;"),
            Some("frbInternalCstEncode")
        );
        assert_eq!(
            reserved_instance_member("// void dispose() {}\nfinal text = 'dispose';"),
            None
        );
        assert_eq!(reserved_instance_member("static void dispose() {}"), None);
        assert_eq!(reserved_instance_member("int extra() => 1;"), None);
    }
}
