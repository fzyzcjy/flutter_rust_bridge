use convert_case::{Case, Casing};

use crate::target::Target;
use crate::utils::misc::{is_multi_blocks_case, mod_from_rust_path, BlockIndex, ShareMode};
use crate::{generator, ir::*, Opts};
use std::collections::{HashMap, HashSet};
use std::thread;

use crate::utils::misc::ExtraTraitForVec;

pub type IrStructPool = HashMap<String, IrStruct>;
pub type IrEnumPool = HashMap<String, IrEnum>;

use std::cell::RefCell;

// this variable should be `none` for single block case, and not none for multi-blocks case.
thread_local!(pub static SHARED_MODULE: RefCell<Option<String>> = RefCell::new(None));

// shared types among all regular API blocks, in context of function paramters
thread_local!(pub static SHARED_TYPES_INPUT: RefCell<HashSet<IrType>> = RefCell::new(HashSet::new()));
thread_local!(static FETCHED_FOR_SHARED_TYPES_INPUT: RefCell<bool> = RefCell::new(false));

// shared types among all regular API blocks in context of function outputs
thread_local!(static SHARED_TYPES_OUTPUT: RefCell<HashSet<IrType>> = RefCell::new(HashSet::new()));
thread_local!(static FETCHED_FOR_SHARED_TYPES_OUTPUT: RefCell<bool> = RefCell::new(false));

// shared types among all regular API blocks
thread_local!(static SHARED_TYPES_ALL: RefCell<HashSet<IrType>> = RefCell::new(HashSet::new()));
thread_local!(static FETCHED_FOR_SHARED_TYPES_ALL: RefCell<bool> = RefCell::new(false));

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct IrFile {
    /// for a regular API block: this field contains APIs defined in this block and
    /// all methods of types(struct/enum) used ONLY in this block
    /// ---NOTE: for shared types used in this block, their methods are also included here;
    /// for a shared block: it ONLY contains methods from ALL shared types.
    funcs: Vec<IrFunc>,
    pub struct_pool: IrStructPool,
    pub enum_pool: IrEnumPool,
    pub has_executor: bool,
    pub block_index: BlockIndex,
    pub shared: ShareMode,
}

impl IrFile {
    pub fn funcs(&self, exclude_shared_method: bool) -> Vec<IrFunc> {
        if self.shared == ShareMode::Unique && exclude_shared_method {
            self.funcs
                .iter()
                .filter(|each| each.shared == ShareMode::Unique)
                .cloned()
                .collect()
        } else {
            self.funcs.clone()
        }
    }

    pub fn set_funcs(&mut self, dst_funcs: Vec<IrFunc>) {
        self.funcs = dst_funcs;
    }

    pub fn append_funcs(&mut self, dst_funcs: Vec<IrFunc>) {
        self.funcs.extend(dst_funcs);
    }

    /// Returns a vector of all shared methods in the file.
    /// For a regular API block, this method returns all methods of shared types used in this block.
    /// For a shared block, it returns all methods from all shared types.
    pub fn get_shared_methods(&self) -> Vec<IrFunc> {
        if self.shared == ShareMode::Unique {
            self.funcs
                .iter()
                .cloned()
                .filter(|each| each.shared == ShareMode::Shared)
                .collect::<Vec<_>>()
        } else {
            if self.shared == ShareMode::Shared {
                assert!(self.funcs.iter().all(|x| x.shared == ShareMode::Shared))
            }
            self.funcs.clone()
        }
    }

    pub fn new(
        funcs: Vec<IrFunc>,
        struct_pool: IrStructPool,
        enum_pool: IrEnumPool,
        has_executor: bool,
        block_index: BlockIndex,
        all_configs: &[Opts],
        shared: ShareMode,
    ) -> IrFile {
        let ir_fie = IrFile {
            funcs,
            struct_pool,
            enum_pool,
            has_executor,
            block_index,
            shared,
        };

        // Get shared types in advance here, because these types will be used multiple times later for multi-blocks case.
        ir_fie.fetch_shared_types_if_needed(all_configs);

        ir_fie
    }

    /// [f] returns [true] if it wants to stop going to the *children* of this subtree
    pub fn visit_types<F: FnMut(&IrType) -> bool>(
        &self,
        f: &mut F,
        include_func_inputs: bool,
        include_func_output: bool,
    ) {
        for func in &self.funcs(false) {
            if include_func_inputs {
                for field in &func.inputs {
                    field.ty.visit_types(f, self);
                }
            }
            if include_func_output {
                func.output.visit_types(f, self);
            }
        }
    }

