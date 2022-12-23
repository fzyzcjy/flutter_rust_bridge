use crate::target::Target;
use crate::utils::mod_from_rust_path;
use crate::{generator, ir::*, Opts};
use std::collections::{HashMap, HashSet};

pub type IrStructPool = HashMap<String, IrStruct>;
pub type IrEnumPool = HashMap<String, IrEnum>;

#[derive(Debug, Clone)]
pub struct IrFile {
    pub funcs: Vec<IrFunc>,
    pub struct_pool: IrStructPool,
    pub enum_pool: IrEnumPool,
    pub has_executor: bool,
}

impl IrFile {
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

    pub fn get_c_struct_names(&self) -> Vec<String> {
        let c_struct_names = self
            .distinct_types(true, true)
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

    pub fn distinct_types(
        &self,
        include_func_inputs: bool,
        include_func_output: bool,
    ) -> Vec<IrType> {
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

        // make the output change less when input change
        ans.sort_by_key(|ty| ty.safe_ident());

        ans
    }

    pub fn generate_rust(&self, config: &Opts) -> generator::rust::Output {
        generator::rust::generate(
            self,
            &mod_from_rust_path(&config.rust_input_path, &config.rust_crate_dir),
            config,
        )
    }

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

        generated_rust
            .extern_func_names
            .into_iter()
            .filter(|s| *s != "free_WireSyncReturn")
            .collect::<Vec<_>>()
    }
}
