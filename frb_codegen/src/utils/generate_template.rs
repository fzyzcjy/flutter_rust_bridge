use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::fs;
use std::hash::Hash;
use std::iter::FromIterator;
use std::process::Command;

use anyhow::anyhow;

use crate::ir::IrFile;
use crate::ir::IrTypeImplTrait;
use crate::source_graph::{Crate, Impl};
use crate::Opts;

use super::parse_sig_from_lib::{self, CallFn};

pub struct OptArray {
    pub configs: Vec<Opts>,
    pub irs: Vec<IrFile>,
    pub explicit_src_impl_pool: HashMap<String, Vec<Impl>>,
    pub explicit_parsed_impl_traits: HashSet<IrTypeImplTrait>,
    pub bound_oject_pool: HashMap<Vec<String>, HashSet<String>>,
}

impl GenerateSourceTemplateTrait for OptArray {
    fn new_without_resolve(configs: &[crate::Opts]) -> Self {
        OptArray {
            configs: configs.to_owned(),
            irs: Vec::new(),
            explicit_src_impl_pool: HashMap::new(),
            explicit_parsed_impl_traits: HashSet::new(),
            bound_oject_pool: HashMap::new(),
        }
    }
}
impl GenerateDependenciesHook for OptArray {}
impl PropTrait for OptArray {
    fn get_configs(&self) -> &[crate::Opts] {
        &self.configs
    }

    fn get_irs(&self) -> &Vec<IrFile> {
        &self.irs
    }

    fn get_mut_irs(&mut self) -> &mut Vec<IrFile> {
        &mut (self.irs)
    }
}
impl GenerateSources for OptArray {}
impl CollectBoundToStruct for OptArray {
    fn get_mut_explicit_src_impl_pool(&mut self) -> &mut HashMap<String, Vec<Impl>> {
        &mut self.explicit_src_impl_pool
    }

    fn get_mut_explicit_parsed_impl_traits(&mut self) -> &mut HashSet<IrTypeImplTrait> {
        &mut self.explicit_parsed_impl_traits
    }

    fn get_mut_bound_oject_pool(&mut self) -> &mut HashMap<Vec<String>, HashSet<String>> {
        &mut self.bound_oject_pool
    }

    fn get_explicit_src_impl_pool(&self) -> &HashMap<String, Vec<Impl>> {
        &self.explicit_src_impl_pool
    }

    fn get_explicit_parsed_impl_traits(&self) -> &HashSet<IrTypeImplTrait> {
        &self.explicit_parsed_impl_traits
    }

