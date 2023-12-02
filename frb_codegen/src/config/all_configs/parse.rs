use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::ir::{IrFile, IrType, IrTypeTrait};

use crate::parser;
use crate::target::Acc;
use crate::utils::misc::{BlockIndex, ExtraTraitForVec, IrTypeUseRange};

use super::{AllConfigs, IrFuncContext, IrTypeContext};

impl AllConfigs {
    pub(in crate::config::all_configs) fn parse(&mut self) {
        // 1.1 get raw regular ir_file(s)
        self.regular_ir_files = self
            .regular_configs
            .iter()
            .map(|regular_config| {
                parser::parse_a_rust_file(
                    &regular_config.manifest_path,
                    &regular_config.rust_input_path,
                    regular_config.block_index,
                    None,
                )
                .unwrap()
            })
            .collect::<Vec<_>>();
        // 1.2 check conflict for the function(not method) Apis among all regular blocks
        let duplicates = self.get_duplicated_funcs();
        if !duplicates.is_empty() {
            let duplicated_apis = duplicates.join(",");
            let (symbol_str, verb_str) = if duplicates.len() == 1 {
                ("API", "has")
            } else {
                ("API", "have")
            };
            panic!(
                "{} [{}] {} already been defined",
                symbol_str, duplicated_apis, verb_str
            );
        }

        // 2.1 parse all Apis(including methods of types) and types used in each regular ir_file
        for regular_ir_file in &mut self.regular_ir_files {
            regular_ir_file.parse_types_and_methods_in_extra_files_for_regular_block(
                &self
                    .regular_configs
                    .iter()
                    .map(|c| &c.rust_input_path)
                    .collect::<Vec<_>>(),
            );
        }
        // 2.2 assign to shared ir_file for multi-blocks case only
        if self.is_multi_blocks_case() {
            let (mut regular_funcs, mut regular_structs_pool, mut regular_enums_pool) =
                (vec![], vec![], vec![]);
            let mut regular_types_maps = vec![];
            for regular_ir_file in &self.regular_ir_files {
                regular_funcs.extend(regular_ir_file.funcs.clone());
                regular_types_maps.push(regular_ir_file.types_use_range_map.clone());
                regular_structs_pool.extend(regular_ir_file.struct_pool.clone().into_iter());
                regular_enums_pool.extend(regular_ir_file.enum_pool.clone().into_iter());
            }
            let shared_funcs = regular_funcs
                .find_duplicates_in_order(true)
                .into_iter()
                .collect::<Vec<_>>();
            // try to get shared_types_map by merging all regular_types_maps by keys,
            // if a key is duplicated, then the value of the key is the union of all values of the key
            let mut shared_types_map = HashMap::<IrType, HashSet<IrTypeUseRange>>::new();
            for regular_types_map in regular_types_maps {
                for (key, value) in regular_types_map {
                    shared_types_map
                        .entry(key)
                        .and_modify(|v| {
                            v.extend(value.clone());
                        })
                        .or_insert(value);
                }
            }

            let shared_structs_pool = regular_structs_pool
                .find_duplicates_in_order(true)
                .into_iter()
                .map(|x| (x.0, x.1))
                .collect::<HashMap<_, _>>();
            let shared_enums_pool = regular_enums_pool
                .find_duplicates_in_order(true)
                .into_iter()
                .map(|x| (x.0, x.1))
                .collect::<HashMap<_, _>>();
            self.shared_ir_file = Some(IrFile::new(
                "",
                "",
                shared_funcs,
                Some(shared_types_map),
                shared_structs_pool,
                shared_enums_pool,
                false, // set false like that in regular API block, in case for the methods of a shared struct,
                BlockIndex(None), // for shared block, the block index is `None`
            ));
        }

        // 3. parse `self.funcs_contexts`
        for regular_ir_file in &self.regular_ir_files {
            for func in &regular_ir_file.funcs {
                if let Some(func_context) = self
                    .funcs_contexts
                    .iter_mut()
                    .find(|each| each.get_func() == func)
                {
                    func_context.insert_block_index(regular_ir_file.block_index);
                } else {
                    self.funcs_contexts
                        .push(IrFuncContext::new(func, regular_ir_file));
                }
            }
        }

        // 4. parse `self.types_contexts`
        for regular_ir_file in &self.regular_ir_files {
            for ir_type in &regular_ir_file.types {
                if let Some(ty_context) = self
                    .types_contexts
                    .iter_mut()
                    .find(|each| each.get_type() == ir_type)
                {
                    ty_context.insert_block_index(regular_ir_file.block_index, false);
                    ty_context.extend_use_range(regular_ir_file.get_type_use_range(ir_type), false);
                } else {
                    // initial a new ir_type context
                    let mut new_ir_type_context = IrTypeContext::new(ir_type, regular_ir_file);
                    // refine `safe_ident_use_range` and `safe_ident_used_block_indices`
                    // with the help from all regular ir_files(including the current one)
                    // Note: to match the safe_ident version, it compares the CORE type's safe_ident signature.
                    // For example, `Option<T>` or `Boxed<T>` has different safe_ident signature with `T`.
                    // But they share same core type `T`, therefore share the same safe_ident context.
                    let target_type_safe_ident =
                        new_ir_type_context.get_type().get_core_type().safe_ident();
                    for regular_ir_file in &self.regular_ir_files {
                        let same_safe_ident_types = regular_ir_file
                            .types
                            .iter()
                            .filter(|each| {
                                each.get_core_type().safe_ident() == target_type_safe_ident
                            })
                            .collect::<Vec<_>>();
                        for each_same_type in same_safe_ident_types {
                            new_ir_type_context
                                .insert_block_index(regular_ir_file.block_index, true);
                            new_ir_type_context.extend_use_range(
                                regular_ir_file.get_type_use_range(each_same_type),
                                true,
                            );
                        }
                    }
                    // finally add the new one to `self.types_contexts`
                    self.types_contexts.push(new_ir_type_context);
                }
            }
        }

        // 5. check
        // 5.1 check func context error
        let err_func_items = self
            .funcs_contexts
            .iter()
            .filter_map(|each| {
                let mut errs = Vec::new();
                if let Err(err) =
                    filter_func_context_error_for_used_block_indices(each.get_block_indices())
                {
                    errs.push(err);
                }
                if !errs.is_empty() {
                    Some((each, errs.join(", ")))
                } else {
                    None
                }
            })
            .map(|(each, errs)| format!("Error: {} - Func: {:?}", errs, each))
            .collect::<Vec<_>>()
            .into_iter()
            .join("\n");
        if !err_func_items.is_empty() {
            panic!(
                "the `funcs_contexts` is errored with the items:\n{}",
                err_func_items
            );
        }
        // 5.2 check type context error
        let err_type_items = self
            .types_contexts
            .iter()
            .filter_map(|each_type_context| {
                // non-safe-ident version check
                let mut errs = Vec::new();
                if let Err(err) = filter_type_context_error_for_used_block_indices(
                    each_type_context.get_block_indices(false),
                    false,
                ) {
                    errs.push(err);
                }
                if let Err(err) = filter_type_context_error_for_use_range(each_type_context, false)
                {
                    errs.push(err);
                }
                // safe-ident version check
                if let Err(err) = filter_type_context_error_for_used_block_indices(
                    each_type_context.get_block_indices(true),
                    true,
                ) {
                    errs.push(err);
                }
                if let Err(err) = filter_type_context_error_for_use_range(each_type_context, true) {
                    errs.push(err);
                }

                if !errs.is_empty() {
                    Some((each_type_context, errs.join(", ")))
                } else {
                    None
                }
            })
            .map(|(each, errs)| format!("Error: {} - Type: {:?}", errs, each))
            .collect::<Vec<_>>()
            .into_iter()
            .join("\n");
        if !err_type_items.is_empty() {
            panic!(
                "the `types_contexts` is errored with the items:\n{}",
                err_type_items
            );
        }

        // 6. parse shared dart api2wire funcs, useful in generating dart code
        self.parse_dart_api2wire_funcs();

        log::debug!("the func context is:{:?}", self.funcs_contexts); // TODO: delete
        log::debug!("the type context is:{:?}", self.types_contexts); // TODO: delete
    }

