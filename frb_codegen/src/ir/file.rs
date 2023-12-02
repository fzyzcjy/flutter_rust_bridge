use crate::utils::misc::{get_rust_module_by_path, BlockIndex, ExtraTraitForVec, IrTypeUseRange};
use crate::{ir::*, parser};
use std::collections::{BTreeMap, HashMap, HashSet};

pub type IrStructPool = HashMap<String, IrStruct>;
pub type IrEnumPool = HashMap<String, IrEnum>;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct IrFile {
    manifest_path: String,  // the full path to the Rust Cargo.toml file
    path: String,           // the full path to the Rust file defining this IrFile
    pub funcs: Vec<IrFunc>, // IrFuncs used in this IrFile
    pub types: Vec<IrType>, // IrTypes used in this IrFile
    pub struct_pool: IrStructPool,
    pub enum_pool: IrEnumPool,
    pub has_executor: bool,
    pub types_use_range_map: HashMap<IrType, HashSet<IrTypeUseRange>>,
    pub block_index: BlockIndex, // Regular blocks takes index starting from 0 with `Some`. For the shared block, this field should be `None`.
}

impl IrFile {
    pub fn new(
        manifest_path: &str,
        rust_file_path: &str,
        funcs: Vec<IrFunc>,
        shared_types_map: Option<HashMap<IrType, HashSet<IrTypeUseRange>>>,
        struct_pool: IrStructPool,
        enum_pool: IrEnumPool,
        has_executor: bool,
        block_index: BlockIndex,
    ) -> IrFile {
        // raw ir file
        let mut ir_file = IrFile {
            manifest_path: manifest_path.to_string(),
            path: rust_file_path.to_string(),
            funcs,
            types: vec![],
            struct_pool,
            enum_pool,
            has_executor,
            block_index,
            types_use_range_map: HashMap::new(),
        };
        // refine fields
        ir_file.funcs.iter_mut().for_each(|f| f.refine());
        ir_file.refine_types(shared_types_map);
        // return
        ir_file
    }

    /// It returns `true` if it is for an auto-generated block
    /// for shared stuff(`shared block` for short); Otherwise it would return `false`.
    pub fn is_shared(&self) -> bool {
        self.block_index.is_shared()
    }

    fn refine_types(&mut self, shared_types_map: Option<HashMap<IrType, HashSet<IrTypeUseRange>>>) {
        if let Some(types_map) = shared_types_map {
            assert!(self.is_shared());
            assert!(self.types_use_range_map.is_empty());
            self.types = types_map.keys().cloned().collect();
            self.types_use_range_map = types_map;
        } else {
            assert!(!self.is_shared());
            // collect inputs and outputs into self.types_use_range_map
            let inputs = self
                .funcs
                .iter()
                .flat_map(|f| f.get_types_used(true, false, false, self))
                .collect::<Vec<_>>()
                .find_uniques_in_order(false);
            let outputs = self
                .funcs
                .iter()
                .flat_map(|f| f.get_types_used(false, true, false, self))
                .collect::<Vec<_>>()
                .find_uniques_in_order(false);

            assert!(self.types_use_range_map.is_empty());
            for ir_type in &inputs {
                self.types_use_range_map
                    .insert(ir_type.clone(), HashSet::from([IrTypeUseRange::Input]));
            }
            for ir_type in outputs {
                self.types_use_range_map
                    .entry(ir_type)
                    .and_modify(|value| {
                        value.insert(IrTypeUseRange::Output);
                    })
                    .or_insert(HashSet::from([IrTypeUseRange::Output]));
            }
        }

        log::debug!(
            "for block:{}, types_use_range_map:\n{:#?}",
            self.block_index,
            self.types_use_range_map
        ); // TODO: delete

        // refine types_use_range_map
        // if the key is a boxed type, then make sure its value has no output,
        // if the key is a syncReturn type, then make sure its value has no input.
        for (ir_type, type_use_range) in self.types_use_range_map.iter_mut() {
            assert!(
                !type_use_range.is_empty(),
                "the type `{:?}` with safe_ident `{}` is empty before refinement",
                ir_type,
                ir_type.safe_ident()
            );
            if let IrType::Boxed(_) = ir_type {
                type_use_range.remove(&IrTypeUseRange::Output);
            }
            if let IrType::SyncReturn(_) = ir_type {
                type_use_range.remove(&IrTypeUseRange::Input);
            }
            assert!(
                !type_use_range.is_empty(),
                "the type `{:?}` with safe_ident `{}` is empty after refinement",
                ir_type,
                ir_type.safe_ident()
            );
        }

        // collect self.types
        self.types = self.types_use_range_map.keys().cloned().collect();
    }

    pub fn set_funcs(&mut self, dst_funcs: Vec<IrFunc>) {
        self.funcs = dst_funcs;
    }