    fn get_bound_oject_pool(&self) -> &HashMap<Vec<String>, HashSet<String>> {
        &self.bound_oject_pool
    }
}
pub trait GenerateSourceTemplateTrait:
    PropTrait + GenerateDependenciesHook + GenerateSources
{
    fn run_impl_trait_enum(&mut self) {
        // remove generate source dependencies
        let root_src_file =
            Crate::new_withoud_resolve(&self.get_configs()[0].manifest_path).root_src_file;
        let root_src_file = root_src_file.to_str().unwrap();

        self.remove_gen_mod(root_src_file);
        for config in self.get_configs().iter() {
            let api_file = config.rust_input_path.clone();
            self.remove_gen_use(api_file);
        }

        let irs = self.collect_irs();
        self.get_mut_irs().extend(irs);
        self.run_collect_bound_to_struct();
        if !self.get_bound_oject_pool().is_empty() {
            self.generate_impl_file();

            // generate source dependencies
            self.gen_mod(root_src_file);
            for config in self.get_configs().iter() {
                let api_file = config.rust_input_path.clone();
                self.gen_use(api_file);
            }

            let irs = self.collect_irs();
            self.get_mut_irs().clear();
            self.get_mut_irs().extend(irs);
        }
    }

    fn new_without_resolve(configs: &[crate::Opts]) -> Self;
}
pub trait PropTrait {
    fn get_configs(&self) -> &[crate::Opts];
    fn get_api_paths(&self) -> HashSet<String> {
        let mut explicit_api_path: HashSet<String> = HashSet::new();
        for config in self.get_configs().iter() {
            explicit_api_path.insert(
                config
                    .rust_input_path
                    .split('/')
                    .last()
                    .map(|s| s.split('.').next())
                    .unwrap()
                    .unwrap()
                    .to_owned(),
            );
        }
        explicit_api_path
    }
    fn get_sig_from_doc(&self) -> HashMap<String, CallFn> {
        let config = &self.get_configs()[0];
        let root_src_file = Crate::new_withoud_resolve(config.manifest_path.as_str()).root_src_file;
        let source_rust_content = fs::read_to_string(&root_src_file).unwrap();
        parse_sig_from_lib::parse_file(&source_rust_content)
    }
    fn get_irs(&self) -> &Vec<IrFile>;
    fn get_mut_irs(&mut self) -> &mut Vec<IrFile>;
    fn collect_irs(&self) -> Vec<IrFile> {
        self.get_configs()
            .iter()
            .map(|config| config.get_ir_file())
            .collect()
    }
    /// check api defined by users, if no duplicates, then generate all symbols (api function name),
    /// including those generated implicitily by frb
    fn get_symbols_if_no_duplicates(&self) -> Result<Vec<String>, anyhow::Error> {
        pub fn find_all_duplicates<T>(iter: &[T]) -> Vec<T>
        where
            T: Eq + Hash + Clone,
        {
            let mut uniq = HashSet::new();
            iter.iter()
                .filter(|x| !uniq.insert(*x))
                .cloned()
                .collect::<Vec<_>>()
        }
        let configs = self.get_configs();
        let mut explicit_raw_symbols = Vec::new();
        let mut all_symbols = Vec::new();
        for (index, raw_ir_file) in self.get_irs().iter().enumerate() {
            explicit_raw_symbols.extend(raw_ir_file.funcs.iter().map(|f| f.name.clone()));
            // for avoiding redundant generation in dart
            all_symbols.extend(raw_ir_file.get_all_symbols(&configs[index]));
        }
        let duplicates = find_all_duplicates(&explicit_raw_symbols);
        if !duplicates.is_empty() {
            let duplicated_symbols = duplicates.join(",");

            let (symbol_str, verb_str) = if duplicates.len() == 1 {
                ("symbol", "has")
            } else {
                ("symbols", "have")
            };
            return Err(anyhow!(
                "{} [{}] {} already been defined",
                symbol_str,
                duplicated_symbols,
                verb_str
            ));
        }

        Ok(all_symbols)
    }
}

pub trait CollectBoundToStruct: PropTrait {
    fn get_mut_explicit_src_impl_pool(&mut self) -> &mut HashMap<String, Vec<Impl>>;
    fn get_mut_explicit_parsed_impl_traits(&mut self) -> &mut HashSet<IrTypeImplTrait>;
    fn get_mut_bound_oject_pool(&mut self) -> &mut HashMap<Vec<String>, HashSet<String>>;
    fn get_explicit_src_impl_pool(&self) -> &HashMap<String, Vec<Impl>>;
    fn get_explicit_parsed_impl_traits(&self) -> &HashSet<IrTypeImplTrait>;
    fn get_bound_oject_pool(&self) -> &HashMap<Vec<String>, HashSet<String>>;
    fn collect_bounds(&mut self) {
        fn collect_impl(
            raw_ir_file: &crate::ir::IrFile,
            explicit_src_impl_pool: &mut HashMap<String, Vec<Impl>>,
        ) {
            // for checking relationship between trait and self_ty with all files
            for impl_ in raw_ir_file.src_impl_pool.iter().clone() {
                if let Some(v) = explicit_src_impl_pool.get_mut(impl_.0) {
                    v.extend(impl_.1.to_owned());
                } else {
                    explicit_src_impl_pool.insert(impl_.0.to_owned(), impl_.1.to_owned());
                }
            }
        }
        fn collect_impl_trait(
            raw_ir_file: &crate::ir::IrFile,
            explicit_parsed_impl_traits: &mut HashSet<IrTypeImplTrait>,
        ) {
            // For getting all trait bound defined in single func
            explicit_parsed_impl_traits.extend(raw_ir_file.parsed_impl_traits.clone());
        }

        let mut explicit_src_impl_pool: HashMap<String, Vec<Impl>> = HashMap::new();
        for raw_ir_file in self.get_irs().iter() {
            collect_impl(raw_ir_file, &mut explicit_src_impl_pool);
        }
        self.get_mut_explicit_src_impl_pool()
            .extend(explicit_src_impl_pool);
        let mut explicit_parsed_impl_traits: HashSet<IrTypeImplTrait> = HashSet::new();
        for raw_ir_file in self.get_irs().iter() {
            collect_impl_trait(raw_ir_file, &mut explicit_parsed_impl_traits)
        }
        self.get_mut_explicit_parsed_impl_traits()
            .extend(explicit_parsed_impl_traits);
    }
    fn collect_bound_oject_pool(&mut self) {
        let explicit_src_impl_pool = self.get_explicit_src_impl_pool();
        let explicit_parsed_impl_traits = self.get_explicit_parsed_impl_traits();
        // get a map from bound to all struct meet
        let mut bound_oject_pool = HashMap::new();
        for type_impl_trait in explicit_parsed_impl_traits.iter() {
            let raw = &type_impl_trait.trait_bounds;

            raw.iter().for_each(|bound| {
                // Check whether the trait bound is capable of being used
                // ~~return None if param unoffical~~
                if !explicit_src_impl_pool.contains_key(bound) {
                    panic!("loss impl {} for some self_ty", bound);
                }
            });

            let sets = raw.iter().map(|bound| {
                let impls = explicit_src_impl_pool.get(bound).unwrap();
                let iter = impls.iter().map(|impl_| impl_.self_ty.to_string());
                HashSet::from_iter(iter)
            });

            let mut iter = sets;

            let intersection_set = iter
                .next()
                .map(|set: HashSet<String>| iter.fold(set, |set1, set2| &set1 & &set2))
                .unwrap();
            bound_oject_pool.insert(raw.clone(), intersection_set);
        }
        self.get_mut_bound_oject_pool().extend(bound_oject_pool);
    }
    fn run_collect_bound_to_struct(&mut self) {
        self.collect_bounds();
        self.collect_bound_oject_pool();
    }
}
pub trait InjectInfoFromDocHook: PropTrait {}
pub trait GenerateDependenciesHook {
    fn remove_gen_mod(&self, file_path: impl Display) {
        Command::new("sh")
            .args([
                "-c",
                format!("sed -i '' '/.*mod .*bridge_generated.*/d' {file_path}").as_str(),
            ])
            .spawn()
            .ok();
    }
    fn remove_gen_use(&self, file_path: impl Display) {
        Command::new("sh")
            .args([
                "-c",
                format!("sed -i '' '/.*use .*bridge_generated_bound.*/d' {file_path}").as_str(),
            ])
            .spawn()
            .ok();
    }
    fn gen_mod(&self, file_path: impl Display) {
        Command::new("sh")
            .args([
                "-c",
                format!("echo 'mod bridge_generated_bound;' >> {file_path}").as_str(),
            ])
            .spawn()
            .ok();
    }

