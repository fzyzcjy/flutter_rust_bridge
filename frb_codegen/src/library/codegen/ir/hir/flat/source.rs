use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
pub(crate) enum HirFlatGenerationSource {
    Normal,
    CopyFromTraitDef,
    MoveFromCrateThirdPartyFolder,
}
