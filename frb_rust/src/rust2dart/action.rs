use crate::generalized_isolate::IntoDart;
use crate::platform_types::DartAbi;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rust2DartAction {
    Success = 0,
    Error = 1, // TODO rename?
    CloseStream = 2,
    Panic = 3,
}

impl IntoDart for Rust2DartAction {
    fn into_dart(self) -> DartAbi {
        (self as i32).into_dart()
    }
}

#[cfg(test)]
mod tests {
    use super::Rust2DartAction;

    /// Preserves the stable Rust-to-Dart action discriminants.
    #[test]
    fn test_action_discriminants_match_the_abi_contract() {
        assert_eq!(Rust2DartAction::Success as i32, 0);
        assert_eq!(Rust2DartAction::Error as i32, 1);
        assert_eq!(Rust2DartAction::CloseStream as i32, 2);
        assert_eq!(Rust2DartAction::Panic as i32, 3);
    }
}