    fn gen_use(&self, file_path: impl Display) {
        Command::new("sh")
            .args([
                "-c",
                format!("echo 'pub use crate::bridge_generated_bound::*;' >> {file_path}").as_str(),
            ])
            .spawn()
            .ok();
    }
}

pub trait GenerateSources: CollectBoundToStruct {
    fn generate_impl_file(&self) {
        let trait_sig_pool = self.get_sig_from_doc();
        let explicit_api_path = self.get_api_paths();
        let bound_oject_pool = self.get_bound_oject_pool();

        let mut lines = String::new();
        for super_ in explicit_api_path.iter() {
            lines += format!("use crate::{super_}::*;").as_str();
        }
        for (_, call_fn) in trait_sig_pool.iter() {
            let impl_dependencies = call_fn.impl_dependencies.clone();
            lines += impl_dependencies.to_string().as_str();
        }
        for (k, v) in bound_oject_pool.iter() {
            lines += format!("pub enum {}Enum {{", k.join("")).as_str();
            for struct_ in v.iter() {
                lines += format!("{}({}),", struct_, struct_).as_str();
            }
            lines += "}".to_string().as_str();
        }

        for (k, v) in bound_oject_pool.iter() {
            let enum_ = format!("{}Enum", k.join(""));
            for trait_ in k.iter() {
                lines += format!("impl {trait_} for {enum_} {{").as_str();
                let call_fn = trait_sig_pool
                    .get(trait_)
                    .unwrap_or_else(|| panic!("Error: {:?} with {:?}", trait_sig_pool, trait_));

                for idx in 0..call_fn.sig.len() {
                    lines += format!("{}{{", call_fn.sig[idx]).as_str();
                    lines += "match *self {".to_string().as_str();
                    for sub_enum in v.iter() {
                        lines += format!(
                            "{enum_}::{sub_enum}(ref __field0) => __field0.{}({}),",
                            call_fn.fn_name[idx], call_fn.args[idx]
                        )
                        .as_str();
                    }
                    lines += "}".to_string().as_str();
                    lines += "}".to_string().as_str();
                }
                lines += "}".to_string().as_str();
            }
        }

        fs::write("src/bridge_generated_bound.rs", lines).unwrap();
    }

    // fn generate_impl_file(&self) {}
}
