use flutter_rust_bridge::DartSafe;

use crate::api::OpaqueRun;

pub struct MyStruct {
    pub content: bool,
}

pub enum MyEnum {
    False,
    True,
}

#[derive(Debug)]
struct PrivateData {
    content: String
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
            })),
        }
    }
}

impl OpaqueRun for HideData {
    fn hide_data(&self) -> String {
        format!("{} - {:?}", self.content, self.box_content)
    }
}
