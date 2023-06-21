use crate::ir::{IrFile, IrFunc};
use crate::utils::misc::{BlockIndex, ExtraTraitForVec};
use crate::{parser, transformer};
use anyhow::{Context, Result};
use convert_case::{Case, Casing};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use std::cell::RefCell;

// A thread-local static variable that holds a `HashMap` with `String` keys and `Vec<IrFunc>` values.
// The key represents a rust module path, and the value is a vector of `IrFunc`(methods) in the path.
// This global variable is used to avoid redundant overhead
// when searching `IrFunc` defined in extra rust modules in multi-blocks case.
thread_local!(static EXTRA_FUNC_MAP: RefCell<HashMap<String, Vec<IrFunc>>> = RefCell::new(HashMap::new()));
// A thread-local static variable that holds a `HashMap` with `BlockIndex` keys and `IrFile` values.
// The key represents a block index, and the value is an intermediate representation of the Rust code(`IrFile`) in the block.
// This global variable is used to avoid redundant overhead for generating the same `IrFile` in multi-blocks case.
thread_local!(static IR_FILE_MAP: RefCell<HashMap<BlockIndex, IrFile>> = RefCell::new(HashMap::new()));

/// Parsed configs, mainly used for internal logic
#[derive(Debug, Clone, serde::Serialize)]
pub struct Opts {
    pub rust_input_path: String,
    pub dart_output_path: String,
    pub dart_decl_output_path: Option<String>,
    pub c_output_paths: Vec<String>,
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
    pub dart3: bool,
    pub shared: bool, // it is true if this Opts instance is for auto-generated shared API block. Otherwise, it is false,
    // for the below 2 fields:
    // in single-block case, they should be `None`;
    // in multi-blocks case, they should be paths to generated files with full directories
    pub shared_rust_output_path: Option<String>,
    pub shared_dart_output_path: Option<String>,
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
        let ir_file = transformer::transform(raw_ir_file);

