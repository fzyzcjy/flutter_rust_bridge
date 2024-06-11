use crate::codec::BaseCodec;
use crate::for_generated::{BaseArc, StreamSinkBase};
use crate::generalized_isolate::{IntoDart, ZeroCopyBuffer};
#[cfg(feature = "rust-async")]
use crate::rust_auto_opaque::{inner::RustAutoOpaqueInner, RustAutoOpaqueBase};
use crate::rust_opaque::RustOpaqueBase;
use std::collections::{HashMap, HashSet};

/// Basically the Into trait.
/// We need this separate trait because we need to implement it for Vec<T> etc.
/// These blanket implementations allow us to accept external types in various places.
/// The initial reason for this was to allow mirrored types in StreamSink<>.
/// See also [PR 1285](https://github.com/fzyzcjy/flutter_rust_bridge/pull/1285)
pub trait IntoIntoDart<D: IntoDart> {
    fn into_into_dart(self) -> D;
}

impl<T, D> IntoIntoDart<Vec<D>> for Vec<T>
where
    T: IntoIntoDart<D>,
    Vec<D>: IntoDart,
    D: IntoDart,
{
    #[inline(always)]
    fn into_into_dart(self) -> Vec<D> {
        self.into_iter().map(|e| e.into_into_dart()).collect()
    }
}

impl<T, D> IntoIntoDart<Option<D>> for Option<T>
where
    T: IntoIntoDart<D>,
    D: IntoDart,
{
    #[inline(always)]
    fn into_into_dart(self) -> Option<D> {
        self.map(|e| e.into_into_dart())
    }
}

impl<T, A: BaseArc<T>> IntoIntoDart<RustOpaqueBase<T, A>> for RustOpaqueBase<T, A> {
    #[inline(always)]
    fn into_into_dart(self) -> RustOpaqueBase<T, A> {
        self
    }
}

#[cfg(feature = "rust-async")]
impl<T, A: BaseArc<RustAutoOpaqueInner<T>>> IntoIntoDart<RustAutoOpaqueBase<T, A>>
    for RustAutoOpaqueBase<T, A>
{
    #[inline(always)]
    fn into_into_dart(self) -> RustAutoOpaqueBase<T, A> {
        self
    }
}

impl<T, D> IntoIntoDart<ZeroCopyBuffer<D>> for ZeroCopyBuffer<T>
where
    T: IntoIntoDart<D>,
    D: IntoDart,
    ZeroCopyBuffer<D>: IntoDart,
{
    #[inline(always)]
    fn into_into_dart(self) -> ZeroCopyBuffer<D> {
        ZeroCopyBuffer(self.0.into_into_dart())
    }
}

impl<T, D, const C: usize> IntoIntoDart<[D; C]> for [T; C]
where
    T: IntoIntoDart<D>,
    [D; C]: IntoDart,
    D: IntoDart,
{
    #[inline(always)]
    fn into_into_dart(self) -> [D; C] {
        self.map(|e| e.into_into_dart())
    }
}

impl<T> IntoIntoDart<T> for Box<T>
where
    T: IntoDart,
{
    #[inline(always)]
    fn into_into_dart(self) -> T {
        *self
    }
}

impl<KT, KD, VT, VD> IntoIntoDart<HashMap<KD, VD>> for HashMap<KT, VT>
where
    KT: IntoIntoDart<KD>,
    VT: IntoIntoDart<VD>,
    HashMap<KD, VD>: IntoDart,
    KD: IntoDart + std::cmp::Eq + std::hash::Hash,
    VD: IntoDart,
{
    #[inline(always)]
    fn into_into_dart(self) -> HashMap<KD, VD> {
        self.into_iter()
            .map(|(k, v)| (k.into_into_dart(), v.into_into_dart()))
            .collect()
    }
}

impl<T, D> IntoIntoDart<HashSet<D>> for HashSet<T>
where
    T: IntoIntoDart<D>,
    HashSet<D>: IntoDart,
    D: IntoDart + Eq + std::hash::Hash,
{
    #[inline(always)]
    fn into_into_dart(self) -> HashSet<D> {
        self.into_iter().map(|e| e.into_into_dart()).collect()
    }
}

// frb-coverage:ignore-start
impl<T, Rust2DartCodec: BaseCodec> IntoIntoDart<StreamSinkBase<T, Rust2DartCodec>>
    for StreamSinkBase<T, Rust2DartCodec>
{
    fn into_into_dart(self) -> StreamSinkBase<T, Rust2DartCodec> {
        unreachable!()
    }
}
// frb-coverage:ignore-end