    pub fn get_c_struct_names(&self, all_configs: &[Opts]) -> Vec<String> {
        let c_struct_names = self
            .distinct_types(true, true, all_configs)
            .iter()
            .filter_map(|ty| {
                if let IrType::StructRef(_) = ty {
                    Some(ty.rust_wire_type(Target::Io))
                } else {
                    None
                }
            })
            .collect();
        c_struct_names
    }

    /// get distinct types for this IrFile instance
    /// for either a regular or an auto-generated shared API block.
    /// NOTE: in multi-blocks case, the output would NOT include shared types for a regular block IrFile;
    /// and it would ONLY contain shared types for a shared block IrFile.
    pub fn distinct_types(
        &self,
        include_func_inputs: bool,
        include_func_output: bool,
        all_configs: &[Opts],
    ) -> Vec<IrType> {
        let shared_types = self.get_shared_distinct_types_for_all_blocks(
            include_func_inputs,
            include_func_output,
            all_configs,
        );

        let types = match self.shared {
            ShareMode::Unique => {
                let raw_distinct_types =
                    self.get_regular_distinct_types(include_func_inputs, include_func_output);

                // NOTE: for `raw_distinct_types`, don't distinguished by hash, but by safe_ident(),
                // since some types have different hashes, but the same safe_ident().
                raw_distinct_types
                    .into_iter()
                    .filter(|item| {
                        !shared_types.iter().any(|other_item| {
                            let cond1 = format!("box_autoadd_{}", other_item.safe_ident())
                                == item.safe_ident();
                            let cond2 = format!("box_autoadd_{}", item.safe_ident())
                                == other_item.safe_ident();
                            let cond3 = item.safe_ident() == other_item.safe_ident();

                            match (include_func_inputs, include_func_output) {
                                (true, true) => cond1 || cond2 || cond3,
                                (true, false) => cond1 || cond3,
                                (false, true) => cond2 || cond3,
                                (false, false) => unreachable!(),
                            }
                        })
                    })
                    .collect::<HashSet<_>>()
            }
            ShareMode::Shared => shared_types,
        };

        let mut types = types.into_iter().collect::<Vec<_>>();
        types.sort_by_key(|ty| ty.safe_ident());
        types
    }

    /// Get dinstinct types only for a regular API block(the current IrFile instance).
    /// NOTE: in multi-blocks case, the returned items would include shared types
    /// usded in the specific regular block.
    fn get_regular_distinct_types(
        &self,
        include_func_inputs: bool,
        include_func_output: bool,
    ) -> HashSet<IrType> {
        assert!(include_func_inputs || include_func_output);
        assert!(self.shared == ShareMode::Unique);
        let mut seen_idents = HashSet::new();
        let mut ans = Vec::new();
        self.visit_types(
            &mut |ty| {
                let ident = ty.safe_ident();
                let contains = seen_idents.contains(&ident);
                if !contains {
                    seen_idents.insert(ident);
                    ans.push(ty.clone());
                }
                contains
            },
            include_func_inputs,
            include_func_output,
        );

        ans.into_iter().collect::<HashSet<_>>()
    }

    /// Get shared types in context of both function inputs and outputs.
    /// for a regular IrFile, only shared types used in this block would be returned.
    /// for a shared IrFile, all shared types would be returned.
    fn get_shared_distinct_types_for_current_block(&self) -> HashSet<IrType> {
        let global_shared = self.get_shared_distinct_types_for_all_blocks(true, true, &[]);
        match self.shared {
            ShareMode::Unique => self
                .get_regular_distinct_types(true, true)
                .intersection(&global_shared)
                .map(|s| s.to_owned())
                .collect(),
            ShareMode::Shared => global_shared,
        }
    }

