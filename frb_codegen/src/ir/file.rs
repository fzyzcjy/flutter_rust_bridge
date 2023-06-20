use convert_case::{Case, Casing};

use crate::target::Target;
use crate::utils::misc::{mod_from_rust_path, BlockIndex};
use crate::{generator, ir::*, Opts};
use std::collections::{HashMap, HashSet};
use std::thread;

use crate::utils::misc::ExtraTraitForVec;

pub type IrStructPool = HashMap<String, IrStruct>;
pub type IrEnumPool = HashMap<String, IrEnum>;

use std::cell::RefCell;

// this variable should be `none` for single block case, and not none for multi-blocks case.
thread_local!(pub static SHARED_MODULE: RefCell<Option<String>> = RefCell::new(None));

thread_local!(pub static SHARED_TYPES_INPUT: RefCell<HashSet<IrType>> = RefCell::new(HashSet::new())); //  shared types among all regular API blocks, in context of function paramters
thread_local!(static FETCHED_FOR_SHARED_TYPES_INPUT: RefCell<bool> = RefCell::new(false));

thread_local!(static SHARED_TYPES_OUTPUT: RefCell<HashSet<IrType>> = RefCell::new(HashSet::new())); //  shared types among all regular API blocks in context of function outputs
thread_local!(static FETCHED_FOR_SHARED_TYPES_OUTPUT: RefCell<bool> = RefCell::new(false));

thread_local!(static SHARED_TYPES_ALL: RefCell<HashSet<IrType>> = RefCell::new(HashSet::new())); //  shared types among all regular API blocks
thread_local!(static FETCHED_FOR_SHARED_TYPES_ALL: RefCell<bool> = RefCell::new(false));

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct IrFile {
    /// for a regular API block, this field contains APIs defined in this block and
    /// all methods of types(struct/enum)---the types are used only in this block;
    /// for a shared block, it ONLY contains methods from ALL shared types.
    pub funcs: Vec<IrFunc>,
    pub struct_pool: IrStructPool,
    pub enum_pool: IrEnumPool,
    pub has_executor: bool,
    pub block_index: BlockIndex,
    pub shared: bool, // `true` for an auto-generated shared block, otherwise `false`
}

impl IrFile {
    pub fn new(
        funcs: Vec<IrFunc>,
        struct_pool: IrStructPool,
        enum_pool: IrEnumPool,
        has_executor: bool,
        block_index: BlockIndex,
        all_configs: &[Opts],
        shared: bool,
    ) -> Self {
        log::debug!("init new IrFile"); //TODO: delete
        let ir_fie = IrFile {
            funcs,
            struct_pool,
            enum_pool,
            has_executor,
            block_index,
            shared,
        };
        log::debug!("Finish init new IrFile"); //TODO: delete

        // Get shared types in advance here, because these types will be used multiple times later for multi-blocks case.
        ir_fie.fetch_shared_types_if_needed(all_configs);

        log::debug!("Finish after all_configs new IrFile"); //TODO: delete

        ir_fie
    }

