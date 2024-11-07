use flutter_rust_bridge::frb;
pub use xml_parser::*;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn load_xml_file(file_path: &str) -> Option<XmlReadout> {
    panic!()
}
