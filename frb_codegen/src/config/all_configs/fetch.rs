use std::collections::HashSet;

use convert_case::{Case, Casing};

use super::{AllConfigs, IrFuncContext, IrTypeContext};
use crate::ir::{IrFile, IrFunc, IrStruct, IrType, IrTypeTrait};
use crate::parser::ParserResult;
use crate::target::{Acc, Target};
use crate::utils::misc::{get_rust_module_name, BlockIndex, ExtraTraitForVec};
use crate::Opts;

impl AllConfigs {
    #[allow(unused)]
    /// get (shared) config by block_index
    pub(crate) fn get_config(&self, block_index: BlockIndex) -> &Opts {
        if block_index == None {
            assert!(self.is_multi_blocks_case());
            return self.get_shared_config().unwrap();
        }
        self.get_regular_configs()
            .iter()
            .find(|c| c.block_index == block_index)
            .unwrap()
    }

    pub(crate) fn get_regular_configs(&self) -> &[Opts] {
        &self.regular_configs
    }

    pub(crate) fn get_shared_config(&self) -> Option<&Opts> {
        self.shared_config.as_ref()
    }

    pub(crate) fn get_c_struct_names(&self, block_index: BlockIndex) -> Vec<String> {
        let c_struct_names = self
            .get_types_contexts(block_index, true, false, true, true)
            .iter()
            .filter_map(|each_type_context| {
                if let IrType::StructRef(_) = each_type_context.get_type() {
                    Some(each_type_context.get_type().rust_wire_type(Target::Io))
                } else {
                    None
                }
            })
            .collect();
        c_struct_names
    }

    /// Given an optional reference to an `Opts` struct, returns the Rust module name
    /// of the corresponding Rust code file.
    ///
    /// If the given reference is `Some`, the function uses the
    /// `rust_input_path` and `rust_crate_dir` fields of
    /// the `Opts` struct to construct the path to the Rust code file,
    /// and returns its module name.
    /// If the given reference is `None`, the function returns
    /// the Rust module name of the shared configuration, if it exists.
    ///
    /// # Arguments
    ///
    /// * `config` - An optional reference to an `Opts` struct.
    ///
    /// # Returns
    ///
    /// An optional string that holds the Rust module name of the corresponding Rust code file,
    /// or `None` if the module name cannot be determined(empty string).
    pub(crate) fn get_rust_module_name(&self, config: Option<&Opts>) -> Option<String> {
        // get essential paths
        let (crate_path, code_path) = match config {
            Some(config) => {
                if config.block_index.is_shared() {
                    assert!(self.is_multi_blocks_case());
                    let shared_config = self.get_shared_config().unwrap();
                    (
                        &shared_config.rust_crate_dir,
                        &shared_config.rust_output_path,
                    )
                } else {
                    (&config.rust_crate_dir, &config.rust_input_path)
                }
            }
            None => {
                assert!(self.is_multi_blocks_case());
                let shared_config = self.get_shared_config().unwrap();
                (
                    &shared_config.rust_crate_dir,
                    &shared_config.rust_output_path,
                )
            }
        };
        let module_name = get_rust_module_name(crate_path, code_path);
        // check before return
        if module_name.is_empty() {
            None
        } else {
            Some(module_name)
        }
    }

    pub(crate) fn get_ir_file(&self, block_index: BlockIndex) -> ParserResult<&IrFile> {
        assert!(!self.regular_configs.is_empty());
        assert!(self.regular_configs.len() == self.regular_ir_files.len());
        let shared = block_index.is_shared();
        if shared {
            assert!(self.is_multi_blocks_case());
            Ok(self.shared_ir_file.as_ref().unwrap())
        } else {
            let index = self
                .regular_configs
                .iter()
                .position(|each_config| each_config.block_index == block_index)
                .unwrap();
            Ok(self.regular_ir_files.get(index).unwrap())
        }
    }

    /// Retrieves the functions(Apis) based on the specified block index and exclusion flag.
    ///
    /// # Arguments
    ///
    /// * `block_index` - The block index to filter the functions.
    /// * `exclude_shared` - A flag indicating whether to exclude shared functions. Note: If it is `false`, it means to include both shared and non-shared functions. And it has no effect if `block_index` points to a shared block.
    ///
    /// # Returns
    ///
    /// A vector of `IrFunc` representing the retrieved functions(Apis).
    pub(crate) fn get_funcs(&self, block_index: BlockIndex, exclude_shared: bool) -> Vec<IrFunc> {
        let mut funcs = self
            .get_funcs_contexts(block_index, exclude_shared)
            .iter()
            .map(|each_func| each_func.get_func().clone())
            .collect::<Vec<_>>();

        funcs.sort_by_key(|f| f.name.clone());
        funcs
    }
    /// Returns a slice of IR function contexts matching specific conditions.
    /// Note: For `exclude_shared`, if it is `false`, it means to include both shared and non-shared ones.
    /// And `exclude_shared` has no effect if `block_index` points to a shared block.
    fn get_funcs_contexts(
        &self,
        block_index: BlockIndex,
        exclude_shared: bool,
    ) -> Vec<IrFuncContext> {
        let exclude_shared = if block_index.is_shared() {
            assert!(
                self.is_multi_blocks_case(),
                "for a shared block, the `block_index` should be None"
            );
            false
        } else {
            exclude_shared
        };

        self.funcs_contexts
            .iter()
            .filter(|each_func| {
                let is_used_in_the_block = each_func.get_block_indices().contains(&block_index);
                let shared_condition = !exclude_shared || !each_func.is_shared();
                is_used_in_the_block && shared_condition
            })
            .cloned()
            .collect::<Vec<_>>()
    }

