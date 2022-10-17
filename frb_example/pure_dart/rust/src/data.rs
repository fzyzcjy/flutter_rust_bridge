
use flutter_rust_bridge::error;

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
}

impl Drop for PrivateData {
    fn drop(&mut self) {
        error("TAKI DROP");
    }
}

#[derive(Debug)]
pub struct HideData {
    content: String,
    box_content: Option<Box<PrivateData>>,
}

impl Drop for HideData {
    fn drop(&mut self) {
        error("TAKI DROP HIDE");
    }
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

    pub fn hide_data(&self) -> String {
        format!("{} - {:?}", self.content, self.box_content)
    }
}
