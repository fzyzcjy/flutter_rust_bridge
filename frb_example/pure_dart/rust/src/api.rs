use std::{sync::RwLock};

use flutter_rust_bridge::{DartSafe, Opaque};
use std::fmt::Debug;

use crate::data::{Magic, TestOpaque};

pub trait DartDebug: DartSafe + Debug {}
impl<T: DartSafe + Debug> DartDebug for T {}

pub trait Wtf {
    fn nested(&self) -> String;
}

pub trait Wtffi: DartSafe + Wtf {}
impl<T: DartSafe + Wtf> Wtffi for T {}

#[derive(Debug)]
pub struct OpaqueBag {
    pub primitive: Opaque<RwLock<i32>>,
    pub array: Opaque<RwLock<[isize; 10]>>,
    pub lifetime: Opaque<&'static str>,
    pub trait_obj: Opaque<Box<dyn DartDebug>>,
}

pub fn handle_opaque_aaa() -> anyhow::Result<TestOpaque> {
    Ok(TestOpaque::new())
}

pub fn magic() -> Opaque<Box<RwLock<dyn Wtffi>>> {
    Opaque::new(Box::new(RwLock::new(Magic {
        message: "MAGIC 1".to_owned(),
        nested: Some(Box::new(Magic {
            message: "NESTED MAGIC".to_owned(),
            nested: None,
        })),
    })))
}

pub fn handle_magic(magic: Opaque<Box<RwLock<dyn Wtffi>>>) -> String {
    magic.as_deref().unwrap().read().unwrap().nested()
}

pub fn handle_opaque_bbb(value: Option<TestOpaque>) -> String {
    value.map(|a| a.nested()).unwrap_or_default()
}

pub fn handle_opaque(value: Option<OpaqueBag>) -> anyhow::Result<OpaqueBag> {
    Ok(value
        .map(|val| {
            if let Some(Ok(mut val)) = val.primitive.write() {
                *val += 1;
            }
            if let Some(Ok(mut val)) = val.array.write() {
                for i in val.iter_mut() {
                    *i += 1;
                }
            }
            val
        })
        .unwrap_or_else(|| OpaqueBag {
            primitive: Opaque::new(RwLock::new(0)),
            array: Opaque::new(RwLock::new([0; 10])),
            lifetime: Opaque::new("Hello there."),
            trait_obj: Opaque::new(Box::new(("first", "second"))),
        }))
}

pub fn handle_opaque_repr(value: Opaque<RwLock<i32>>) -> anyhow::Result<Option<String>> {
    Ok(if let Some(Ok(value)) = value.read() {
        Some(value.to_string())
    } else {
        None
    })
}