    fn get_duplicated_funcs(&mut self) -> Vec<String> {
        let manual_defined_apis = self
            .regular_ir_files
            .iter()
            .flat_map(|regular_ir_file| regular_ir_file.funcs.clone())
            .map(|f| f.name)
            .collect::<Vec<_>>();

        manual_defined_apis.find_duplicates_in_order(true)
    }

    fn parse_dart_api2wire_funcs(&mut self) {
        // 1. parse shared dart api2wire funcs, useful in generating dart code
        if let Some(shared_config) = self.get_shared_config() {
            let shared_api2wire_funcs = self
                .get_types(shared_config.block_index, true, true, true, false)
                .iter()
                .map(|ty| crate::generator::dart::generate_api2wire_func(ty, shared_config, self))
                .collect::<Acc<_>>()
                .join("\n");
            assert!(self
                .dart_api2wire_funcs_map
                .insert(shared_config.block_index, shared_api2wire_funcs)
                .is_none());
        }

        // 2. parse regular dart api2wire funcs, useful in generating dart code
        for regular_config in &self.regular_configs {
            let regular_api2wire_funcs = self
                .get_types(regular_config.block_index, true, true, true, false)
                .iter()
                .map(|ty| crate::generator::dart::generate_api2wire_func(ty, regular_config, self))
                .collect::<Acc<_>>()
                .join("\n");
            assert!(self
                .dart_api2wire_funcs_map
                .insert(regular_config.block_index, regular_api2wire_funcs)
                .is_none());
        }
    }
}