    /// Retrieves all types for a specified block.
    ///
    /// # Arguments
    ///
    /// * `block_index` - The index of the block, in witch type is used.
    /// * `exclude_shared` - Whether to exclude shared types(if it is `false`, it means to include both shared and non-shared ones).
    /// * `diff_by_safe_ident` - Whether to differentiate types by their safe identifiers.
    /// * `input` - Whether to include input types.
    /// * `output` - Whether to include output types.
    ///
    /// Note: when both `input` and `output` are `true`, it means to include types either met condition input or output or both.
    /// And `exclude_shared` has no effect if `block_index` points to a shared block.
    pub(crate) fn get_types(
        &self,
        block_index: BlockIndex,
        exclude_shared: bool,
        diff_by_safe_ident: bool,
        input: bool,
        output: bool,
    ) -> Vec<IrType> {
        let mut types = self
            .get_types_contexts(
                block_index,
                exclude_shared,
                diff_by_safe_ident,
                input,
                output,
            )
            .iter()
            .map(|e| e.get_type().clone())
            .collect::<Vec<_>>();

        if input && !output {
            types = refined_input_types(&types);
        }
        types.sort_by_key(|t| t.safe_ident());
        types
    }

    /// Returns a slice of IR type contexts matching specific conditions.
    ///
    /// # Arguments
    ///
    /// * `block_index` - The config(Opts) block index to match.
    /// * `exclude_shared` - A boolean flag indicating whether to exclude shared types(if it is `false`, it means to include both shared and non-shared ones).
    /// * `diff_by_safe_ident` - A boolean flag indicating whether to differentiate types by their safe identifiers.
    /// * `input` - A boolean flag indicating whether to include types AT LEAST act as input param.
    /// * `output` - A boolean flag indicating whether to include types AT LEAST act as output return value.
    ///
    /// Note: when both `input` and `output` are `true`, it means to filter types either met condition input or output or both.
    /// And `exclude_shared` has no effect if `block_index` points to a shared block.
    pub(in crate::config::all_configs) fn get_types_contexts(
        &self,
        block_index: BlockIndex,
        exclude_shared: bool,
        diff_by_safe_ident: bool,
        input: bool,
        output: bool,
    ) -> Vec<IrTypeContext> {
        let exclude_shared = if block_index.is_shared() {
            assert!(
                self.is_multi_blocks_case(),
                "for a shared block, the `block_index` should be None"
            );
            false
        } else {
            exclude_shared
        };
        assert!(input || output);

        self.types_contexts
            .iter()
            .filter(|each_type_context| {
                let is_used_in_the_block = each_type_context
                    .get_block_indices(diff_by_safe_ident)
                    .contains(&block_index)
                    && (!exclude_shared
                        || each_type_context
                            .get_block_indices(diff_by_safe_ident)
                            .len()
                            == 1);

                let is_input_type =
                    input && each_type_context.is_input(exclude_shared, diff_by_safe_ident);
                let is_output_type =
                    output && each_type_context.is_output(exclude_shared, diff_by_safe_ident);

                is_used_in_the_block && (is_input_type || is_output_type)
            })
            .cloned()
            .collect::<Vec<_>>()
    }
    /// Retrieves all structure types for a specified block.
    ///
    /// # Arguments
    ///
    /// * `block_index` - The index of the block for which to retrieve the types.
    /// * `exclude_shared_type` - A boolean flag indicating whether to exclude shared types.
    /// * `diff_by_safe_ident` - A boolean flag indicating whether to differentiate types by safe identifier.
    /// * `input` - A boolean flag indicating whether to include types that AT LEAST act as input parameters.
    /// * `output` - A boolean flag indicating whether to include types that AT LEAST act as output return values.
    ///
    /// Note: when both `input` and `output` are `true`, it means to include types that meet either the input condition, the output condition, or both.
    pub(crate) fn get_ir_structs(
        &self,
        block_index: BlockIndex,
        exclude_shared_type: bool,
        diff_by_safe_ident: bool,
        input: bool,
        output: bool,
    ) -> Vec<IrStruct> {
        let ir_file = self.get_ir_file(block_index).unwrap();
        self.get_types(
            block_index,
            exclude_shared_type,
            diff_by_safe_ident,
            input,
            output,
        )
        .iter()
        .filter_map(|ty| {
            // TODO: compact
            if ty.is_struct_ref() {
                if let IrType::StructRef(s) = ty {
                    Some(s.get(ir_file).clone())
                } else {
                    panic!("should be struct ref")
                }
            } else {
                None
            }
        })
        .collect()
    }

