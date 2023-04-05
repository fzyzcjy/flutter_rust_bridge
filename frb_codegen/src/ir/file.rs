use convert_case::{Case, Casing};

use crate::target::Target;
use crate::transformer::tranform_input_type;
use crate::utils::misc::{mod_from_rust_path, BlockIndex};
use crate::{generator, ir::*, Opts};
use std::collections::{HashMap, HashSet};

use crate::utils::misc::ExtraTraitForVec;

pub type IrStructPool = HashMap<String, IrStruct>;
pub type IrEnumPool = HashMap<String, IrEnum>;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct IrFile {
    pub funcs: Vec<IrFunc>,
    pub struct_pool: IrStructPool,
    pub enum_pool: IrEnumPool,
    pub has_executor: bool,
    pub block_index: BlockIndex,
    pub shared_types: HashSet<IrType>, // if there is no shared IrType among API blocks, it should be empty
    pub shared: bool, // `true` for instance in charge of an auto-generated shared API block
}

impl IrFile {
    /// NOTE: if `all_configs` is empty or with only 1 element, no shared_types would be produced
    pub fn new(
        funcs: Vec<IrFunc>,
        struct_pool: IrStructPool,
        enum_pool: IrEnumPool,
        has_executor: bool,
        block_index: BlockIndex,
        shared: bool,
        all_configs: &[Opts],
    ) -> Self {
        let mut ir_file = IrFile {
            funcs,
            struct_pool,
            enum_pool,
            has_executor,
            block_index,
            shared_types: HashSet::new(),
            shared,
        };
        ir_file.shared_types = ir_file.get_shared_distinct_types(true, true, all_configs);
        log::debug!("the init shared types are:{:?}", ir_file.shared_types); //TODO: delete
        ir_file
    }

