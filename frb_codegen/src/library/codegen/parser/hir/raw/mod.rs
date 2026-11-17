use crate::codegen::dumper::Dumper;
use crate::codegen::ir::hir::raw::crates::HirRawCrate;
use crate::codegen::ir::hir::raw::pack::HirRawPack;
use crate::codegen::parser::hir::internal_config::ParserHirInternalConfig;
use crate::library::commands::cargo_expand::run_cargo_expand;
use crate::utils::crate_name::CrateName;
use itertools::concat;

pub(crate) fn parse(
    config: &ParserHirInternalConfig,
    dumper: &Dumper,
) -> anyhow::Result<HirRawPack> {
    let crates = concat([
        vec![CrateName::self_crate()],
        config.third_party_crate_names.clone(),
    ])
    .iter()
    .map(|crate_name| {
        Ok(HirRawCrate {
            name: crate_name.to_owned(),
            syn_file: run_cargo_expand(
                &config.rust_crate_dir,
                (!crate_name.is_self_crate()).then_some(crate_name),
                dumper,
                compute_features(crate_name, config.rust_features.as_deref()),
                config.tool_install_mode,
            )?,
        })
    })
    .collect::<anyhow::Result<Vec<_>>>()?
    .into_iter()
    .collect();
    Ok(HirRawPack { crates })
}

// `rust_features` are features of the first-party crate; cargo rejects `--features` for
// third-party packages outside the workspace
fn compute_features<'a>(
    crate_name: &CrateName,
    rust_features: Option<&'a [String]>,
) -> Option<&'a [String]> {
    rust_features.filter(|_| crate_name.is_self_crate())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Passes `rust_features` only when expanding the first-party crate.
    #[test]
    fn compute_features_only_applies_to_self_crate() {
        let features = vec!["my_feature".to_owned()];

        assert_eq!(
            compute_features(&CrateName::self_crate(), Some(&features)),
            Some(features.as_slice())
        );
        assert_eq!(
            compute_features(&CrateName::new("dependency".into()), Some(&features)),
            None
        );
    }
}