    // TODO: delete
    /// Retrieves the Rust names of all types for a specified block.
    ///
    /// # Arguments
    ///
    /// * `block_index` - The index of the block for which to retrieve the types.
    /// * `exclude_shared_type` - A boolean flag indicating whether to exclude shared types.
    /// * `diff_by_safe_ident` - A boolean flag indicating whether to differentiate types by safe identifier.
    /// * `input` - A boolean flag indicating whether to include types that AT LEAST act as input parameters.
    /// * `output` - A boolean flag indicating whether to include types that AT LEAST act as output return values.
    ///
    /// Note: when both `input` and `output` are `true`, it means to include types that meet either the input condition, the output condition, or both.
    pub(crate) fn get_types_rust_names(
        &self,
        block_index: BlockIndex,
        exclude_shared_type: bool,
        diff_by_safe_ident: bool,
        input: bool,
        output: bool,
    ) -> Vec<String> {
        let types = self.get_types(
            block_index,
            exclude_shared_type,
            diff_by_safe_ident,
            input,
            output,
        );
        let mut result = types
            .iter()
            .map(|ty| {
                let type_raw_name = ty.safe_ident();
                if !ty.is_list(true) {
                    type_raw_name.to_case(Case::Pascal)
                } else {
                    type_raw_name
                }
            })
            .collect::<Vec<_>>();
        // deduplicated
        result.sort_by_key(|ty| ty.to_owned());
        result.dedup();

        result
    }

    /// Checks if the given IR type is sharely used with the specific conditions.
    ///
    /// # Arguments
    ///
    /// * `ir_type` - A reference to the IR type to check.
    /// * `must_used_in_the_regular_config` - If it is not None, the function will further check if the given IR type is used in the regular config with the given option.
    /// * `diff_by_safe_ident` - A boolean flag indicating whether to treat types are "sharely used" or not by their safe identifiers.
    ///
    /// # Panics
    ///
    /// If `is_used_in_regular_config` is not `None` and its `shared` field is `true`, the function will panic.
    ///
    /// # Returns
    ///
    /// A boolean value indicating whether the given IR type is sharely used with the specific conditions.
    pub(crate) fn is_type_shared(&self, ir_type: &IrType, diff_by_safe_ident: bool) -> bool {
        if !self.is_multi_blocks_case() {
            return false;
        }
        let binding = self.get_types_contexts(
            self.shared_config.clone().unwrap().block_index,
            false,
            diff_by_safe_ident,
            true,
            true,
        );
        let shared_types = binding
            .iter()
            .map(|each_type_context| each_type_context.get_type())
            .collect::<HashSet<_>>();

        shared_types.contains(&ir_type)
    }

    /// Get all symbols(Api/function/method names) defined explicitly or implictily
    /// within regular blocks
    // TODO: what is `extern_func_names`, vs `ir_file.get_funcs`? Does this need?
    pub(crate) fn get_all_symbols(&self) -> Vec<String> {
        let v = self
            .regular_configs
            .iter()
            .flat_map(|config| self.generate_rust(config).extern_func_names)
            .collect::<Vec<_>>();
        v.find_uniques_in_order(false)
    }

    pub(crate) fn get_shared_methods_wire_names(&self) -> Vec<String> {
        self.get_funcs(self.shared_config.as_ref().unwrap().block_index, true)
            .iter()
            .map(|method| format!("wire_{}", method.name))
            .collect::<Vec<_>>()
    }

    pub(crate) fn get_dart_api2wire_funcs(&self, block_index: BlockIndex) -> Option<&Acc<String>> {
        self.dart_api2wire_funcs_map.get(&block_index)
    }
}

fn refined_input_types(distinct_input_types: &[IrType]) -> Vec<IrType> {
    let mut refined_types = Vec::new();
    let mut boxed_inner_types = Vec::new();
    for t in distinct_input_types {
        match t {
            IrType::Boxed(ref inner) => {
                let value = inner.clone();
                boxed_inner_types.push(value);
                refined_types.push(t);
            }
            IrType::SyncReturn(_) => {}
            _ => refined_types.push(t),
        }
    }

    // refine the refined_types
    refined_types.retain(|t| match t {
        IrType::Boxed(_) => true,
        _ => !boxed_inner_types
            .iter()
            .any(|boxed_inner| t.safe_ident() == boxed_inner.safe_ident()),
    });

    refined_types.into_iter().cloned().collect::<Vec<_>>()
}
