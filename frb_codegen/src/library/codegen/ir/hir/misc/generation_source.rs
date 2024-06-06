use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
pub(crate) enum HirGenerationSource {
    Normal,
    CopyFromTraitDef,
    MoveFromCrateThirdPartyFolder,
    FromFrbOverride,
}
