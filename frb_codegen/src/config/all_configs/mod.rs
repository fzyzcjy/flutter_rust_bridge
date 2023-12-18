mod fetch;
mod parse;
mod sub_type;

use std::collections::HashMap;
use std::ops::Index;

use crate::ir::{IrFile, IrFuncDisplay};

use crate::target::Acc;
use crate::utils::misc::BlockIndex;
use crate::{generator, Opts};

use self::sub_type::{IrFuncContext, IrTypeContext};

#[derive(Debug, Clone)]
pub struct AllConfigs {
    regular_configs: Vec<Opts>,
    shared_config: Option<Opts>,
    regular_ir_files: Vec<IrFile>,
    shared_ir_file: Option<IrFile>,
    funcs_contexts: Vec<IrFuncContext>,
    types_contexts: Vec<IrTypeContext>, // it contains all IrTypes within `IrTypeContext` used among all blocks
    dart_api2wire_funcs_map: HashMap<BlockIndex, Acc<String>>,
}

impl AllConfigs {
    /// TODO: refine this doc
    /// Creates a new `AllConfigs` instance from a slice of `Opts` instances.
    ///
    /// # Arguments
    ///
    /// * `configs`: A slice of `Opts` instances to be included in the `AllConfigs` instance.
    ///
    /// # Panics
    ///
    /// Panics if there are more than one `Opts` instances with `shared` set to `true`.
    ///
    /// # Returns
    ///
    /// A new `AllConfigs` instance with the `Opts` instances sorted so that the one with `shared: true`
    /// comes first, followed by the ones with `shared: false` within the original order.
    /// But the `BlockIndex` of each one would be reset starting from 0 ascendingly according to the new order.
    /// The `is_multi_blocks` field of the instance is set to `true` if there is no `Opts` instance with `shared: true`; or the field is set to `true` otherwise.
    pub fn new(mut raw_configs: Vec<Opts>) -> Self {
        // pre-check
        let regular_configs = raw_configs
            .iter()
            .filter(|c| !c.shared)
            .cloned()
            .collect::<Vec<_>>();
        // Get shared config if there is one
        let shared_config = if regular_configs.len() > 1 {
            let shared_configs = raw_configs
                .iter()
                .filter(|c| c.shared)
                .cloned()
                .collect::<Vec<_>>();
            assert!(
                shared_configs.len() == 1,
                "There should be 1 shared config for multi-blocks case"
            );
            Some(shared_configs[0].to_owned())
        } else {
            let shared_configs = raw_configs.iter().filter(|c| c.shared).collect::<Vec<_>>();
            assert!(
                shared_configs.is_empty(),
                "There should be 0 shared config for single-block case "
            );
            None
        };
        // put shared config at very first, and leave others no changed
        raw_configs.sort_by_key(|c| !c.shared);
        // create the instance
        let mut all_configs = AllConfigs {
            regular_configs,
            shared_config,
            // the below fields will be filled later
            regular_ir_files: vec![],
            shared_ir_file: None,
            funcs_contexts: vec![],
            types_contexts: vec![],
            dart_api2wire_funcs_map: HashMap::new(),
        };
        // check again with instance `all_configs`
        if all_configs.get_shared_config().is_some() {
            assert!(
                all_configs.get_regular_configs().len() > 1,
                "There is only one or no regular config, but there is a shared config"
            );
            true
        } else {
            assert!(
                all_configs.get_regular_configs().len() == 1,
                "There are more than one regular configs, but no shared config"
            );
            false
        };
        // Do the dirty parse job in advance
        all_configs.parse();
        // return
        all_configs
    }

    pub fn is_multi_blocks_case(&self) -> bool {
        self.regular_configs.len() > 1 && self.shared_config.is_some()
    }

    /// iterate withe the shared config at first, then the regular configs
    /// Note: This method can only be used for multi-blocks case
    pub fn iter_all(&self) -> impl Iterator<Item = &Opts> {
        assert!(self.is_multi_blocks_case());
        let shared_config = self.shared_config.as_ref().unwrap();
        std::iter::once(shared_config).chain(self.regular_configs.iter())
    }

    pub fn generate_rust(&self, config: &Opts) -> generator::rust::Output {
        generator::rust::generate(config, self)
    }

    pub fn generate_dart(
        &self,
        config: &Opts,
        wasm_funcs: &[IrFuncDisplay],
    ) -> generator::dart::Output {
        generator::dart::generate(config, self, wasm_funcs)
    }
}
// TODO: delete
// impl<'a> IntoIterator for &'a AllConfigs {
//     type Item = &'a Opts;
//     type IntoIter = std::slice::Iter<'a, Opts>;

//     fn into_iter(self) -> Self::IntoIter {
//         self.regular_configs.iter()
//     }
// }

// impl IntoIterator for AllConfigs {
//     type Item = Opts;
//     type IntoIter = std::vec::IntoIter<Opts>;

//     fn into_iter(self) -> Self::IntoIter {
//         self.regular_configs.into_iter()
//     }
// }

impl Index<usize> for AllConfigs {
    type Output = Opts;
    fn index(&self, index: usize) -> &Self::Output {
        &self.regular_configs[index]
    }
}

// TODO: refine test
// #[cfg(test)]
// mod tests {
//     use crate::utils::misc::BlockIndex;

//     use super::*;

//     /// Get a configs for tests, which includes
//     /// 2 regular configs and 1 shared config.
//     /// Note: the order of the output list is [regular_2, regular_1, shared],
//     /// which is made specifically for the test cases.
//     fn get_shared_sample_configs() -> Vec<Opts> {
//         let configs = vec![
//             get_one_sample_config(2, false),
//             get_one_sample_config(1, false),
//             get_one_sample_config(3, true),
//         ];
//         configs
//     }