fn filter_func_context_error_for_used_block_indices(
    used_block_indices: &HashSet<BlockIndex>,
) -> Result<Vec<BlockIndex>, String> {
    if used_block_indices.is_empty() {
        Err("`used_block_indices` is empty.".to_string())
    } else if used_block_indices.len() == 1 && used_block_indices.iter().next().unwrap().is_shared()
    {
        Err("`used_block_indices` has only one item and it is shared.".to_string())
    } else if used_block_indices.len() >= 2 {
        let count_shared = used_block_indices.iter().filter(|&x| x.is_shared()).count();
        let count_regular = used_block_indices
            .iter()
            .filter(|&x| !x.is_shared())
            .count();
        if count_shared == 0 || count_regular < 2 {
            Err(
                "`used_block_indices` has no shared block or less than two regular blocks."
                    .to_string(),
            )
        } else {
            Ok(used_block_indices.iter().cloned().collect())
        }
    } else {
        Ok(used_block_indices.iter().cloned().collect())
    }
}

fn filter_type_context_error_for_used_block_indices(
    used_block_indices: &HashSet<BlockIndex>,
    for_safe_ident: bool,
) -> Result<Vec<BlockIndex>, String> {
    let prefix = if for_safe_ident {
        "safe_ident_used_block_indices"
    } else {
        "used_block_indices"
    };

    if used_block_indices.is_empty() {
        Err(format!("`{}` is empty.", prefix))
    } else if used_block_indices.len() == 1 && used_block_indices.iter().next().unwrap().is_shared()
    {
        Err(format!("`{}` has only one item and it is shared.", prefix))
    } else if used_block_indices.len() >= 2 {
        let count_shared = used_block_indices.iter().filter(|&x| x.is_shared()).count();
        let count_regular = used_block_indices
            .iter()
            .filter(|&x| !x.is_shared())
            .count();
        if count_shared == 0 || count_regular < 2 {
            Err(format!(
                "`{}` has no shared block or less than two regular blocks.",
                prefix
            ))
        } else {
            Ok(used_block_indices.iter().cloned().collect())
        }
    } else {
        Ok(used_block_indices.iter().cloned().collect())
    }
}
fn filter_type_context_error_for_use_range(
    type_context: &IrTypeContext,
    for_safe_ident: bool,
) -> Result<Vec<IrTypeUseRange>, String> {
    let prefix = if for_safe_ident {
        "safe_ident_use_range"
    } else {
        "use_range"
    };
    let use_range = type_context.get_use_range(prefix == "safe_ident_use_range");

    if !for_safe_ident {
        if let IrType::Boxed(_) = type_context.get_type().clone() {
            if type_context
                .get_use_range(for_safe_ident)
                .contains(&IrTypeUseRange::Output)
            {
                return Err(format!("A `Boxed` type but has `output` in `{prefix}`."));
            }
        }

        if let IrType::SyncReturn(_) = type_context.get_type().clone() {
            if type_context
                .get_use_range(for_safe_ident)
                .contains(&IrTypeUseRange::Input)
            {
                return Err(format!(
                    "A `SyncReturn` type but has `input` in `{prefix}`."
                ));
            }
        }
    }

    if use_range.is_empty() {
        Err(format!("`{}` is empty", prefix))
    } else if use_range.len() > 2 {
        Err(format!("`{}` has more than two items", prefix))
    } else {
        Ok(use_range.iter().cloned().collect())
    }
}
