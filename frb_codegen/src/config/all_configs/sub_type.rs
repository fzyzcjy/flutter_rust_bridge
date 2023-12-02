use std::collections::HashSet;

use crate::ir::{IrFile, IrFunc, IrType};
use crate::utils::misc::{BlockIndex, IrTypeUseRange};

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct IrFuncContext {
    ir_func: IrFunc,
    // TODO: delete parent_type_name: Option<IrType>, // if `ir_func` is for a method, this filed is the inner type name
    // TODO: delete defined_path: String,             // the path the api defined in
    used_block_indices: HashSet<BlockIndex>, // the block indice(including shared block) the api used in
}

impl IrFuncContext {
    pub fn new(ir_func: &IrFunc, regular_ir_file: &IrFile) -> IrFuncContext {
        IrFuncContext {
            ir_func: ir_func.clone(),
            used_block_indices: HashSet::from([regular_ir_file.block_index]),
        }
    }
    pub fn get_func(&self) -> &IrFunc {
        &self.ir_func
    }
    #[allow(unused)]
    pub fn get_func_name(&self) -> &str {
        &self.ir_func.name
    }
    pub fn get_block_indices(&self) -> &HashSet<BlockIndex> {
        &self.used_block_indices
    }
    pub fn insert_block_index(&mut self, block_index: BlockIndex) {
        assert!(block_index != BlockIndex::shared());
        self.used_block_indices.insert(block_index);
        if self.used_block_indices.len() >= 2 {
            self.used_block_indices.insert(BlockIndex::shared());
        }
    }
    pub fn is_shared(&self) -> bool {
        log::debug!("the used block indices are:{:?}", self.used_block_indices); // TODO: delete
        self.used_block_indices.len() > 1
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct IrTypeContext {
    ir_type: IrType,
    use_range: HashSet<IrTypeUseRange>,
    safe_ident_use_range: HashSet<IrTypeUseRange>, // similar to `use_range`, but differ by `safe_ident()` function of IrType
    // TODO: delete defined_path: String,                 // the path the type defined in
    used_block_indices: HashSet<BlockIndex>, // the block indice(including shared block) the type used in
    safe_ident_used_block_indices: HashSet<BlockIndex>, // the block indice(including shared block) the type used in
}

impl IrTypeContext {
    pub fn new(ir_type: &IrType, regular_ir_file: &IrFile) -> IrTypeContext {
        IrTypeContext {
            ir_type: ir_type.clone(),
            use_range: regular_ir_file.get_type_use_range(ir_type),
            used_block_indices: HashSet::from([regular_ir_file.block_index]),
            safe_ident_use_range: HashSet::from([]),
            safe_ident_used_block_indices: HashSet::from([]),
        }
    }

    pub fn get_type(&self) -> &IrType {
        &self.ir_type
    }
    pub fn get_block_indices(&self, safe_ident: bool) -> &HashSet<BlockIndex> {
        if safe_ident {
            &self.safe_ident_used_block_indices
        } else {
            &self.used_block_indices
        }
    }
    pub fn get_use_range(&self, safe_ident: bool) -> &HashSet<IrTypeUseRange> {
        if safe_ident {
            &self.safe_ident_use_range
        } else {
            &self.use_range
        }
    }
    pub fn insert_block_index(&mut self, block_index: BlockIndex, safe_ident: bool) {
        assert!(block_index != BlockIndex::shared());
        if safe_ident {
            self.safe_ident_used_block_indices.insert(block_index);
            if self.safe_ident_used_block_indices.len() >= 2 {
                self.safe_ident_used_block_indices
                    .insert(BlockIndex::shared());
            }
        } else {
            self.used_block_indices.insert(block_index);
            if self.used_block_indices.len() >= 2 {
                self.used_block_indices.insert(BlockIndex::shared());
            }
        }
    }
    #[allow(unused)]
    pub fn insert_use_range(&mut self, use_range: IrTypeUseRange, safe_ident: bool) {
        if safe_ident {
            self.safe_ident_use_range.insert(use_range);
        } else {
            self.use_range.insert(use_range);
        }
    }
    pub fn extend_use_range(&mut self, use_range: HashSet<IrTypeUseRange>, safe_ident: bool) {
        if safe_ident {
            self.safe_ident_use_range.extend(use_range);
        } else {
            self.use_range.extend(use_range);
        }
    }
    fn is_shared(&self, safe_ident: bool) -> bool {
        if safe_ident {
            self.safe_ident_used_block_indices.len() > 1
        } else {
            self.used_block_indices.len() > 1
        }
    }
    pub fn is_input(&self, exclude_shared: bool, safe_ident: bool) -> bool {
        let shared_condition = !exclude_shared || !self.is_shared(safe_ident);
        if safe_ident {
            self.safe_ident_use_range.contains(&IrTypeUseRange::Input) && shared_condition
        } else {
            self.use_range.contains(&IrTypeUseRange::Input) && shared_condition
        }
    }
    pub fn is_output(&self, exclude_shared: bool, safe_ident: bool) -> bool {
        let shared_condition = !exclude_shared || !self.is_shared(safe_ident);
        if safe_ident {
            self.safe_ident_use_range.contains(&IrTypeUseRange::Output) && shared_condition
        } else {
            self.use_range.contains(&IrTypeUseRange::Output) && shared_condition
        }
    }

    pub(crate) fn remove_use_range(&mut self, removed_use_range: IrTypeUseRange, safe_ident: bool) {
        if safe_ident {
            self.safe_ident_use_range.remove(&removed_use_range);
        } else {
            self.use_range.remove(&removed_use_range);
        }
    }
}