//     /// only 1 regular config is initialized
//     fn get_one_regular_sample_config() -> Vec<Opts> {
//         let configs = vec![get_one_sample_config(1, false)];
//         configs
//     }

//     fn get_one_sample_config(index: usize, shared: bool) -> Opts {
//         let shared_input = if shared {
//             "rust_input_share".into()
//         } else {
//             let relative_path =
//                 format!("../frb_example/pure_dart_multi/rust/src/api_block_{index}.rs");
//             let full_path = std::env::current_dir().unwrap().join(relative_path);
//             full_path.into_os_string().into_string().unwrap()
//         };
//         Opts {
//             rust_input_path: shared_input,
//             dart_output_path: "".into(),
//             dart_decl_output_path: None,
//             c_output_paths: vec![],
//             rust_crate_dir: "".into(),
//             rust_output_path: "".into(),
//             class_name: "".into(),
//             dart_format_line_length: 98,
//             dart_enums_style: true,
//             skip_add_mod_to_lib: true,
//             llvm_path: vec![],
//             llvm_compiler_opts: "".into(),
//             manifest_path: "".into(),
//             dart_root: None,
//             build_runner: true,
//             block_index: BlockIndex(0),
//             skip_deps_check: false,
//             wasm_enabled: false,
//             inline_rust: true,
//             bridge_in_method: false,
//             extra_headers: "".into(),
//             dart3: true,
//             keep_going: true,
//             shared,
//         }
//     }

//     #[test]
//     fn test_all_configs() {
//         let configs = get_shared_sample_configs();
//         let all_configs = AllConfigs::new(configs);

//         assert_eq!(
//             all_configs.get_shared_config().unwrap().rust_input_path,
//             "rust_input_share"
//         );
//         assert_eq!(all_configs.get_regular_configs().len(), 2);
//         assert_eq!(all_configs.iter().count(), 3);

//         let mut iter = all_configs.iter();
//         assert_eq!(iter.next().unwrap().rust_input_path, "rust_input_share");
//         assert_eq!(iter.next().unwrap().rust_input_path, "rust_input_2");
//         assert_eq!(iter.next().unwrap().rust_input_path, "rust_input_1");
//         assert_eq!(iter.next(), None);

//         let mut iter = all_configs.clone().into_iter();
//         assert_eq!(iter.next().unwrap().rust_input_path, "rust_input_share");
//         assert_eq!(iter.next().unwrap().rust_input_path, "rust_input_2");
//         assert_eq!(iter.next().unwrap().rust_input_path, "rust_input_1");
//         assert_eq!(iter.next(), None);
//     }

//     #[test]
//     fn test_all_configs_enumerate() {
//         let configs = get_shared_sample_configs();
//         let all_configs = AllConfigs::new(configs.clone());

//         let mut iter = all_configs.iter().enumerate();
//         assert_eq!(iter.next().unwrap(), (0, &configs[2]));
//         assert_eq!(iter.next().unwrap(), (1, &configs[0]));
//         assert_eq!(iter.next().unwrap(), (2, &configs[1]));
//         assert_eq!(iter.next(), None);

//         let mut iter = all_configs.clone().into_iter().enumerate();
//         assert_eq!(iter.next().unwrap(), (0, configs[2].clone()));
//         assert_eq!(iter.next().unwrap(), (1, configs[0].clone()));
//         assert_eq!(iter.next().unwrap(), (2, configs[1].clone()));
//         assert_eq!(iter.next(), None);
//     }

//     #[test]
//     fn test_regular_configs() {
//         let configs = get_one_regular_sample_config();
//         let all_configs = AllConfigs::new(configs);
//         let regular_configs = all_configs.get_regular_configs();

//         assert_eq!(regular_configs.len(), 1);
//         assert_eq!(regular_configs[0].rust_input_path, "rust_input_1");
//         assert!(all_configs.get_shared_config().is_none());
//         assert!(!all_configs.is_multi_blocks_case());
//     }

//     #[test]
//     fn test_shared_configs() {
//         let configs = get_shared_sample_configs();
//         let all_configs = AllConfigs::new(configs);

//         let regular_configs = all_configs.get_regular_configs();
//         assert_eq!(regular_configs.len(), 2);
//         assert_eq!(regular_configs[0].rust_input_path, "rust_input_2");
//         assert_eq!(regular_configs[1].rust_input_path, "rust_input_1");

//         assert!(all_configs.is_multi_blocks_case());
//     }

//     #[test]
//     #[should_panic(expected = "Only one shared config is allowed")]
//     fn test_panic_because_of_multi_shared_configs() {
//         let configs = vec![
//             get_one_sample_config(2, true),
//             get_one_sample_config(1, true),
//             get_one_sample_config(3, true),
//         ];
//         AllConfigs::new(configs);
//     }

//     #[test]
//     #[should_panic(expected = "There are more than one regular configs, but no shared config")]
//     fn test_panic_because_of_multi_regular_configs_but_no_shared_config() {
//         let configs = vec![
//             get_one_sample_config(2, false),
//             get_one_sample_config(1, false),
//         ];
//         AllConfigs::new(configs);
//     }

//     #[test]
//     #[should_panic(
//         expected = "There is only one or no regular config, but there is a shared config"
//     )]
//     fn test_panic_because_of_a_shared_config_with_no_more_than_one_regular_config() {
//         let configs = vec![get_one_sample_config(1, true)];
//         AllConfigs::new(configs);
//     }
// }
