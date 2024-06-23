// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

#![allow(dead_code, clippy::new_without_default)]

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
pub struct HideDataRaw {
    content: String,
    box_content: Option<Box<PrivateData>>,
}

impl HideDataRaw {
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
        "MUT SELF".clone_into(&mut self.content);
    }
}

#[derive(Debug)]
pub struct NonCloneDataRaw {
    content: String,
}

impl NonCloneDataRaw {
    pub fn new() -> Self {
        Self {
            content: "content".to_owned(),
        }
    }
    pub fn hide_data(&self) -> String {
        self.content.clone()
    }
}

pub type Id = u64;
pub type UserIdAlias = Id;
pub type EnumAlias = MyEnum;
pub type StructAlias = MyStruct;
