use flutter_rust_bridge::{opaque::Opaque, DartSafe};

use crate::api::{Wtf, Wtffi};

pub struct MyStruct {
    pub content: bool,
}

pub enum MyEnum {
    False,
    True,
}

pub struct Magic {
    pub message: String,
    pub nested: Option<Box<Magic>>,
}

impl Drop for Magic {
    fn drop(&mut self) {
        println!("AHAHAHAHAHA");
    }
}

impl Wtf for Magic {
    fn nested(&self) -> String {
        let current = &self.message;
        if let Some(next) = &self.nested {
            return format!("{}{}", current, next.nested());
        }
        current.clone()
    }
}

pub struct TestOpaque {
    pub magic: Opaque<Box<dyn Wtffi>>,
}

impl TestOpaque {
    pub fn new() -> Self {
        Self {
            magic: Opaque::new(Box::new(Magic {
                message: "MAGIC 1".to_owned(),
                nested: Some(Box::new(Magic {
                    message: "NESTED MAGIC".to_owned(),
                    nested: None,
                })),
            })),
        }
    }

    pub fn nested(&self) -> String {
        self.magic.as_deref().map(|a| a.nested()).unwrap_or_default()
    }
}
