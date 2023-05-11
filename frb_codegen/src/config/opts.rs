use crate::ir::IrFile;
use crate::utils::misc::{BlockIndex, ExtraTraitForVec};
use crate::{parser, transformer};
use anyhow::{Context, Result};
use convert_case::{Case, Casing};
use std::collections::{HashMap, HashSet};
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
    pub bridge_in_method: bool,
    pub extra_headers: String,
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

    // TODO: need or not?
    // fn find_impl_block(&self, s: String, v: &[&str]) -> String {
    //     let mut result_string = "".to_string();
    //     for struct_name in v {
    //         let my_r = &format!(r"impl\s+{}\s*\{{([^}}]*)\}}", struct_name); // correct
    //         let re: Regex = Regex::new(my_r).unwrap();
    //         for matched in re.find_iter(&s) {
    //             result_string += &format!("{}\n", matched.as_str());
    //         }
    //     }
    //     result_string
    // }

    fn get_regular_ir_file(&self, all_configs: &[Opts]) -> Result<IrFile> {
        log::debug!("Phase: Parse source code to AST");
        let mut source_rust_content =
            fs::read_to_string(&self.rust_input_path).with_context(|| {
                let err_msg = format!(
                    "Failed to read rust input file \"{}\"",
                    self.rust_input_path
                );
                log::error!("{}", err_msg);
                err_msg
            })?;

        // TODO: refine `file_ast` to get methods of structs defined in customized module
        let extra_method_source = "".to_owned();
        // let paths = get_module_paths(&source_rust_content);
        // log::debug!("extra module paths:{paths:?}"); //TODO: delete
        // let v = vec!["OnlyForApi1Struct"]; // TODO:
        // for path in paths {
        //     let content = fs::read_to_string(path).with_context(|| {
        //         let err_msg = format!("Failed to read rust path \"{path}\"");
        //         log::error!("{}", err_msg);
        //         err_msg
        //     })?;
        //     extra_method_source = format!(
        //         "{extra_method_source}\n{}",
        //         self.find_impl_block(content, &v)
        //     );
        // }
        source_rust_content += &extra_method_source;
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
        assert!(regular_configs.len() > 1);
        assert!(regular_configs.iter().all(|c| !c.shared));

        // get shared funcs, struct_pool and enum_pool
        let funcs = HashSet::new();
        let mut structs = Vec::new();
        let mut enums = Vec::new();
        for config in regular_configs {
            let ir_file = config.get_ir_file(all_configs)?;

            // TODO:
            // funcs.extend(ir_file.get_shared_methods().clone());
            structs.extend(ir_file.struct_pool);
            enums.extend(ir_file.enum_pool);
        }
        let funcs = funcs.into_iter().collect::<Vec<_>>();
        let struct_pool = structs
            .find_duplicates(true)
            .into_iter()
            .map(|x| (x.0, x.1))
            .collect::<HashMap<_, _>>();
        let enum_pool = enums
            .find_duplicates(true)
            .into_iter()
            .map(|x| (x.0, x.1))
            .collect::<HashMap<_, _>>();

        log::debug!("the shared funcs:{:?}", funcs); //TODO: delete
        log::debug!("the shared struct_pool:{:?}", struct_pool); //TODO: delete
        log::debug!("the shared enum_pool:{:?}", enum_pool); //TODO: delete

        Ok(IrFile::new(
            funcs, // this field would effect `IrFile.visit_types(...)` and others
            struct_pool,
            enum_pool,
            true, // set true, in case for the methods of a shared struct,
            BlockIndex(shared_index),
            all_configs,
            true,
        ))
    }

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
        if self.bridge_in_method {
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