        Ok(ir_file)
    }

    fn get_regular_ir_file(&self, all_configs: &[Opts]) -> Result<IrFile> {
        let mut ir_file = IR_FILE_MAP.with(|data| {
            let mut ir_file_map = data.borrow_mut();
            let ir_file = if let std::collections::hash_map::Entry::Vacant(e) =
                ir_file_map.entry(self.block_index)
            {
                log::debug!("Phase: Parse source code to AST");
                let source_rust_content = try_read_from_file(
                    &self.rust_input_path,
                    &format!(
                        "Failed to read rust input file \"{}\"",
                        self.rust_input_path
                    ),
                )
                .unwrap();

                let file_ast = syn::parse_file(&source_rust_content).unwrap();
                log::debug!("Phase: Parse AST to IR");
                let ir_file = parser::parse(
                    &source_rust_content,
                    file_ast,
                    &self.manifest_path,
                    self.block_index,
                    self.shared,
                    all_configs,
                );

                e.insert(ir_file.clone());

                ir_file
            } else {
                ir_file_map[&self.block_index].clone()
            };

            ir_file
        });

        // if this is a single block case or `all_configs` is empty,
        // then there is no need to deal with funcs with
        // extra API blocks or extra rust module files.
        if all_configs.len() <= 1 {
            return Ok(ir_file);
        }

        log::debug!("before fetch_shared_types_if_needed"); //TODO: delete
        ir_file.fetch_shared_types_if_needed(all_configs);
        log::debug!("after fetch_shared_types_if_needed"); //TODO: delete

        let original_func_len = ir_file.funcs.len();
        EXTRA_FUNC_MAP.with(|data| {
            let mut extra_func_map = data.borrow_mut();

            log::debug!("my struct pool are:{:?}", ir_file.struct_pool); //TODO: delete
            log::debug!("my enum pool are:{:?}", ir_file.enum_pool); //TODO: delete

            // define type_pool_map from `struct_pool` and `enum_pool` in an ordered map
            let mut type_pool_map = std::collections::BTreeMap::new();
            for value in ir_file.struct_pool.values() {
                let struct_paths = value.path.clone().unwrap();
                let raw_code_path =
                    format!("{}.rs", struct_paths[..struct_paths.len() - 1].join("/"));
                let type_name = struct_paths.last().unwrap();
                type_pool_map.insert(type_name.to_string(), raw_code_path.clone());
            }
            for value in ir_file.enum_pool.values() {
                let struct_paths = value.path.clone();
                let raw_code_path =
                    format!("{}.rs", struct_paths[..struct_paths.len() - 1].join("/"));
                let type_name: &String = struct_paths.last().unwrap();
                type_pool_map.insert(type_name.to_string(), raw_code_path.clone());
            }

            // add extra methods
            for (type_name, raw_code_path) in type_pool_map {
                log::debug!("the code_path:{raw_code_path}"); //TODO: delete
                log::debug!("the type_name:{type_name}"); //TODO: delete

                let extra_path_funcs = if let std::collections::hash_map::Entry::Vacant(e) =
                    extra_func_map.entry(raw_code_path.clone())
                {
                    assert!(raw_code_path.contains("crate/"));
                    let correct_prefix = self.manifest_path.replace("Cargo.toml", "src/");
                    let code_path = raw_code_path.replace("crate/", &correct_prefix);
                    if let Some(code_path) = check_rust_path(&code_path) {
                        log::debug!("the refined code_path:{code_path}"); //TODO: delete140 +
                        let extra_source_rust_content = try_read_from_file(
                            &code_path,
                            &format!("Failed to read extra rust module file \"{}\"", code_path),
                        )
                        .unwrap();

                        //↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓extra parse↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓
                        let extra_file_ast = syn::parse_file(&extra_source_rust_content).unwrap();
                        log::debug!("Phase: Parse EXTRA AST to IR");
                        let extra_ir_file = parser::parse(
                            &extra_source_rust_content,
                            extra_file_ast,
                            &self.manifest_path,
                            self.block_index,
                            self.shared,
                            &[],
                        );
                        log::debug!("Finished Phase: Parse EXTRA AST to IR");
                        //↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑extra parse↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑

                        // collect extra_ir_file.funcs into vec
                        let extra_path_funcs = extra_ir_file.funcs.into_iter().collect::<Vec<_>>();
                        e.insert(extra_path_funcs.clone());

                        extra_path_funcs
                    } else {
                        continue;
                    }
                } else {
                    extra_func_map[&raw_code_path].clone()
                };

                // add types(struct/enum) methods defined in extra rust modules
                for each_func in extra_path_funcs {
                    if each_func.name.ends_with(&(format!("__{type_name}"))) {
                        log::debug!("func added: {each_func:?}"); //TODO: delete
                        let mut method = each_func.clone();
                        if ir_file.get_shared_type_names().contains(&type_name) {
                            log::debug!("`{}` is found to be a shared type", type_name); //TODO: delete
                            method.shared = true;
                        }

                        if !ir_file.funcs.contains(&method) {
                            ir_file.funcs.push(method);
                        }
                    }
                }

                // update the ir_file back to the global irfile map
                IR_FILE_MAP.with(|data| {
                    let mut ir_file_map = data.borrow_mut();
                    ir_file_map.insert(self.block_index, ir_file.clone());
                });

                // if new methods are added, then it is essential to refine (no-)shared types.
                if original_func_len != ir_file.funcs.len() {
                    ir_file.fetch_shared_types_forcely(all_configs);
                }
            }
        });

        Ok(ir_file)
    }

    fn get_shared_ir_file(&self, all_configs: &[Opts]) -> Result<IrFile> {
        log::debug!("get_shared_ir_file 1"); //TODO: delete
        assert!(
            all_configs.len() > 1,
            "`get_shared_ir_file(..)` should not be called when all_configs.len()<=1"
        );
        let (regular_configs, shared_index) = if !all_configs.last().unwrap().shared {
            (all_configs, all_configs.len())
        } else {
            let last_index = all_configs.len() - 1;
            (&all_configs[0..last_index], last_index)
        };
        assert!(regular_configs.len() > 1);
        assert!(regular_configs.iter().all(|c| !c.shared));

        // get shared funcs, struct_pool and enum_pool
        let mut funcs = Vec::new();
        let mut structs = Vec::new();
        let mut enums = Vec::new();
        for regular_config in regular_configs {
            let regular_ir_file = regular_config.get_ir_file(all_configs)?;
            funcs.extend(regular_ir_file.get_shared_funcs().clone());
            structs.extend(regular_ir_file.struct_pool);
            enums.extend(regular_ir_file.enum_pool);
        }
        let funcs = funcs.find_duplicates(true).into_iter().collect::<Vec<_>>();
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
            false, // set false like that in regular API block, in case for the methods of a shared struct,
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

fn check_rust_path(path_str: &str) -> Option<String> {
    let path = Path::new(path_str);
    if !path.exists() {
        return None;
    }
    if path.is_file() {
        if let Some(ext) = path.extension() {
            if ext == "rs" {
                return Some(path.to_string_lossy().to_string());
            }
        }
    } else if path.is_dir() {
        let mod_rs_path = path.join("mod.rs");
        if mod_rs_path.exists() {
            return Some(mod_rs_path.to_string_lossy().to_string());
        }
    }
    None
}

fn try_read_from_file(file_path: &str, error_msg: &str) -> Result<String, anyhow::Error> {
    let file_content = fs::read_to_string(file_path).with_context(|| {
        log::error!("{}", error_msg);
        error_msg.to_owned()
    })?;
    Ok(file_content)
}

pub struct PathForGeneration {
    pub base_path: PathBuf,
    pub io_path: PathBuf,
    pub wasm_path: PathBuf,
}
