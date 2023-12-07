use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Hash)]
pub(crate) enum TransferMode {
    Cst,
    Dco,
    Sse,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Hash)]
pub(crate) struct TransferModePack {
    pub dart2rust: TransferMode,
    pub rust2dart: TransferMode,
}

#[doc(hidden)]
#[macro_export]
macro_rules! codegen_transfer_structs {
    ($enum_name:ident) => (
        crate::codegen_transfer_structs!(
            @private

            $enum_name;

            Cst,
            Dco,
            Sse,
        );
    );
    (@private $enum_name:ident ; $($name:ident),*,) => (
        paste::paste! {
            #[enum_dispatch([<$enum_name Trait>])]
            pub(crate) enum $enum_name {
                $(
                $name([<$name $enum_name>]),
                )*
            }

            impl $enum_name {
                pub(crate) fn new(mode: TransferMode) -> Self {
                    match mode {
                        $(
                        TransferMode::$name => Self::$name([<$name $enum_name>] {}),
                        )*
                    }
                }
            }
        }
    )
}
