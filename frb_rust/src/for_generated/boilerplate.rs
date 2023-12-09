// Why put these code as boilerplate instead of putting in frb_rust crate directly?
// Because we need to avoid the Rust's `orphan rule`.
#[macro_export]
macro_rules! frb_generated_boilerplate {
    () => {
        pub trait CstDecode<T> {
            fn cst_decode(self) -> T;
        }

        impl<T, S> CstDecode<Option<T>> for *mut S
        where
            *mut S: CstDecode<T>,
        {
            fn cst_decode(self) -> Option<T> {
                (!self.is_null()).then(|| self.cst_decode())
            }
        }
    };
}