// ref: into_dart.rs
macro_rules! impl_into_into_dart_for_tuple {
    ($( ($($A:ident)+ ; $($AD:ident)+ ; $($N:tt)+) )*) => {$(
        impl<$($A: IntoIntoDart<$AD>, $AD: IntoDart),+> IntoIntoDart<($($AD),+,)> for ($($A),+,)
        where
            $($A: IntoIntoDart<$AD>, $AD: IntoDart),+,
        {
            #[inline(always)]
            fn into_into_dart(self) -> ($($AD),+,) {
                (
                    $(
                        self.$N.into_into_dart(),
                    )+
                )
            }
        }
    )*};
}

impl_into_into_dart_for_tuple! {
    (A ; AD ; 0)
    (A B ; AD BD ; 0 1)
    (A B C ; AD BD CD ; 0 1 2)
    (A B C D ; AD BD CD DD ; 0 1 2 3)
    (A B C D E ; AD BD CD DD ED ; 0 1 2 3 4)
    (A B C D E F ; AD BD CD DD ED FD ; 0 1 2 3 4 5)
    (A B C D E F G ; AD BD CD DD ED FD GD ; 0 1 2 3 4 5 6)
    (A B C D E F G H ; AD BD CD DD ED FD GD HD ; 0 1 2 3 4 5 6 7)
    (A B C D E F G H I ; AD BD CD DD ED FD GD HD ID ; 0 1 2 3 4 5 6 7 8)
    (A B C D E F G H I J ; AD BD CD DD ED FD GD HD ID JD ; 0 1 2 3 4 5 6 7 8 9)
}

// more generic impls do not work because they crate possibly conflicting trait impls
// this is why here are some more specific impls

// Implementations for simple types
macro_rules! impl_into_into_dart_by_self {
    ($t:ty) => {
        impl IntoIntoDart<$t> for $t {
            #[inline(always)]
            fn into_into_dart(self) -> $t {
                self
            }
        }
    };
}

// Impls for primitive types are taken from the IntoDart trait

impl_into_into_dart_by_self!(u8);
impl_into_into_dart_by_self!(i8);
impl_into_into_dart_by_self!(u16);
impl_into_into_dart_by_self!(i16);
impl_into_into_dart_by_self!(u32);
impl_into_into_dart_by_self!(i32);
impl_into_into_dart_by_self!(u64);
impl_into_into_dart_by_self!(i64);
impl_into_into_dart_by_self!(u128);
impl_into_into_dart_by_self!(i128);
impl_into_into_dart_by_self!(f32);
impl_into_into_dart_by_self!(f64);
impl_into_into_dart_by_self!(bool);
impl_into_into_dart_by_self!(());
impl_into_into_dart_by_self!(usize);
impl_into_into_dart_by_self!(isize);
impl_into_into_dart_by_self!(String);
impl_into_into_dart_by_self!(char);
#[cfg(feature = "dart-opaque")]
impl_into_into_dart_by_self!(crate::dart_opaque::DartOpaque);
#[cfg(not(target_family = "wasm"))]
impl_into_into_dart_by_self!(allo_isolate::ffi::DartCObject);
#[cfg(target_family = "wasm")]
impl_into_into_dart_by_self!(wasm_bindgen::JsValue);
#[cfg(feature = "uuid")]
impl_into_into_dart_by_self!(uuid::Uuid);
#[cfg(feature = "backtrace")]
impl_into_into_dart_by_self!(backtrace::Backtrace);
#[cfg(feature = "anyhow")]
impl_into_into_dart_by_self!(anyhow::Error);
// TODO await upstream
// impl_into_into_dart_by_self!(std::backtrace::Backtrace);

#[cfg(feature = "chrono")]
mod chrono_impls {
    use super::IntoIntoDart;
    use chrono::{Local, Utc};
    impl_into_into_dart_by_self!(chrono::Duration);
    impl_into_into_dart_by_self!(chrono::NaiveDateTime);
    impl_into_into_dart_by_self!(chrono::DateTime<Local>);
    impl_into_into_dart_by_self!(chrono::DateTime<Utc>);
}

#[cfg(test)]
mod tests {
    #[cfg(not(target_family = "wasm"))]
    #[test]
    fn test_zero_copy_buffer() {
        use crate::misc::into_into_dart::IntoIntoDart;
        let raw: allo_isolate::ZeroCopyBuffer<Vec<u8>> = allo_isolate::ZeroCopyBuffer(vec![10]);
        assert_eq!(raw.into_into_dart().0, vec![10]);
    }
}
