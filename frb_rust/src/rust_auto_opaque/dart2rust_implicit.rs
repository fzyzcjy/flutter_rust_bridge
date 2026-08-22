use crate::for_generated::{BaseArc, Lockable, LockableOrder, RustAutoOpaqueBase};
use crate::rust_auto_opaque::inner::RustAutoOpaqueInner;
use crate::rust_opaque::RustOpaqueBase;
use tokio::sync::RwLock;

// NOTE: Make these functions instead of methods, thus we can control its visibility by exporting
// only through `for_generated::...` and do not expose to end users.

// the function signature is not covered while the whole body is covered - looks like a bug in coverage tool
// frb-coverage:ignore-start
pub fn rust_auto_opaque_decode_owned<T, A: BaseArc<RustAutoOpaqueInner<T>>>(
    opaque: RustOpaqueBase<RustAutoOpaqueInner<T>, A>,
) -> T {
    // frb-coverage:ignore-end
    opaque.into_inner()
            .expect("Cannot convert RustOpaque to inner value. This is probably because you are having more than one references to it.")
            .data
            .into_inner()
}

pub fn rust_auto_opaque_encode<T, A: BaseArc<RustAutoOpaqueInner<T>>>(
    value: T,
) -> RustOpaqueBase<RustAutoOpaqueInner<T>, A> {
    RustOpaqueBase::new(RustAutoOpaqueInner::new(RwLock::new(value)))
}

pub fn rust_auto_opaque_lockable_order<T: Send + Sync, A: BaseArc<RustAutoOpaqueInner<T>>>(
    opaque: &RustAutoOpaqueBase<T, A>,
) -> LockableOrder {
    opaque.0.lockable_order()
}

#[cfg(test)]
mod tests {
    use super::{
        rust_auto_opaque_decode_owned, rust_auto_opaque_encode, rust_auto_opaque_lockable_order,
    };
    use crate::for_generated::Lockable;
    use crate::for_generated::StdArc;
    use crate::rust_auto_opaque::inner::RustAutoOpaqueInner;
    use crate::RustAutoOpaqueNom;

    /// Decodes an implicitly encoded opaque value with sole ownership.
    #[test]
    fn decodes_owned_value_when_no_clone_exists() {
        let encoded = rust_auto_opaque_encode::<i32, StdArc<RustAutoOpaqueInner<i32>>>(42);

        assert_eq!(rust_auto_opaque_decode_owned(encoded), 42);
    }

    /// Preserves the lock ordering identity through the opaque wrapper.
    #[test]
    fn returns_the_wrapped_lockable_order() {
        let opaque = RustAutoOpaqueNom::new(42);

        assert!(rust_auto_opaque_lockable_order(&opaque) == opaque.0.lockable_order());
    }
}
