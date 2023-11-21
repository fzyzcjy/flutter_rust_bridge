use crate::target::Target;
use crate::utils::misc::mod_from_rust_path;
use crate::{generator, ir::*, Opts};
use std::collections::{HashMap, HashSet};

impl IrPack {
    pub fn generate_dart(
        &self,
        config: &Opts,
        wasm_funcs: &[IrFuncDisplay],
    ) -> generator::dart::Output {
        generator::dart::generate(self, config, wasm_funcs)
    }
    /// get all symbols(function names) defined explicitly or implictily
    pub fn get_all_symbols(&self, config: &Opts) -> Vec<String> {
        let generated_rust = self.generate_rust(config);

        generated_rust.extern_func_names.into_iter().collect_vec()
    }
}
