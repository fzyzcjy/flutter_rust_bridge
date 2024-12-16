use crate::for_generated::BaseArc;
use crate::generalized_isolate::ZeroCopyBuffer;
use crate::platform_types::DartAbi;
#[cfg(feature = "rust-async")]
use crate::rust_auto_opaque::{inner::RustAutoOpaqueInner, RustAutoOpaqueBase};
use crate::rust_opaque::RustOpaqueBase;
use js_sys::Array;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use wasm_bindgen::JsValue;

pub trait IntoDart {
    fn into_dart(self) -> DartAbi;
}

pub trait IntoDartExceptPrimitive: IntoDart {}
impl IntoDartExceptPrimitive for JsValue {}
impl<T, A: BaseArc<T>> IntoDartExceptPrimitive for RustOpaqueBase<T, A> {}
#[cfg(feature = "rust-async")]
impl<T, A: BaseArc<RustAutoOpaqueInner<T>>> IntoDartExceptPrimitive for RustAutoOpaqueBase<T, A> {}
#[cfg(feature = "dart-opaque")]
impl IntoDartExceptPrimitive for crate::dart_opaque::DartOpaque {}
impl IntoDartExceptPrimitive for String {}
impl IntoDartExceptPrimitive for bool {}
impl<T: IntoDart> IntoDartExceptPrimitive for Option<T> {}

impl IntoDart for () {
    #[inline]
    fn into_dart(self) -> DartAbi {
        JsValue::undefined()
    }
}
#[cfg(feature = "chrono")]
impl<Tz: chrono::TimeZone> IntoDart for chrono::DateTime<Tz> {
    #[inline]
    fn into_dart(self) -> DartAbi {
        self.timestamp_millis().into_dart()
    }
}
#[cfg(feature = "chrono")]
impl IntoDart for chrono::NaiveDateTime {
    #[inline]
    fn into_dart(self) -> DartAbi {
        self.timestamp_millis().into_dart()
    }
}
#[cfg(feature = "chrono")]
impl IntoDart for chrono::Duration {
    #[inline]
    fn into_dart(self) -> DartAbi {
        self.num_milliseconds().into_dart()
    }
}
#[cfg(feature = "chrono")]
impl<Tz: chrono::TimeZone> IntoDart for Vec<chrono::DateTime<Tz>> {
    fn into_dart(self) -> DartAbi {
        self.into_iter()
            .map(IntoDart::into_dart)
            .collect::<Vec<_>>()
            .into_dart()
    }
}
#[cfg(feature = "chrono")]
impl IntoDart for Vec<chrono::NaiveDateTime> {
    fn into_dart(self) -> DartAbi {
        self.into_iter()
            .map(IntoDart::into_dart)
            .collect::<Vec<_>>()
            .into_dart()
    }
}
#[cfg(feature = "chrono")]
impl IntoDart for Vec<chrono::Duration> {
    fn into_dart(self) -> DartAbi {
        self.into_iter()
            .map(IntoDart::into_dart)
            .collect::<Vec<_>>()
            .into_dart()
    }
}

#[cfg(feature = "uuid")]
impl IntoDart for uuid::Uuid {
    #[inline]
    fn into_dart(self) -> DartAbi {
        self.as_bytes().to_vec().into_dart()
    }
}

#[cfg(feature = "uuid")]
impl IntoDart for Vec<uuid::Uuid> {
    #[inline]
    fn into_dart(self) -> DartAbi {
        use std::io::Write;
        let mut buffer =
            Vec::<u8>::with_capacity(self.len() * crate::misc::manual_impl::UUID_SIZE_IN_BYTES);
        for id in self {
            let _ = buffer.write(id.as_bytes());
        }
        js_sys::Uint8Array::from(buffer.as_slice()).into()
    }
}

#[cfg(feature = "backtrace")]
impl IntoDart for backtrace::Backtrace {
    fn into_dart(self) -> DartAbi {
        format!("{:?}", self).into_dart()
    }
}

macro_rules! delegate {
    ($( $ty:ty )*) => {$(
        impl IntoDart for $ty {
            #[inline]
            fn into_dart(self) -> DartAbi {
                DartAbi::from(self)
            }
        }
    )*};
}
macro_rules! delegate_buffer {
    ($( $ty:ty => $buffer:ty )*) => {$(
        impl IntoDart for Vec<$ty> {
            #[inline]
            fn into_dart(self) -> DartAbi {
                <$buffer>::from(self.as_slice()).into()
            }
        }

        impl IntoDart for ZeroCopyBuffer<Vec<$ty>> {
            #[inline]
            fn into_dart(self) -> DartAbi {
                self.0.into_dart()
            }
        }

        impl IntoDartExceptPrimitive for Vec<$ty> {}

        impl IntoDart for HashSet<$ty> {
            #[inline]
            fn into_dart(self) -> DartAbi {
                self.into_iter().collect::<Vec<_>>().into_dart()
            }
        }

        impl IntoDartExceptPrimitive for HashSet<$ty> {}
    )*};
}
// Orphan rules disallow blanket implementations, so we have to manually delegate here.
delegate! {
    bool
    i8 u8 i16 u16 i32 u32 i64 u64 isize usize
    f32 f64
    &str String JsValue
}
delegate_buffer! {
    i8 => js_sys::Int8Array
    u8 => js_sys::Uint8Array
    i16 => js_sys::Int16Array
    u16 => js_sys::Uint16Array
    i32 => js_sys::Int32Array
    u32 => js_sys::Uint32Array
    f32 => js_sys::Float32Array
    f64 => js_sys::Float64Array
}

