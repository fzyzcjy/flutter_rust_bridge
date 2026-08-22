use crate::enum_map;
use anyhow::bail;
use serde::Serialize;
use std::convert::{TryFrom, TryInto};
use strum_macros::{Display, EnumIter};

#[derive(Debug, Clone, Copy, Eq, PartialEq, EnumIter, Display, Serialize, Hash)]
pub enum Target {
    Io,
    Web,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, EnumIter, Display, Serialize, Hash)]
pub enum TargetOrCommon {
    Common,
    Io,
    Web,
}

enum_map!(
    TargetOrCommonMap, TargetOrCommon;
    Common, Io, Web;
    common, io, web;
);

impl TryFrom<TargetOrCommon> for Target {
    type Error = anyhow::Error;

    fn try_from(src: TargetOrCommon) -> Result<Self, Self::Error> {
        Ok(match src {
            // This will stop the whole generator and tell the users, so we do not care about testing it
            // frb-coverage:ignore-start
            TargetOrCommon::Common => bail!("Cannot convert TargetOrCommon::Common to Target"),
            // frb-coverage:ignore-end
            TargetOrCommon::Io => Target::Io,
            TargetOrCommon::Web => Target::Web,
        })
    }
}

impl From<Target> for TargetOrCommon {
    fn from(value: Target) -> Self {
        match value {
            Target::Io => TargetOrCommon::Io,
            Target::Web => TargetOrCommon::Web,
        }
    }
}

impl TargetOrCommon {
    pub(crate) fn as_target_or(&self, when_common: Target) -> Target {
        match self {
            TargetOrCommon::Common => when_common,
            TargetOrCommon::Io | TargetOrCommon::Web => (*self).try_into().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Converts IO and web target values in both directions without remapping them.
    #[test]
    fn converts_platform_targets_bidirectionally() {
        assert_eq!(Target::try_from(TargetOrCommon::Io).unwrap(), Target::Io);
        assert_eq!(Target::try_from(TargetOrCommon::Web).unwrap(), Target::Web);
        assert_eq!(TargetOrCommon::from(Target::Io), TargetOrCommon::Io);
        assert_eq!(TargetOrCommon::from(Target::Web), TargetOrCommon::Web);
    }

    /// Uses the caller-provided fallback only for the common target.
    #[test]
    fn resolves_common_and_platform_targets() {
        assert_eq!(TargetOrCommon::Common.as_target_or(Target::Io), Target::Io);
        assert_eq!(TargetOrCommon::Io.as_target_or(Target::Web), Target::Io);
        assert_eq!(TargetOrCommon::Web.as_target_or(Target::Io), Target::Web);
    }
}