    /// Whatever this IrFile instance represents for, this method would get shared types through `all_configs`,
    /// which refers to a list may or may not include the shared config at the last index.
    /// For single-block case, it should return empty.
    /// For multi-block case, it should return ALL shared types among ALL regular
    /// blocks, which means that some of the shared types may not be used in all regular blocks.
    /// Also note `include_func_inputs` and `include_func_output` are essential for getting
    /// shared types for different context.
    fn get_shared_distinct_types_for_all_blocks(
        &self,
        include_func_inputs: bool,
        include_func_output: bool,
        all_configs: &[Opts],
    ) -> HashSet<IrType> {
        let fetch_func =
            |global_shared_types: &'static thread::LocalKey<RefCell<HashSet<IrType>>>,
             global_fetched: &'static thread::LocalKey<RefCell<bool>>| {
                global_fetched.with(|has_fetched| {
                    global_shared_types.with(|shared| {
                        let mut shares = shared.borrow_mut();

                        if all_configs.len() <= 1 {
                            // CASE 1: it is single block case;
                            // CASE 2: it is multi-blocks case, but `all_configs` is not available;
                            return shares.clone();
                        }

                        // For multi-blocks case, directly return it if it is fetched before.
                        let mut fetched: std::cell::RefMut<bool> = has_fetched.borrow_mut();
                        if *fetched {
                            return shares.clone();
                        }

                        // If not fetched before, do it from scracth.

                        *shares = self.get_shared_types_for_all_blocks_from_scratch(
                            include_func_inputs,
                            include_func_output,
                            all_configs,
                        );

                        *fetched = true;
                        shares.clone()
                    })
                })
            };