impl IntoDart for char {
    #[inline]
    fn into_dart(self) -> DartAbi {
        (self as u32).into_dart()
    }
}

impl IntoDart for i128 {
    #[inline]
    fn into_dart(self) -> DartAbi {
        self.to_string().into_dart()
    }
}
impl IntoDart for u128 {
    #[inline]
    fn into_dart(self) -> DartAbi {
        self.to_string().into_dart()
    }
}

fn into_dart_iterator<T, It>(iter: It) -> DartAbi
where
    T: IntoDart,
    It: Iterator<Item = T>,
{
    Array::from_iter(iter.map(IntoDart::into_dart)).into()
}

impl<T: IntoDartExceptPrimitive> IntoDart for Vec<T> {
    #[inline]
    fn into_dart(self) -> DartAbi {
        into_dart_iterator(self.into_iter())
    }
}

impl<T: IntoDartExceptPrimitive> IntoDart for HashSet<T> {
    #[inline]
    fn into_dart(self) -> DartAbi {
        into_dart_iterator(self.into_iter())
    }
}

impl<T: IntoDartExceptPrimitive> IntoDartExceptPrimitive for HashSet<T> {}

impl<K, V> IntoDart for HashMap<K, V>
where
    K: IntoDart,
    V: IntoDart,
{
    #[inline]
    fn into_dart(self) -> DartAbi {
        into_dart_iterator(self.into_iter())
    }
}

impl<K, V> IntoDartExceptPrimitive for HashMap<K, V>
where
    K: IntoDart,
    V: IntoDart,
{
}

impl<T: IntoDart> IntoDart for Option<T> {
    #[inline]
    fn into_dart(self) -> DartAbi {
        self.map(T::into_dart).unwrap_or_else(JsValue::null)
    }
}
impl<T> IntoDart for *const T {
    #[inline]
    fn into_dart(self) -> DartAbi {
        (self as usize).into_dart()
    }
}
impl<T> IntoDart for *mut T {
    #[inline]
    fn into_dart(self) -> DartAbi {
        (self as usize).into_dart()
    }
}

impl<T, A: BaseArc<T>> IntoDart for RustOpaqueBase<T, A> {
    #[inline]
    fn into_dart(self) -> DartAbi {
        self.into()
    }
}

#[cfg(feature = "rust-async")]
impl<T, A: BaseArc<RustAutoOpaqueInner<T>>> IntoDart for RustAutoOpaqueBase<T, A> {
    #[inline]
    fn into_dart(self) -> DartAbi {
        self.into()
    }
}

#[cfg(feature = "dart-opaque")]
impl IntoDart for crate::dart_opaque::DartOpaque {
    #[inline]
    fn into_dart(self) -> DartAbi {
        self.into()
    }
}

impl<const N: usize, T: IntoDartExceptPrimitive> IntoDart for [T; N] {
    #[inline]
    fn into_dart(self) -> DartAbi {
        let boxed: Box<[T]> = Box::new(self);
        boxed.into_vec().into_dart()
    }
}

macro_rules! impl_into_dart_for_primitive {
    ($($prim:ty)*) => {$(
        impl<const N: usize> IntoDart for [$prim; N] {
            #[inline]
            fn into_dart(self) -> DartAbi {
                Vec::from(self).into_dart()
            }
        }
    )*};
}

impl_into_dart_for_primitive!(i8 u8 i16 u16 i32 u32 f32 f64);

macro_rules! delegate_big_buffers {
    ($($buf:ty)*) => {$(
        impl IntoDart for $buf {
            fn into_dart(self) -> DartAbi {
                into_dart_iterator(self.into_iter())
                // let buf: &[i32] = bytemuck::cast_slice(&self[..]);
                // let buf = Int32Array::from(buf);
                // <$buffer>::new(&buf.buffer()).into()
            }
        }
    )*};
}
delegate_big_buffers! {
    Vec<i64>
    Vec<u64>
}

macro_rules! impl_into_dart_for_tuple {
    ($( ($($T:ident)+) )*) => {$(
        impl<$($T: IntoDart),+> IntoDart for ($($T),+,) {
            #[allow(non_snake_case)]
            fn into_dart(self) -> DartAbi {
                let ($($T),+,) = self;
                vec![$($T.into_dart()),+].into_dart()
            }
        }

        impl<$($T: IntoDart),+> IntoDartExceptPrimitive for ($($T),+,) {}
    )*};
}

impl_into_dart_for_tuple! {
    (A)
    (A B)
    (A B C)
    (A B C D)
    (A B C D E)
    (A B C D E F)
    (A B C D E F G)
    (A B C D E F G H)
    (A B C D E F G H I)
    (A B C D E F G H I J)
}

impl IntoDart for ZeroCopyBuffer<Vec<i64>> {
    #[inline]
    fn into_dart(self) -> DartAbi {
        self.0.into_dart()
    }
}
impl IntoDart for ZeroCopyBuffer<Vec<u64>> {
    #[inline]
    fn into_dart(self) -> DartAbi {
        self.0.into_dart()
    }
}

#[cfg(feature = "anyhow")]
impl IntoDart for anyhow::Error {
    fn into_dart(self) -> DartAbi {
        format!("{:?}", self).into_dart()
    }
}
