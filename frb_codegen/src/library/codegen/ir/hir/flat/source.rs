use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize)]
pub(crate) enum HirFlatGenerationSource {
    Normal,
    CopyFromTraitDef,
    MoveFromCrateThirdPartyFolder,
}