    pub fn add_func(&mut self, dst_func: IrFunc) {
        self.funcs.push(dst_func);
    }

    pub fn set_shared_types(&mut self, types: Vec<IrType>) {
        assert!(self.block_index.is_shared());
        self.types = types;
    }

    /// This method would parse all types and Apis(functions and methods) used but not defined in the original regular Api file.
    /// In detail, it would update `self.types` and the `self.funcs` for the IrFile instance.
    pub fn parse_types_and_methods_in_extra_files_for_regular_block(
        &mut self,
        avoid_file_paths: &[&String],
    ) {
        assert!(!self.is_shared());
        log::info!(
            "Phase: Refine Api and Types defined in extra rust files for regular block: {}",
            self.block_index
        );

        // 1. define type_pool_map from `struct_pool` and `enum_pool` in an ordered map
        let mut file_type_map = BTreeMap::new();
        for value in self.struct_pool.values() {
            let struct_paths = value.path.clone().unwrap();
            let raw_path = format!("{}.rs", struct_paths[..struct_paths.len() - 1].join("/"));
            let type_name = struct_paths.last().unwrap();

            file_type_map
                .entry(raw_path.clone())
                .and_modify(|v: &mut Vec<String>| v.push(type_name.to_string().clone()))
                .or_insert(vec![type_name.to_string()]);
        }
        for value in self.enum_pool.values() {
            let enum_paths = value.path.clone();
            let raw_path = format!("{}.rs", enum_paths[..enum_paths.len() - 1].join("/"));
            let type_name: &String = enum_paths.last().unwrap();
            file_type_map
                .entry(raw_path.clone())
                .and_modify(|v: &mut Vec<String>| v.push(type_name.to_string().clone()))
                .or_insert(vec![type_name.to_string()]);
        }

        if self.block_index == Some(0) {
            log::debug!(
                "the block 0 map len is:{:?}, they are:\n{file_type_map:?}",
                file_type_map.len()
            ); // TODO: delete
        }
        // 2. add extra methods and types defined in extra rust files
        for (type_def_path, types_names) in &file_type_map {
            for each_type_name in types_names {
                // 2.1 parse the extra rust file with the specific type
                assert!(type_def_path.contains("crate/"));
                let extra_rust_file_path = type_def_path
                    .replace("crate/", &self.manifest_path.replace("Cargo.toml", "src/"));
                if get_rust_module_by_path(&extra_rust_file_path).is_none() {
                    continue;
                }
                if avoid_file_paths.contains(&&extra_rust_file_path) {
                    continue;
                }
                // 2.2 parse the extra rust file for the specific type
                let extra_type_ir_file = parser::parse_a_rust_file(
                    &self.manifest_path,
                    &extra_rust_file_path,
                    self.block_index,
                    Some(&[each_type_name]),
                )
                .unwrap();
                // 2.3 combine the extra ir file into the original ir file
                self.combine(extra_type_ir_file);
            }
        }
    }

    /// Combine `extra_ir_file` into `self`.
    /// A minor issue: if a type defined in `extra_ir_file` is already in `self.types`,
    /// it would be removed. While logically, even with the same type definition,
    /// as long as they are defined in different files, they should be treated as distinct types.
    /// This "issue" is ignored, since defining types with the same name but in different files
    /// is not a good practice.
    fn combine(&mut self, extra_ir_file: Self) {
        // refine funcs
        self.funcs.extend(extra_ir_file.funcs.clone());
        self.funcs = self.funcs.find_uniques_in_order(false);
        // refine types
        self.types.extend(extra_ir_file.types.clone());
        self.types = self.types.find_uniques_in_order(false);
        // extend `extra_ir_file.struct_pool` using loop to avoid duplication
        for (key, value) in extra_ir_file.struct_pool {
            self.struct_pool.entry(key).or_insert(value);
        }
        // extend `extra_ir_file.enum_pool` using loop to avoid duplication
        for (key, value) in extra_ir_file.enum_pool {
            self.enum_pool.entry(key).or_insert(value);
        }
        // extend `type_use_range_map` using loop to avoid duplication
        for (key, value) in extra_ir_file.types_use_range_map {
            self.types_use_range_map
                .entry(key)
                .and_modify(|v| v.extend(value.clone()))
                .or_insert(value);
        }
    }
    /// Get the type use range for the specific type.
    /// And the type must be defined in the current ir_file,
    /// otherwise it would panic.
    pub(crate) fn get_type_use_range(&self, ir_type: &IrType) -> HashSet<IrTypeUseRange> {
        let v = self.types_use_range_map.get(ir_type).cloned().unwrap();
        assert!(!v.is_empty(), "the type:{:?} is empty", ir_type);
        v
    }
}
