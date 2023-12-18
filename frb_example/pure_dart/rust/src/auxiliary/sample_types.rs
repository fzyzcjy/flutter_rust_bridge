#![allow(dead_code, clippy::new_without_default)]

use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct MySize {
    pub width: i32,
    pub height: i32,
}

pub struct MyStruct {
    pub content: bool,
}

pub enum MyEnum {
    False,
    True,
}

#[derive(Debug)]
struct PrivateData {
    content: String,
    primitive: usize,
    array: [isize; 10],
    lifetime: &'static str,
}

#[derive(Debug)]
pub struct NonSendHideData {
    content: String,
    box_content: Option<Rc<PrivateData>>,
}

impl NonSendHideData {
    pub fn new() -> Self {
        Self {
            content: "content".to_owned(),
            box_content: Some(Rc::new(PrivateData {
                content: "content nested".to_owned(),
                primitive: 424242,
                array: [451; 10],
                lifetime: "static str",
            })),
        }
    }

    pub fn hide_data(&self) -> String {
        format!("{} - {:?}", self.content, self.box_content)
    }

    pub fn change_data(&mut self) {
        self.content = "MUT SELF".to_owned();
    }
}

#[derive(Debug)]
pub struct HideData {
    content: String,
    box_content: Option<Box<PrivateData>>,
}

impl HideData {
    pub fn new() -> Self {
        Self {
            content: "content".to_owned(),
            box_content: Some(Box::new(PrivateData {
                content: "content nested".to_owned(),
                primitive: 424242,
                array: [451; 10],
                lifetime: "static str",
            })),
        }
    }

    pub fn hide_data(&self) -> String {
        format!("{} - {:?}", self.content, self.box_content)
    }

    pub fn change_data(&mut self) {
        self.content = "MUT SELF".to_owned();
    }
}

#[derive(Debug)]
pub struct NonCloneData {
    content: String,
}

impl NonCloneData {
    pub fn new() -> Self {
        Self {
            content: "content".to_owned(),
        }
    }
    pub fn hide_data(&self) -> String {
        self.content.clone()
    }
}

/// Structure for testing the RustOpaque code generator.
/// FrbOpaqueReturn must be only return type.
/// FrbOpaqueReturn must not be used as an argument.
pub struct FrbOpaqueReturn;

/// Structure for testing the SyncReturn<RustOpaque> code generator.
/// FrbOpaqueSyncReturn must be only return type.
/// FrbOpaqueSyncReturn must should be without wrapper like Option<> Vec<> etc.
pub struct FrbOpaqueSyncReturn;

pub type Id = u64;
pub type UserIdAlias = Id;
pub type EnumAlias = MyEnum;
pub type StructAlias = MyStruct;
