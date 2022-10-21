#![allow(dead_code)]

use std::cell::RefCell;
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
pub struct HideSyncData {
    content: RefCell<String>,
    box_content: Option<Box<PrivateData>>,
}

impl HideSyncData {
    pub fn new() -> Self {
        Self {
            content: RefCell::new("content".to_owned()),
            box_content: Some(Box::new(PrivateData {
                content: "content nested".to_owned(),
                primitive: 424242,
                array: [451; 10],
                lifetime: "static str",
            })),
        }
    }

    pub fn hide_data(&self) -> String {
        format!("{} - {:?}", self.content.borrow(), self.box_content)
    }

    pub fn change_data(&mut self) {
        self.content.replace("MUT SELF".to_owned());
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
