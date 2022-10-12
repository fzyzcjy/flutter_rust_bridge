use std::sync::RwLock;

use flutter_rust_bridge::{DartSafe, opaque::Opaque};
use std::fmt::Debug;



pub trait DartDebug: DartSafe + Debug {}
impl<T: DartSafe + Debug> DartDebug for T {}

#[derive(Debug)]
pub struct OpaqueBag {
    pub primitive: Opaque<RwLock<i32>>,
    pub array: Opaque<RwLock<[isize; 10]>>,
    pub lifetime: Opaque<&'static str>,
    pub trait_obj: Opaque<Box<dyn DartDebug>>,
}

pub struct TestBag {
    pub test: String
}

pub fn test42() -> Box<TestBag> {
    Box::new(TestBag { test: "WOT TAK".to_owned() })
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