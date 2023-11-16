use crate::ir::IrPack;
use crate::parser::{self, anyhow::Result};
use crate::utils::misc::{read_rust_file, BlockIndex};
use convert_case::{Case, Casing};
use std::path::{Path, PathBuf};

pub struct Opts {}

impl Opts {
    // moved to parser
    // pub fn get_ir_pack(&self) -> anyhow::Result<IrPack> {
    //     // info!("Phase: Parse source code to AST");
    //     let source_rust_content = read_rust_file(&PathBuf::from(&self.rust_input_path));
    //     let file_ast = syn::parse_file(&source_rust_content)?;
    //
    //     // info!("Phase: Parse AST to IR");
    //     parser::parse(&source_rust_content, file_ast, &self.manifest_path)
    // }

    pub fn dart_api_class_name(&self) -> &str {
        &self.class_name
    }

    pub fn dart_api_impl_class_name(&self) -> String {
        format!("{}Impl", self.class_name)
    }

    pub fn dart_wire_class_name(&self) -> String {
        format!("{}Wire", self.class_name)
    }

    pub fn dart_platform_class_name(&self) -> String {
        format!("{}Platform", self.class_name)
    }

    pub fn dart_wasm_module(&self) -> String {
        format!("{}WasmModule", self.class_name)
    }

    pub(crate) fn dart_output_root(&self) -> Option<&str> {
        Path::new(
            self.dart_decl_output_path
                .as_ref()
                .unwrap_or(&self.dart_output_path),
        )
        .file_stem()?
        .to_str()
    }

    pub fn dart_wasm_output_path(&self) -> PathBuf {
        Path::new(&self.dart_output_path).with_extension("web.dart")
    }

    pub fn dart_io_output_path(&self) -> PathBuf {
        Path::new(&self.dart_output_path).with_extension("io.dart")
    }

    pub fn dart_common_output_path(&self) -> PathBuf {
        Path::new(&self.dart_output_path).with_extension("common.dart")
    }

    pub fn rust_wasm_output_path(&self) -> PathBuf {
        Path::new(&self.rust_output_path).with_extension("web.rs")
    }

    pub fn rust_io_output_path(&self) -> PathBuf {
        Path::new(&self.rust_output_path).with_extension("io.rs")
    }

    pub fn dart_root_or_default(&self) -> String {
        self.dart_root
            .clone()
            .unwrap_or_else(|| env!("CARGO_MANIFEST_DIR").to_string())
    }

    pub fn dart_freezed_path(&self) -> PathBuf {
        PathBuf::from(
            self.dart_decl_output_path
                .as_deref()
                .unwrap_or(&self.dart_output_path),
        )
        .with_extension("freezed.dart")
    }

    pub fn get_rust_output_paths(&self) -> PathForGeneration {
        PathForGeneration {
            base_path: PathBuf::from(self.rust_output_path.clone()),
            io_path: self.rust_io_output_path(),
            wasm_path: self.rust_wasm_output_path(),
        }
    }

    pub fn get_dart_output_paths(&self) -> PathForGeneration {
        PathForGeneration {
            base_path: PathBuf::from(self.dart_output_path.clone()),
            io_path: self.dart_io_output_path(),
            wasm_path: self.dart_wasm_output_path(),
        }
    }
    pub fn get_dart_api_bridge_name(&self) -> String {
        if self.use_bridge_in_method {
            "bridge".to_owned()
        } else {
            Path::new(&self.rust_input_path)
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned()
                .to_case(Case::Camel)
        }
    }
}

pub struct PathForGeneration {
    pub base_path: PathBuf,
    pub io_path: PathBuf,
    pub wasm_path: PathBuf,
}
