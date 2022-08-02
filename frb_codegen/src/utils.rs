use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::fmt::Display;
use std::fs;
use std::hash::Hash;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use cargo_metadata::{Version, VersionReq};
use log::debug;
use serde::Deserialize;

pub fn mod_from_rust_path(code_path: &str, crate_path: &str) -> String {
    Path::new(code_path)
        .strip_prefix(Path::new(crate_path).join("src"))
        .unwrap()
        .with_extension("")
        .into_os_string()
        .into_string()
        .unwrap()
        .replace('/', "::")
}

pub fn with_changed_file<F: FnOnce() -> anyhow::Result<()>>(
    path: &str,
    append_content: &str,
    f: F,
) -> anyhow::Result<()> {
    let content_original = fs::read_to_string(&path)?;
    fs::write(&path, content_original.clone() + append_content)?;

    f()?;

    Ok(fs::write(&path, content_original)?)
}

pub fn find_all_duplicates<T>(iter: &[T]) -> Vec<T>
where
    T: Eq + Hash + Clone,
{
    let mut uniq = HashSet::new();
    iter.iter()
        .filter(|x| !uniq.insert(*x))
        .cloned()
        .collect::<Vec<_>>()
}

/// check api defined by users, if no duplicates, then generate all symbols (api function name),
/// including those generated implicitily by frb
pub fn get_symbols_if_no_duplicates(configs: &[crate::Opts]) -> Result<Vec<String>, anyhow::Error> {
    let mut explicit_raw_symbols = Vec::new();
    let mut all_symbols = Vec::new();
    for config in configs {
        let raw_ir_file = config.get_ir_file();

        // for checking explicit api duplication
        explicit_raw_symbols.extend(raw_ir_file.funcs.iter().map(|f| f.name.clone()));

        // for avoiding redundant generation in dart
        all_symbols.extend(raw_ir_file.get_all_symbols(config));
    }
    let duplicates = find_all_duplicates(&explicit_raw_symbols);
    if !duplicates.is_empty() {
        let duplicated_symbols = duplicates.join(",");

        let (symbol_str, verb_str) = if duplicates.len() == 1 {
            ("symbol", "has")
        } else {
            ("symbols", "have")
        };
        panic!(
            "{} [{}] {} already been defined",
            symbol_str, duplicated_symbols, verb_str
        );
    }

    Ok(all_symbols)
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct BlockIndex(pub usize);

impl BlockIndex {
    pub const PRIMARY: BlockIndex = BlockIndex(0);
}

impl Display for BlockIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum DartToolchain {
    Dart,
    Flutter,
}

impl DartToolchain {
    pub(crate) fn as_run_command(&self) -> &'static str {
        match self {
            DartToolchain::Dart => "dart",
            DartToolchain::Flutter => "flutter pub",
        }
    }
}

#[derive(Debug, Deserialize)]
struct PubspecLock {
    pub packages: HashMap<String, serde_yaml::Value>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum PackageVersion {
    Inline(String),
    Multiline { version: Option<String> },
}

#[derive(Debug)]
pub enum PackageVersionKind {
    Exact(Version),
    Range(VersionReq),
}

impl TryFrom<&PackageVersion> for PackageVersionKind {
    type Error = anyhow::Error;
    fn try_from(version: &PackageVersion) -> Result<Self, Self::Error> {
        match version {
            PackageVersion::Inline(ref version) => {
                let version = PackageVersionKind::from_str(version)?;
                Ok(version)
            }
            PackageVersion::Multiline { ref version } => {
                if let Some(version) = version {
                    let version = PackageVersionKind::from_str(version)?;
                    return Ok(version);
                }
                Err(anyhow::anyhow!("no version found"))
            }
        }
    }
}

impl FromStr for PackageVersionKind {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let range: [char; 4] = ['>', '<', '=', '^'];
        if s.contains(range) {
            let version_req = VersionReq::parse(s)?;
            Ok(PackageVersionKind::Range(version_req))
        } else {
            let version = Version::parse(s)?;
            Ok(PackageVersionKind::Exact(version))
        }
    }
}