    /// [f] returns [true] if it wants to stop going to the *children* of this subtree
    pub fn visit_types<F: FnMut(&IrType) -> bool>(
        &self,
        f: &mut F,
        include_func_inputs: bool,
        include_func_output: bool,
    ) {
        for func in &self.funcs {
            // log::debug!("the func is :{func:?}"); //TODO: delete
            if include_func_inputs {
                // log::debug!("start visit input types of all len:{}", func.inputs.len()); //TODO: delete
                for field in &func.inputs {
                    field.ty.visit_types(f, self);
                }
            }
            if include_func_output {
                // log::debug!("start visit output types"); //TODO: delete
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

    /// get distinct types for this instance,
    /// which could be either for a regular or for an auto-generated shared API block
    pub fn distinct_types(
        &self,
        include_func_inputs: bool,
        include_func_output: bool,
        all_configs: &[Opts],
    ) -> Vec<IrType> {
        let shared_types =
            self.get_shared_distinct_types(include_func_inputs, include_func_output, all_configs);
        let types =
            match !self.shared {
                true => {
                    let raw_distinct_types =
                        self.get_regular_distinct_types(include_func_inputs, include_func_output);
                    log::debug!("the raw disntinct types for {include_func_inputs}-{include_func_output}:{:?}", raw_distinct_types); //TODO: delete
                    let collect = raw_distinct_types
                        .difference(&shared_types)
                        .cloned()
                        .collect();
                    log::debug!("the final disntinct types:{:?}", collect); //TODO: delete
                    collect
                }
                false => shared_types,
            };

        let mut types = types.into_iter().collect::<Vec<_>>();
        types.sort_by_key(|ty| ty.safe_ident());
        types
    }

    /// get dinstinct types only for a certain regular API block(the current IrFile instance)
    fn get_regular_distinct_types(
        &self,
        include_func_inputs: bool,
        include_func_output: bool,
    ) -> HashSet<IrType> {
        assert!(!(!include_func_inputs && !include_func_output));
        assert!(!self.shared);
        let mut ans = HashSet::new();
        self.visit_types(
            &mut |ty| !ans.insert(ty.clone()),
            include_func_inputs,
            include_func_output,
        );
        log::debug!("the set ans len: {} is: {:?}", ans.len(), ans); //TODO: delete
        ans
    }

    /// Whatever this IrFile instance represents for, get shared types through `all_configs`, which
    /// may or may not include the shared config at the last index.
    /// For single-block case, it should return empty.
    /// For multi-block case, it should return ALL shared types among ALL regular
    /// blocks, which means that some of the shared types may not be used in all regular blocks.
    /// Also note `include_func_inputs` and `include_func_output` are essential for getting
    /// shared types within different context.
    fn get_shared_distinct_types(
        &self,
        include_func_inputs: bool,
        include_func_output: bool,
        all_configs: &[Opts],
    ) -> HashSet<IrType> {
        assert!(!(!include_func_inputs && !include_func_output));
        log::debug!("start get_shared_distinct_types"); //TODO: delete
        if all_configs.is_empty() || all_configs.len() == 1 {
            log::debug!("empty, return"); //TODO: delete
            return HashSet::new();
        }
        let regular_configs = if !all_configs.last().unwrap().shared {
            all_configs
        } else {
            &all_configs[0..all_configs.len() - 1]
        };
        assert!(regular_configs.iter().all(|c| !c.shared));
        let mut cur_block_uniques = HashSet::new();
        let mut all_regular_types = Vec::new();
        let mut regular_ir_files = Vec::new();
        let mut maps = HashMap::<IrType, Vec<_>>::new();
        for config in regular_configs.iter() {
            let ir_file = config.get_ir_file(&[]).unwrap();
            let distinct_types =
                ir_file.get_regular_distinct_types(include_func_inputs, include_func_output);
            regular_ir_files.push(ir_file.clone());
            all_regular_types.extend(distinct_types.clone());

            if config.block_index == self.block_index {
                cur_block_uniques.extend(distinct_types.clone())
            }
            for each in distinct_types {
                maps.entry(each)
                    .and_modify(|value: _| {
                        value.push(ir_file.block_index);
                    })
                    .or_insert(vec![ir_file.block_index]);
            }
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
        ); //TODO: delete

        //↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓for cross shared types↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓
        // NOTE: For a special kind of cross shared type,
        // which is shared as an input parameter ONLY ONCE in one block
        // and shared as an output value ONLY ONCE in ANOTHER block,
        // the current logic from `get_regular_distinct_types(..)` would not pick it out,
        // even both `include_func_inputs` and `include_func_output` are set true.
        // But this special kind of cross shared type should be treated as a shared type in
        // either in input or output context, even it is not shared in all of regular API blocks.
        // Thus, here comes an extra manipulation to pick it out.
        fn add_cross_shared_types(
            shares: &mut HashSet<IrType>,
            regular_ir_files: &[IrFile],
            block_index: BlockIndex,
            include_func_inputs: bool,
            include_func_output: bool,
            suspected_type: &IrType,
        ) {
            // TODO: refine. The code here is a little overhead, but workable.
            for ir_file in regular_ir_files.iter() {
                if ir_file.block_index != block_index {
                    let oppo_distinct_types = ir_file
                        .get_regular_distinct_types(!include_func_inputs, !include_func_output);
                    if oppo_distinct_types.contains(suspected_type) {
                        shares.insert(suspected_type.clone());
                        // because the type is indeed different for input(parameter) and output(return value),
                        // That is, for a cross shared type, both the input and output ones for this specific type
                        // should be treated as shared types.
                        shares.insert(tranform_input_type(suspected_type));
                        break;
                    }
                }
            }
        }

        if include_func_inputs != include_func_output {
            if !self.shared {
                for suspected_type in &cur_block_uniques {
                    add_cross_shared_types(
                        &mut shares,
                        &regular_ir_files,
                        self.block_index,
                        include_func_inputs,
                        include_func_output,
                        suspected_type,
                    );
                }
            } else {
                let matches = maps
                    .iter()
                    .filter(|&(_, v)| v.len() == 1)
                    .map(|(k, v)| (v[0], k))
                    .collect::<Vec<_>>();

                for (block_index, suspected_type) in matches {
                    add_cross_shared_types(
                        &mut shares,
                        &regular_ir_files,
                        block_index,
                        include_func_inputs,
                        include_func_output,
                        suspected_type,
                    );
                }
            }
        }
        //↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑for cross shared types↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑

        log::debug!(
            "the new shared for index:{} {include_func_inputs}-{include_func_output} is:{:?}",
            self.block_index.0,
            shares
        ); //TODO: delete

        shares
    }

    pub fn generate_rust(&self, config: &Opts, all_configs: &[Opts]) -> generator::rust::Output {
        log::debug!("generate_rust 1"); //TODO: delete

        let regular_mod = mod_from_rust_path(config, all_configs, false).unwrap();
        let shared_mod = mod_from_rust_path(config, all_configs, true);
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

    /// get all symbols(function names) defined explicitly or implictily
    pub fn get_all_symbols(&self, config: &Opts, all_configs: &[Opts]) -> Vec<String> {
        self.generate_rust(config, all_configs).extern_func_names
    }

    pub fn is_type_shared(&self, ty: &IrType) -> bool {
        if ty.safe_ident().contains("box_autoadd_") {
            // though `self.shared_types` collects both input and output types,
            // "cross" shared types are not distinguished from input and output yet.
            // Therefore, here work around it by manually check `box_autoadd_`, which
            // is used as an input type prefix.
            // for more info, please take a look at `get_shared_distinct_types(...)`
            log::debug!("now checking {}", ty.safe_ident()); //TODO: delete
            for each in &self.shared_types {
                if ty.safe_ident().contains(&each.safe_ident()) {
                    log::debug!("{} is shared!", ty.safe_ident()); //TODO: delete
                    return true;
                }
            }
        } else {
            return self.shared_types.contains(ty);
        }
        false
    }

    pub fn get_shared_type_names(&self) -> HashSet<String> {
        self.shared_types
            .iter()
            .flat_map(|ty| {
                let safe_ident = ty.safe_ident();
                std::iter::once(safe_ident.clone())
                    .chain(std::iter::once(safe_ident.to_case(Case::UpperCamel)))
            })
            .collect()
    }
}