        match (include_func_inputs, include_func_output) {
            (true, true) => fetch_func(&SHARED_TYPES_ALL, &FETCHED_FOR_SHARED_TYPES_ALL),
            (true, false) => fetch_func(&SHARED_TYPES_INPUT, &FETCHED_FOR_SHARED_TYPES_INPUT),
            (false, true) => fetch_func(&SHARED_TYPES_OUTPUT, &FETCHED_FOR_SHARED_TYPES_OUTPUT),
            (false, false) => panic!("either input or output should be true"),
        }
    }
    /// Whatever the last config is shared or not,
    /// this method would get shared types through `all_configs`,
    fn get_shared_types_for_all_blocks_from_scratch(
        &self,
        include_func_inputs: bool,
        include_func_output: bool,
        all_configs: &[Opts],
    ) -> HashSet<IrType> {
        let regular_configs = match all_configs.last().unwrap().shared {
            ShareMode::Unique => all_configs,
            ShareMode::Shared => &all_configs[0..all_configs.len() - 1],
        };
        assert!(regular_configs
            .iter()
            .all(|c| c.shared == ShareMode::Unique));
        let mut regular_block_uniques = Vec::new();
        let mut all_regular_types = Vec::new();
        let mut regular_ir_files = Vec::new();
        for config in regular_configs.iter() {
            let ir_file = config.get_ir_file(&[]).unwrap(); // all_configs` is empty, no need to care about other configs here
            let distinct_types =
                ir_file.get_regular_distinct_types(include_func_inputs, include_func_output);
            regular_ir_files.push(ir_file.clone());
            all_regular_types.extend(distinct_types.clone());

            regular_block_uniques.push(distinct_types);
        }

        // pick out the raw shared types
        let mut shares = all_regular_types
            .find_duplicates(true)
            .into_iter()
            .collect::<HashSet<_>>();
        //↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓for cross shared types↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓
        if self.shared == ShareMode::Unique {
            for (i, each_block_set) in regular_block_uniques.iter().enumerate() {
                for suspected_shared_type in each_block_set {
                    self.add_cross_shared_types(
                        suspected_shared_type,
                        BlockIndex(i),
                        &mut shares,
                        &regular_ir_files,
                        include_func_inputs,
                        include_func_output,
                    );
                }
            }
        } else {
            unreachable!(
                "when dealing cross shared types for a shared IrFile,
                 there should have been some shared types got
                 when dealing with regular IrFile before."
            )
        }
        //↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑for cross shared types↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑

        //↓↓↓↓↓↓↓↓↓↓↓↓for types with same idents but different hashes↓↓↓↓↓↓↓↓↓↓↓↓
        // Suppose there is a type `String`, it is used as output for a function defined in a
        // regular block and used as output type of `SyncReturn` version in anthoer function of
        // another regular block, then the above logic would not treat `String` and the `SyncReturn`
        // version of it as shared types, which both should be.
        for (i, set) in regular_block_uniques.iter().enumerate() {
            for item in set {
                // if item.safe_ident().contains("SharedStructOnlyForSyncTest"){
                if shares.contains(item) {
                    continue;
                }
                for (_, other_set) in regular_block_uniques.iter().enumerate().skip(i + 1) {
                    // Note: There is no need to check `item` with that in `shares` additionally.
                    // With cross types got above, the below logic is enough
                    // to cover all other cases.
                    for other_item in other_set {
                        let cond1 = item.safe_ident() == other_item.safe_ident();
                        let cond2 =
                            format!("box_autoadd_{}", item.safe_ident()) == other_item.safe_ident();
                        let cond3 =
                            format!("box_autoadd_{}", other_item.safe_ident()) == item.safe_ident();
                        if cond1 || cond2 || cond3 {
                            shares.insert(item.clone());
                            shares.insert(other_item.clone());
                        }
                    }
                }
            }
        }
        //↑↑↑↑↑↑↑↑↑↑↑↑for types with same idents but different hashes↑↑↑↑↑↑↑↑↑↑↑↑

        shares
    }

    /// For a CROSS shared type which is shared as an input parameter ONLY ONCE in one block
    /// and shared as an output value in ANOTHER/OTHER block(s) or vice versa,
    /// the current logic of `get_regular_distinct_types(..)` would not pick it out,
    /// even both `include_func_inputs` and `include_func_output` are set true.
    /// because the signatures for the same raw type(e.g. `bool`) are different when it is used as input and output of a function.
    /// But indeed, this special kind of type should be treated as shared type within
    /// either input or output context or both.
    /// Thus, here comes an extra manipulation to pick it out.
    fn add_cross_shared_types(
        &self,
        suspected_shared_type: &IrType,
        block_index: BlockIndex,
        shares: &mut HashSet<IrType>,
        regular_ir_files: &[IrFile],
        include_func_inputs: bool,
        include_func_output: bool,
    ) {
        match (include_func_inputs, include_func_output) {
            (true, true) => {
                for ir_file in regular_ir_files.iter() {
                    if ir_file.block_index != block_index {
                        let oppo_distinct_types = ir_file.get_regular_distinct_types(true, true);
                        let found = oppo_distinct_types.iter().find(|each| {
                            let cond1 = format!("box_autoadd_{}", each.safe_ident())
                                == suspected_shared_type.safe_ident();
                            let cond2 =
                                format!("box_autoadd_{}", suspected_shared_type.safe_ident())
                                    == each.safe_ident();
                            let cond3 = suspected_shared_type.safe_ident() == each.safe_ident();
                            cond1 || cond2 || cond3
                        });
                        if found.is_some() {
                            shares.insert(suspected_shared_type.clone());
                            break;
                        }
                    }
                }
            }
            (true, false) => {
                for each_ir_file in regular_ir_files.iter() {
                    if each_ir_file.block_index != block_index {
                        let oppo_distinct_types = each_ir_file
                            .get_regular_distinct_types(!include_func_inputs, !include_func_output);
                        let found = oppo_distinct_types.iter().find(|each| {
                            format!("box_autoadd_{}", each.safe_ident())
                                == suspected_shared_type.safe_ident()
                                || suspected_shared_type.safe_ident() == each.safe_ident()
                        });
                        if found.is_some() {
                            shares.insert(suspected_shared_type.clone());
                            break;
                        }
                    }
                }
            }
            (false, true) => {
                for ir_file in regular_ir_files.iter() {
                    if ir_file.block_index != block_index {
                        let oppo_distinct_types = ir_file
                            .get_regular_distinct_types(!include_func_inputs, !include_func_output);
                        let found = oppo_distinct_types.iter().find(|each| {
                            format!("box_autoadd_{}", suspected_shared_type.safe_ident())
                                == each.safe_ident()
                                || suspected_shared_type.safe_ident() == each.safe_ident()
                        });
                        if found.is_some() {
                            shares.insert(suspected_shared_type.clone());
                            break;
                        }
                    }
                }
            }
            (false, false) => unreachable!(),
        }
    }

    pub fn generate_rust(&self, config: &Opts, all_configs: &[Opts]) -> generator::rust::Output {
        let regular_mod = mod_from_rust_path(config, false).unwrap();
        let shared_mod = if is_multi_blocks_case(None) {
            SHARED_MODULE.with(|text| {
                let mut x = text.borrow_mut();
                if x.is_none() {
                    *x = mod_from_rust_path(config, true);
                }
                x.clone()
            })
        } else {
            None
        };

        generator::rust::generate(
            self,
            &regular_mod,
            shared_mod.as_deref(),
            config,
            all_configs,
        )
    }

    pub fn generate_dart(
        &self,
        config: &Opts,
        all_configs: &[Opts],
        wasm_funcs: &[IrFuncDisplay],
    ) -> generator::dart::Output {
        generator::dart::generate(self, config, all_configs, wasm_funcs)
    }

    /// Get all symbols(function names) defined explicitly or implictily.
    /// It is ok to just parse 1 config, whatever it is for single or multi-blocks case,
    /// since only symbols related to the specific config is needed.
    pub fn get_all_symbols(&self, config: &Opts) -> Vec<String> {
        self.generate_rust(config, &[config.clone()])
            .extern_func_names
    }

    /// check if `ty` is sharely used in current block
    pub fn is_type_shared_by_safe_ident(&self, ty: &IrType) -> ShareMode {
        let get_shared_distinct_types_for_current_block =
            &self.get_shared_distinct_types_for_current_block();
        let found_op = get_shared_distinct_types_for_current_block
            .iter()
            .find(|each| each.safe_ident() == ty.safe_ident());
        if found_op.is_some() {
            return ShareMode::Shared;
        }
        ShareMode::Unique
    }

    /// if `only_for_this_block` is `true`,
    /// then it returns shared types used in current IrFile block(for shared IrFile，it returns all shared types);
    /// otherwise, it returns shared types used in all blocks，whatever the IrFile represents for.
    pub fn get_shared_type_names(
        &self,
        only_for_this_block: bool,
        filter_func: Option<impl Fn(&IrType) -> bool>,
    ) -> Vec<String> {
        let shared_types = match only_for_this_block {
            true => self.get_shared_distinct_types_for_current_block(),
            false => self.get_shared_distinct_types_for_all_blocks(true, true, &[]),
        };

        let shared_types = match filter_func {
            Some(f) => shared_types.into_iter().filter(|ty| f(ty)).collect(),
            None => shared_types,
        };

        let mut f = shared_types
            .iter()
            .map(|ty| {
                let safe_ident = ty.safe_ident();
                if !ty.is_list(true) {
                    safe_ident.to_case(Case::Pascal)
                } else {
                    safe_ident
                }
            })
            .collect::<Vec<_>>();

        // deduplicated
        f.sort_by_key(|ty| ty.to_owned());
        f.dedup();

        f
    }

    pub fn fetched_all_shared_types(&self) -> bool {
        FETCHED_FOR_SHARED_TYPES_INPUT.with(|fetched_input| {
            FETCHED_FOR_SHARED_TYPES_OUTPUT.with(|fetched_output| {
                FETCHED_FOR_SHARED_TYPES_ALL.with(|fetched_all| {
                    *fetched_input.borrow() && *fetched_output.borrow() && *fetched_all.borrow()
                })
            })
        })
    }

    pub fn fetch_shared_types_if_needed(&self, all_configs: &[Opts]) {
        if all_configs.len() > 1 && !self.fetched_all_shared_types() {
            self.get_shared_distinct_types_for_all_blocks(true, true, all_configs);

            self.get_shared_distinct_types_for_all_blocks(true, false, all_configs);

            self.get_shared_distinct_types_for_all_blocks(false, true, all_configs);
        }
    }

    pub fn fetch_shared_types_forcely(&self, all_configs: &[Opts]) {
        SHARED_TYPES_ALL.with(|shared| {
            let mut shares = shared.borrow_mut();

            *shares = self.get_shared_types_for_all_blocks_from_scratch(true, true, all_configs);
        });
        FETCHED_FOR_SHARED_TYPES_ALL.with(|has_fetched| {
            *has_fetched.borrow_mut() = true;
        });

        SHARED_TYPES_INPUT.with(|shared| {
            let mut shares = shared.borrow_mut();

            *shares = self.get_shared_types_for_all_blocks_from_scratch(true, false, all_configs);
        });
        FETCHED_FOR_SHARED_TYPES_INPUT.with(|has_fetched| {
            *has_fetched.borrow_mut() = true;
        });

        SHARED_TYPES_OUTPUT.with(|shared| {
            let mut shares = shared.borrow_mut();

            *shares = self.get_shared_types_for_all_blocks_from_scratch(false, true, all_configs);
        });
        FETCHED_FOR_SHARED_TYPES_OUTPUT.with(|has_fetched| {
            *has_fetched.borrow_mut() = true;
        });
    }
}