impl ToString for PackageVersionKind {
    fn to_string(&self) -> String {
        match self {
            PackageVersionKind::Exact(v) => v.to_string(),
            PackageVersionKind::Range(v) => v.to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum PackageManager {
    Dependencies,
    DevDependencies,
}

impl std::fmt::Display for PackageManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PackageManager::Dependencies => write!(f, "dependencies"),
            PackageManager::DevDependencies => write!(f, "dev_dependencies"),
        }
    }
}

#[derive(Debug, Deserialize)]
struct Pubspec {
    pub dependencies: Option<HashMap<String, PackageVersion>>,
    pub dev_dependencies: Option<HashMap<String, PackageVersion>>,
}

#[inline]
fn read_file(at: &str, filename: &str) -> anyhow::Result<String> {
    let file = PathBuf::from(at).join(filename);
    if !file.exists() {
        return Err(anyhow::Error::msg(format!(
            "missing {} in {}",
            filename, at
        )));
    }
    let content = std::fs::read_to_string(file)
        .map_err(|_| anyhow::Error::msg(format!("unable to read {} in {}", filename, at)))?;
    Ok(content)
}

pub(crate) fn guess_toolchain(dart_root: &str) -> anyhow::Result<DartToolchain> {
    debug!("Guessing toolchain the runner is run into");
    let lock_file = read_file(dart_root, "pubspec.lock")?;
    let lock_file: PubspecLock = serde_yaml::from_str(&lock_file).map_err(|_| {
        anyhow::Error::msg(format!("unable to parse pubspec.lock in {}", dart_root))
    })?;
    if lock_file.packages.contains_key("flutter") {
        return Ok(DartToolchain::Flutter);
    }
    Ok(DartToolchain::Dart)
}

pub(crate) fn ensure_dependencies(
    dart_root: &str,
    package: &str,
    manager: PackageManager,
) -> anyhow::Result<Option<PackageVersionKind>> {
    debug!(
        "Checking presence of {} in {} at {}",
        package, manager, dart_root
    );
    let manifest_file = read_file(dart_root, "pubspec.yaml")?;
    let manifest_file: Pubspec = serde_yaml::from_str(&manifest_file).map_err(|_| {
        anyhow::Error::msg(format!("unable to parse pubspec.yaml in {}", dart_root))
    })?;
    let deps = if manager == PackageManager::DevDependencies {
        manifest_file.dev_dependencies.unwrap_or_default()
    } else {
        manifest_file.dependencies.unwrap_or_default()
    };
    let version = deps.get(package);
    let version = version.map(|v| PackageVersionKind::try_from(v).unwrap());
    Ok(version)
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use super::{guess_toolchain, DartToolchain};
    use lazy_static::lazy_static;

    lazy_static! {
        static ref FRB_EXAMPLES_FOLDER: PathBuf = {
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("..")
                .join("frb_example")
        };
    }

    fn guess_toolchain_base(path: &Path, expect_toolchain: DartToolchain) {
        let toolchain = guess_toolchain(&path.to_string_lossy()).expect(&format!(
            "can get toolchain from {}",
            path.to_string_lossy()
        ));
        assert_eq!(toolchain, expect_toolchain);
    }

    #[test]
    fn guess_dart_toolchain() {
        guess_toolchain_base(
            FRB_EXAMPLES_FOLDER.join("pure_dart").join("dart").as_path(),
            DartToolchain::Dart,
        );
        guess_toolchain_base(
            FRB_EXAMPLES_FOLDER
                .join("pure_dart_multi")
                .join("dart")
                .as_path(),
            DartToolchain::Dart,
        );
    }

    #[test]
    fn guess_flutter_toolchain() {
        guess_toolchain_base(
            FRB_EXAMPLES_FOLDER.join("with_flutter").as_path(),
            DartToolchain::Flutter,
        );
    }
}