    /// [f] returns [true] if it wants to stop going to the *children* of this subtree
    pub fn visit_types<F: FnMut(&IrType) -> bool>(
        &self,
        f: &mut F,
        include_func_inputs: bool,
        include_func_output: bool,
    ) {
        for func in &self.funcs {
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

    /// get distinct types for this IrFile instance,
    /// for either a regular or an auto-generated shared API block.
    /// NOTE: in multi-blocks case, the output would NOT include shared types for a regular block IrFile.
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

        log::debug!(
            "block_{}, the shared_types:{:?}",
            self.block_index,
            shared_types
        ); //TODO: delete

        let types = match !self.shared {
            true => {
                let raw_distinct_types =
                    self.get_regular_distinct_types(include_func_inputs, include_func_output);

                log::debug!(
                    "block_{}, the raw_distinct_types:{:?}",
                    self.block_index,
                    raw_distinct_types
                ); //TODO: delete

                // way1: distinguished by `safe_ident()`
                let r = raw_distinct_types
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
                                (true, false) => {
                                    // assert!(item.safe_ident().contains("box_autoadd_"));
                                    cond1 || cond3
                                }
                                (false, true) => {
                                    // assert!(!item.safe_ident().contains("box_autoadd_"));
                                    cond1 || cond3
                                }
                                (false, false) => unreachable!(),
                            }
                        })
                    })
                    .collect::<HashSet<_>>();

                // way2: distinguished by hash. Not sure if this is needed
                // let r = raw_distinct_types
                //     .difference(&shared_types)
                //     .cloned()
                //     .collect();

                log::debug!(
                    "block_{}, the final_distinct_types:{:?}",
                    self.block_index,
                    r
                ); //TODO: delete

                r
            }
            false => shared_types,
        };

        let mut types = types.into_iter().collect::<Vec<_>>();
        types.sort_by_key(|ty| ty.safe_ident());
        types
    }

    /// Get dinstinct types only for a regular API block(the current IrFile instance).
    /// NOTE: in multi-blocks case, the returned items would include shared types
    /// usded in this regular block.
    fn get_regular_distinct_types(
        &self,
        include_func_inputs: bool,
        include_func_output: bool,
    ) -> HashSet<IrType> {
        assert!(include_func_inputs || include_func_output);
        assert!(!self.shared);
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
        if self.shared {
            global_shared
        } else {
            self.get_regular_distinct_types(true, true)
                .intersection(&global_shared)
                .map(|s| s.to_owned())
                .collect()
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
                        log::debug!("got ref shared"); //TODO: delete

                        if all_configs.len() <= 1 {
                            // CASE 1: it is single block case;
                            // CASE 2: it is multi-blocks case, but `all_configs` is not available;
                            return shares.clone();
                        }

                        let mut fetched: std::cell::RefMut<bool> = has_fetched.borrow_mut();
                        if *fetched {
                            return shares.clone();
                        }

                        log::debug!("before get_shared_types_for_all_blocks_from_scratch"); //TODO: delete
                        *shares = self.get_shared_types_for_all_blocks_from_scratch(
                            include_func_inputs,
                            include_func_output,
                            all_configs,
                        );
                        log::debug!("after get_shared_types_for_all_blocks_from_scratch"); //TODO: delete

                        *fetched = true;
                        shares.clone()
                    })
                })
            };

        match (include_func_inputs, include_func_output) {
            (true, true) => {
                log::debug!("start for SHARED_TYPES_ALL"); //TODO: delete
                fetch_func(&SHARED_TYPES_ALL, &FETCHED_FOR_SHARED_TYPES_ALL)
            }
            (true, false) => {
                log::debug!("start for SHARED_TYPES_INPUT"); //TODO: delete
                fetch_func(&SHARED_TYPES_INPUT, &FETCHED_FOR_SHARED_TYPES_INPUT)
            }
            (false, true) => {
                log::debug!("start for SHARED_TYPES_OUTPUT"); //TODO: delete
                fetch_func(&SHARED_TYPES_OUTPUT, &FETCHED_FOR_SHARED_TYPES_OUTPUT)
            }
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
        // get it from scratch for multi-blocks case
        let regular_configs = if !all_configs.last().unwrap().shared {
            all_configs
        } else {
            &all_configs[0..all_configs.len() - 1]
        };
        assert!(regular_configs.iter().all(|c| !c.shared));
        let mut regular_block_uniques = Vec::new();
        let mut all_regular_types = Vec::new();
        let mut regular_ir_files = Vec::new();
        for (i, config) in regular_configs.iter().enumerate() {
            log::debug!("before get regular irfile for {i}"); //TODO: delete
            let ir_file = config.get_ir_file(&[]).unwrap(); // all_configs` is empty, no need to care about other configs here
            log::debug!("after get regular irfile for {i}"); //TODO: delete

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
        log::debug!(
            "the raw shared for index:{} {include_func_inputs}-{include_func_output} is:{:?}",
            self.block_index.0,
            shares
        );
        //TODO: delete

        //↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓for cross shared types↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓
        if !self.shared {
            log::debug!("block_uniques:{regular_block_uniques:?}"); //TODO: delete
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

        // TODO: delete? The below block of code may be redundant check?
        //↓↓↓↓↓↓↓↓↓↓↓↓for types with same idents but different hashes↓↓↓↓↓↓↓↓↓↓↓↓
        // Suppose there is a type `String`, it is used as output for a function defined in a
        // regular block and used as output type of `SyncReturn` version in anthoer function of
        // another regular block, then the above logic would not treat `String` and the `SyncReturn`
        // version of it as shared types, which both should be.
        for (i, set) in regular_block_uniques.iter().enumerate() {
            for item in set {
                // if item.safe_ident().contains("SharedStructOnlyForSyncTest"){
                log::debug!(
                    "the check IrType:{:?} with safe_ident:{} in block {}",
                    item,
                    item.safe_ident(),
                    i
                ); //TODO: delete
                   // }
                if shares.contains(item) {
                    log::debug!("already contained "); //TODO: delete
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

                            log::debug!("inserted {:?}", item); //TODO: delete
                            log::debug!("inserted {:?}", other_item); //TODO: delete
                        }
                    }
                }
            }
        }
        //↑↑↑↑↑↑↑↑↑↑↑↑for types with same idents but different hashes↑↑↑↑↑↑↑↑↑↑↑↑
        log::debug!(
            "the new shared for index:{} {include_func_inputs}-{include_func_output} is:{:?}",
            self.block_index.0,
            shares
        );
        //TODO: delete

        shares
    }

    /// For a CROSS shared type which is shared as an input parameter ONLY ONCE in one block
    /// and shared as an output value in ANOTHER/OTHER block(s) or vice versa,
    /// the current logic of `get_regular_distinct_types(..)` would not pick it out,
    /// even both `include_func_inputs` and `include_func_output` are set true.
    /// because the signatures for the same raw type(e.g. `bool`) are different when it is used as input and output of a function.
    /// But indeed, this special kind of type should be treated as shared type within
    /// either input or output context or both, even it is shared but not shared in ALL of regular API blocks.
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
                            log::debug!(
                                "insert `{:?}` for {}-{}",
                                suspected_shared_type,
                                include_func_inputs,
                                include_func_output
                            ); //TODO: delete
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
                            log::debug!(
                                "insert `{:?}` for {}-{}",
                                suspected_shared_type,
                                include_func_inputs,
                                include_func_output
                            ); //TODO: delete
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
                            log::debug!(
                                "insert `{:?}` for {}-{}",
                                suspected_shared_type,
                                include_func_inputs,
                                include_func_output
                            ); //TODO: delete
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
        log::debug!("generate_rust 1"); //TODO: delete

        let regular_mod = mod_from_rust_path(config, all_configs, false).unwrap();
        let shared_mod = SHARED_MODULE.with(|text| {
            log::debug!("inner all configs len:{:?}", all_configs.len()); //TODO: delete
                                                                          // TODO：check option first?
            *text.borrow_mut() = mod_from_rust_path(config, all_configs, true);
            text.borrow().clone()
        });
        log::debug!("regular mod:{regular_mod}"); //TODO: delete
        log::debug!("shared mod:{:?}", shared_mod); //TODO: delete

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
    pub fn is_type_shared_by_safe_ident(&self, ty: &IrType) -> bool {
        let get_shared_distinct_types_for_current_block =
            &self.get_shared_distinct_types_for_current_block();
        let found_op = get_shared_distinct_types_for_current_block
            .iter()
            .find(|each| each.safe_ident() == ty.safe_ident());
        found_op.is_some()
    }

    pub fn get_shared_type_names(&self) -> HashSet<String> {
        self.get_shared_distinct_types_for_current_block()
            .iter()
            .flat_map(|ty| {
                let safe_ident = ty.safe_ident();
                std::iter::once(safe_ident.clone())
                    .chain(std::iter::once(safe_ident.to_case(Case::UpperCamel)))
            })
            .collect()
    }

    /// Get all shared methods used in current block.
    /// This method should only be called for a regular IrFile instance.
    pub fn get_shared_funcs(&self) -> Vec<IrFunc> {
        if self.shared {
            panic!("`get_shared_methods` should not be called from a shared ir_file")
        }
        self.funcs
            .iter()
            .filter(|irfunc| irfunc.shared)
            .cloned()
            .collect()
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
            log::debug!("start for global"); //TODO: delete
            self.get_shared_distinct_types_for_all_blocks(true, true, all_configs);
            log::debug!("start for input"); //TODO: delete
            self.get_shared_distinct_types_for_all_blocks(true, false, all_configs);
            log::debug!("start for output"); //TODO: delete
            self.get_shared_distinct_types_for_all_blocks(false, true, all_configs);
        }
    }
    pub fn fetch_shared_types_forcely(&self, all_configs: &[Opts]) {
        log::debug!("start for global"); //TODO: delete
        SHARED_TYPES_ALL.with(|shared| {
            let mut shares = shared.borrow_mut();
            log::debug!("the old shares len={}", shares.len()); //TODO: delete
            *shares = self.get_shared_types_for_all_blocks_from_scratch(true, true, all_configs);
            log::debug!("the new shares len={}", shares.len()); //TODO: delete
        });
        FETCHED_FOR_SHARED_TYPES_ALL.with(|has_fetched| {
            *has_fetched.borrow_mut() = true;
        });

        log::debug!("start for input"); //TODO: delete
        SHARED_TYPES_INPUT.with(|shared| {
            let mut shares = shared.borrow_mut();
            log::debug!("the old shares len={}", shares.len()); //TODO: delete
            *shares = self.get_shared_types_for_all_blocks_from_scratch(true, false, all_configs);
            log::debug!("the new shares len={}", shares.len()); //TODO: delete
        });
        FETCHED_FOR_SHARED_TYPES_INPUT.with(|has_fetched| {
            *has_fetched.borrow_mut() = true;
        });

        log::debug!("start for output"); //TODO: delete
        SHARED_TYPES_OUTPUT.with(|shared| {
            let mut shares = shared.borrow_mut();
            log::debug!("the old shares len={}", shares.len()); //TODO: delete
            *shares = self.get_shared_types_for_all_blocks_from_scratch(false, true, all_configs);
            log::debug!("the new shares len={}", shares.len()); //TODO: delete
        });
        FETCHED_FOR_SHARED_TYPES_OUTPUT.with(|has_fetched| {
            *has_fetched.borrow_mut() = true;
        });
    }
}
