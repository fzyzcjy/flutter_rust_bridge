use crate::ir::IrFile;
use crate::utils::misc::{BlockIndex, ExtraTraitForVec};
use crate::{parser, transformer};
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Parsed configs, mainly used for internal logic
#[derive(Debug, Clone, serde::Serialize)]
pub struct Opts {
    pub rust_input_path: String,
    pub dart_output_path: String,
    pub dart_decl_output_path: Option<String>,
    pub c_output_path: Vec<String>,
    pub rust_crate_dir: String,
    pub rust_output_path: String,
    pub class_name: String,
    pub dart_format_line_length: u32,
    pub dart_enums_style: bool,
    pub skip_add_mod_to_lib: bool,
    pub llvm_path: Vec<String>,
    pub llvm_compiler_opts: String,
    pub manifest_path: String,
    pub dart_root: Option<String>,
    pub build_runner: bool,
    pub block_index: BlockIndex,
    pub skip_deps_check: bool,
    pub wasm_enabled: bool,
    pub inline_rust: bool,
    pub shared: bool, // it is true if this Opts instance is for auto-generated shared API block. Otherwise, it is false,
    // for the below 2 fields:
    // whatever the Opts is for regular or auto-generated shared API block,
    // and whatever there is or no shared stuff,
    // these fields should always contain (default) shared paths.
    pub shared_rust_output_path: String,
    pub shared_dart_output_path: String,
}

impl Opts {
    /// NOTE: for `Opts` for a regular API block, if `all_configs` is empty or with only 1 element,
    /// it would return an `IrFile` with field of EMPTY `shared_types` for the regular block;
    /// for `Opts` for an auto-generated shared API block, make sure `all_configs` has at least 2 items, each of which
    /// is for the regular block. Otherwise, it would panic.
    pub fn get_ir_file(&self, all_configs: &[Opts]) -> Result<IrFile> {
        let raw_ir_file = if !self.shared {
            self.get_regular_ir_file(all_configs)?
        } else {
            self.get_shared_ir_file(all_configs)?
        };
        log::debug!("Phase: Transform IR");
        Ok(transformer::transform(raw_ir_file))
    }

    fn get_regular_ir_file(&self, all_configs: &[Opts]) -> Result<IrFile> {
        log::debug!("Phase: Parse source code to AST");
        let source_rust_content = fs::read_to_string(&self.rust_input_path).with_context(|| {
            let err_msg = format!(
                "Failed to read rust input file \"{}\"",
                self.rust_input_path
            );
            log::error!("{}", err_msg);
            err_msg
        })?;
        let file_ast = syn::parse_file(&source_rust_content).unwrap();

        log::debug!("Phase: Parse AST to IR");
        Ok(parser::parse(
            &source_rust_content,
            file_ast,
            &self.manifest_path,
            self.block_index,
            self.shared,
            all_configs,
        ))
    }

    fn get_shared_ir_file(&self, all_configs: &[Opts]) -> Result<IrFile> {
        log::debug!("get_shared_ir_file 1"); //TODO: delete
        if all_configs.len() <= 1 {
            panic!("`get_shared_ir_file(..)` should not be called when all_configs.len()<=1")
        }
        let (regular_configs, shared_index) = if !all_configs.last().unwrap().shared {
            (all_configs, all_configs.len())
        } else {
            let last_index = all_configs.len() - 1;
            (&all_configs[0..last_index], last_index)
        };

        log::debug!("get_shared_ir_file 2"); //TODO: delete
        log::debug!("configs len:{}", regular_configs.len()); //TODO: delete

        assert!(regular_configs.len() > 1);
        assert!(regular_configs.iter().all(|c| !c.shared));

        log::debug!("get_shared_ir_file 3"); //TODO: delete

        // get regular ir_files
        let funcs = Vec::new(); // TODO:
        let mut structs = Vec::new();
        let mut enums = Vec::new();
        for config in regular_configs {
            let ir_file = config.get_ir_file(&[])?;
            // funcs.extend(raw_ir_file.funcs);// TODO: only methods of shared structs needed
            structs.extend(ir_file.struct_pool);
            enums.extend(ir_file.enum_pool);
        }
        let shared_structs = structs.find_duplicates(true);
        let shared_enums = enums.find_duplicates(true);

        let struct_pool = shared_structs
            .into_iter()
            .map(|x| (x.0, x.1))
            .collect::<HashMap<_, _>>();
        let enum_pool = shared_enums
            .into_iter()
            .map(|x| (x.0, x.1))
            .collect::<HashMap<_, _>>();

        log::debug!("the shared struct_pool:{:?}", struct_pool); //TODO: delete
        log::debug!("the shared enum_pool:{:?}", enum_pool); //TODO: delete
        Ok(IrFile::new(
            funcs, // this field would effect `IrFile.visit_types(...)` and others
            struct_pool,
            enum_pool,
            true, // set true, in case for the methods of a shared struct,
            BlockIndex(shared_index),
            true,
            all_configs,
        ))
    }

    pub fn dart_api_class_name(&self) -> String {
        self.class_name.clone()
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
}

pub struct PathForGeneration {
    pub base_path: PathBuf,
    pub io_path: PathBuf,
    pub wasm_path: PathBuf,
}
