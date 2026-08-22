from importlib.util import module_from_spec, spec_from_file_location
from pathlib import Path
import sys


MODULE_PATH = Path(__file__).with_name("categorize_code_diff.py")
SPEC = spec_from_file_location("categorize_code_diff", MODULE_PATH)
assert SPEC is not None and SPEC.loader is not None
MODULE = module_from_spec(SPEC)
sys.modules[SPEC.name] = MODULE
SPEC.loader.exec_module(MODULE)


def test_dedicated_test_paths_are_recognized() -> None:
    """Recognizes conventional test directories and filenames."""
    assert MODULE._is_dedicated_test_path("frb_dart/test/read_buffer_test.dart")
    assert MODULE._is_dedicated_test_path("pkg/tests/parser.rs")
    assert not MODULE._is_dedicated_test_path("pkg/src/parser.rs")


def test_rust_test_module_lines_are_recognized() -> None:
    """Recognizes a complete colocated Rust test module."""
    source = """fn production() {}

#[cfg(test)]
mod tests {
    #[test]
    fn works() {
        assert_eq!(\"}\", \"}\");
    }
}

fn later_production() {}
"""

    assert MODULE._rust_test_lines(source) == set(range(3, 10))


def test_rust_comments_do_not_change_test_region_braces() -> None:
    """Ignores braces inside nested comments and strings."""
    source = """#[cfg(test)]
mod tests {
    /* { /* } */ } */
    #[test]
    fn works() { let value = \"}\"; }
}
fn production() {}
"""

    assert MODULE._rust_test_lines(source) == set(range(1, 7))


def test_zero_context_diff_tracks_added_and_removed_lines() -> None:
    """Maps zero-context hunks to old and new line numbers."""
    diff = """diff --git a/src/a.rs b/src/a.rs
--- a/src/a.rs
+++ b/src/a.rs
@@ -2,2 +2,3 @@
-old
+new
+extra
 context
"""

    changed = MODULE._parse_diff(diff)["src/a.rs"]
    assert changed.old == frozenset({2})
    assert changed.new == frozenset({2, 3})


def test_blank_lines_are_not_substantive_changes() -> None:
    """Excludes formatting-only blank lines from category counts."""
    assert MODULE._substantive_lines("code\n\nmore\n") == {1, 3}


def test_deleted_file_uses_its_old_path() -> None:
    """Keeps deleted files in the classification input."""
    diff = """diff --git a/src/old.rs b/src/old.rs
deleted file mode 100644
--- a/src/old.rs
+++ /dev/null
@@ -1 +0,0 @@
-fn old() {}
"""

    changed = MODULE._parse_diff(diff)["src/old.rs"]
    assert changed.old == frozenset({1})
    assert changed.new == frozenset()
