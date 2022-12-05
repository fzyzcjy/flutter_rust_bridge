/*
    Things this doesn't currently support that it might need to later:

    - Import parsing is unfinished and so is currently disabled
    - When import parsing is enabled:
        - Import renames (use a::b as c) - these are silently ignored
        - Imports that start with two colons (use ::a::b) - these are also silently ignored
*/

use std::{
    cmp::Ordering,
    collections::HashMap,
    fmt::Debug,
    fs,
    path::{Path, PathBuf},
};

use cargo_metadata::MetadataCommand;
use log::{debug, warn};
use syn::{
    Attribute, Ident, ItemEnum, ItemStruct, PathArguments, TraitBound, Type, TypeImplTrait, UseTree,
};

use crate::markers;

/// Represents a crate, including a map of its modules, imports, structs and
/// enums.
#[derive(Debug, Clone)]
pub struct Crate {
    pub name: String,
    pub manifest_path: PathBuf,
    pub root_src_file: PathBuf,
    pub root_module: Module,
}

impl Crate {
    pub fn pre_new(manifest_path: &str) -> (String, PathBuf) {
        let mut cmd = MetadataCommand::new();
        cmd.manifest_path(manifest_path);

        let metadata = cmd.exec().unwrap();

        let root_package = metadata.root_package().unwrap();
        let root_src_file = {
            let lib_file = root_package
                .manifest_path
                .parent()
                .unwrap()
                .join("src/lib.rs");
            let main_file = root_package
                .manifest_path
                .parent()
                .unwrap()
                .join("src/main.rs");

            if lib_file.exists() {
                fs::canonicalize(lib_file).unwrap()
            } else if main_file.exists() {
                fs::canonicalize(main_file).unwrap()
            } else {
                panic!("No src/lib.rs or src/main.rs found for this Cargo.toml file");
            }
        };
        (root_package.name.to_owned(), root_src_file)
    }
    pub fn new(manifest_path: &str) -> Self {
        let mut result = Crate::new_withoud_resolve(manifest_path);
        result.resolve();
        result
    }
    pub fn new_withoud_resolve(manifest_path: &str) -> Self {
        let (name, root_src_file) = Crate::pre_new(manifest_path);

        let source_rust_content = fs::read_to_string(&root_src_file).unwrap();
        let file_ast = syn::parse_file(&source_rust_content).unwrap();

        let result = Crate {
            name: name,
            manifest_path: fs::canonicalize(manifest_path).unwrap(),
            root_src_file: root_src_file.clone(),
            root_module: Module {
                visibility: Visibility::Public,
                file_path: root_src_file,
                module_path: vec!["crate".to_string()],
                source: Some(ModuleSource::File(file_ast)),
                scope: None,
            },
        };

        result
    }

    /// Create a map of the modules for this crate
    pub fn resolve(&mut self) {
        self.root_module.resolve();
    }
}

/// Mirrors syn::Visibility, but can be created without a token
#[derive(Debug, Clone)]
pub enum Visibility {
    Public,
    Crate,
    Restricted, // Not supported
    Inherited,  // Usually means private
}

fn syn_vis_to_visibility(vis: &syn::Visibility) -> Visibility {
    match vis {
        syn::Visibility::Public(_) => Visibility::Public,
        syn::Visibility::Crate(_) => Visibility::Crate,
        syn::Visibility::Restricted(_) => Visibility::Restricted,
        syn::Visibility::Inherited => Visibility::Inherited,
    }
}

#[derive(Debug, Clone)]
pub struct Import {
    pub path: Vec<String>,
    pub visibility: Visibility,
}

#[derive(Debug, Clone)]
pub enum ModuleSource {
    File(syn::File),
    ModuleInFile(Vec<syn::Item>),
}

#[derive(Clone)]
pub struct Struct {
    pub ident: Ident,
    pub src: ItemStruct,
    pub visibility: Visibility,
    pub path: Vec<String>,
    pub mirror: bool,
}

impl Debug for Struct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Struct")
            .field("ident", &self.ident)
            .field("src", &"omitted")
            .field("visibility", &self.visibility)
            .field("path", &self.path)
            .field("mirror", &self.mirror)
            .finish()
    }
}

#[derive(Clone)]
pub struct Enum {
    pub ident: Ident,
    pub src: ItemEnum,
    pub visibility: Visibility,
    pub path: Vec<String>,
    pub mirror: bool,
}

impl Debug for Enum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Enum")
            .field("ident", &self.ident)
            .field("src", &"omitted")
            .field("visibility", &self.visibility)
            .field("path", &self.path)
            .field("mirror", &self.mirror)
            .finish()
    }
}

#[derive(Debug, Clone)]
pub struct ImplTrait {
    pub ident: Ident,
    pub src: TypeImplTrait,
    pub bound: TraitBound,
}
#[derive(Debug, Clone, Eq)]
pub struct Impl {
    pub self_ty: Ident,
    pub trait_: Ident,
}

impl Ord for Impl {
    fn cmp(&self, other: &Impl) -> Ordering {
        self.self_ty.cmp(&other.self_ty)
    }
}

impl PartialOrd for Impl {
    fn partial_cmp(&self, other: &Impl) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Impl {
    fn eq(&self, other: &Impl) -> bool {
        self.self_ty == other.self_ty
    }
}

#[derive(Debug, Clone)]
pub struct ModuleScope {
    pub modules: Vec<Module>,
    pub enums: Vec<Enum>,
    pub impls: Vec<Impl>,
    pub structs: Vec<Struct>,
    pub imports: Vec<Import>,
}

#[derive(Clone)]
pub struct Module {
    pub visibility: Visibility,
    pub file_path: PathBuf,
    pub module_path: Vec<String>,
    pub source: Option<ModuleSource>,
    pub scope: Option<ModuleScope>,
}

impl Debug for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Module")
            .field("visibility", &self.visibility)
            .field("module_path", &self.module_path)
            .field("file_path", &self.file_path)
            .field("source", &"omitted")
            .field("scope", &self.scope)
            .finish()
    }
}

/// Get a struct or enum ident, possibly remapped by a mirror marker
fn get_ident(ident: &Ident, attrs: &[Attribute]) -> (Vec<Ident>, bool) {
    let res = markers::extract_mirror_marker(attrs)
        .into_iter()
        .filter_map(|path| {
            // eq: path.get_ident().map(Clone::clone)
            if path.leading_colon.is_none()
                && path.segments.len() == 1
                && path.segments[0].arguments == PathArguments::None
            {
                Some(path.segments.into_iter().next().unwrap().ident)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let mirror = !res.is_empty();
    if mirror {
        (res, mirror)
    } else {
        (vec![ident.clone()], mirror)
    }
}

fn try_get_module_file_path(
    folder_path: &Path,
    module_name: &str,
    tried: &mut Vec<PathBuf>,
) -> Option<PathBuf> {
    let file_path = folder_path.join(module_name).with_extension("rs");
    if file_path.exists() {
        return Some(file_path);
    }
    tried.push(file_path);

    let file_path = folder_path.join(module_name).join("mod.rs");
    if file_path.exists() {
        return Some(file_path);
    }
    tried.push(file_path);

    None
}

fn get_module_file_path(
    module_name: String,
    parent_module_file_path: &Path,
) -> Result<PathBuf, Vec<PathBuf>> {
    let mut tried = Vec::new();

    if let Some(file_path) = try_get_module_file_path(
        parent_module_file_path.parent().unwrap(),
        &module_name,
        &mut tried,
    ) {
        return Ok(file_path);
    }
    if let Some(file_path) = try_get_module_file_path(
        &parent_module_file_path.with_extension(""),
        &module_name,
        &mut tried,
    ) {
        return Ok(file_path);
    }
    Err(tried)
}

impl Module {
    pub fn resolve(&mut self) {
        self.resolve_modules();
        // self.resolve_imports();
    }

    /// Maps out modules, structs and enums within the scope of this module
    fn resolve_modules(&mut self) {
        let mut scope_modules = Vec::new();
        let mut scope_structs = Vec::new();
        let mut scope_enums = Vec::new();
        let mut scope_impls = Vec::new();

        let items = match self.source.as_ref().unwrap() {
            ModuleSource::File(file) => &file.items,
            ModuleSource::ModuleInFile(items) => items,
        };

        for item in items.iter() {
            match item {
                syn::Item::Struct(item_struct) => {
                    let (idents, mirror) = get_ident(&item_struct.ident, &item_struct.attrs);

                    scope_impls.extend(get_impl_trait_from_attrs(
                        &item_struct.ident,
                        &item_struct.attrs,
                    ));
                    scope_structs.extend(idents.into_iter().map(|ident| {
                        let ident_str = ident.to_string();
                        Struct {
                            ident,
                            src: item_struct.clone(),
                            visibility: syn_vis_to_visibility(&item_struct.vis),
                            path: {
                                let mut path = self.module_path.clone();
                                path.push(ident_str);
                                path
                            },
                            mirror,
                        }
                    }));
                }
                syn::Item::Enum(item_enum) => {
                    let (idents, mirror) = get_ident(&item_enum.ident, &item_enum.attrs);

                    scope_enums.extend(idents.into_iter().map(|ident| {
                        let ident_str = ident.to_string();
                        Enum {
                            ident,
                            src: item_enum.clone(),
                            visibility: syn_vis_to_visibility(&item_enum.vis),
                            path: {
                                let mut path = self.module_path.clone();
                                path.push(ident_str);
                                path
                            },
                            mirror,
                        }
                    }));
                }
                syn::Item::Impl(item_impl) => {
                    if let Some((_b, ref path, _f)) = item_impl.trait_ {
                        // To rule out segments[0].arguments have values
                        if let Some(trait_) = path.get_ident() {
                            if let Type::Path(ref type_path) = *(item_impl.self_ty) {
                                scope_impls.push(Impl {
                                    self_ty: type_path.path.get_ident().unwrap().to_owned(),
                                    trait_: trait_.to_owned(),
                                })
                            }
                        }
                    }
                }
                syn::Item::Mod(item_mod) => {
                    let ident = item_mod.ident.clone();

                    let mut module_path = self.module_path.clone();
                    module_path.push(ident.to_string());

                    scope_modules.push(match &item_mod.content {
                        Some(content) => {
                            let mut child_module = Module {
                                visibility: syn_vis_to_visibility(&item_mod.vis),
                                file_path: self.file_path.clone(),
                                module_path,
                                source: Some(ModuleSource::ModuleInFile(content.1.clone())),
                                scope: None,
                            };

                            child_module.resolve();

                            child_module
                        }
                        None => {
                            let file_path =
                                get_module_file_path(ident.to_string(), &self.file_path);

                            match file_path {
                                Ok(file_path) => {
                                    let source = {
                                        let source_rust_content =
                                            fs::read_to_string(&file_path).unwrap();
                                        debug!("Trying to parse {:?}", file_path);
                                        Some(ModuleSource::File(
                                            syn::parse_file(&source_rust_content).unwrap(),
                                        ))
                                    };
                                    let mut child_module = Module {
                                        visibility: syn_vis_to_visibility(&item_mod.vis),
                                        file_path,
                                        module_path,
                                        source,
                                        scope: None,
                                    };

                                    child_module.resolve();
                                    child_module
                                }
                                Err(tried) => {
                                    warn!(
                                        "Skipping unresolvable module {} (tried {})",
                                        &ident,
                                        tried
                                            .into_iter()
                                            .map(|it| it.to_string_lossy().to_string())
                                            .fold(String::new(), |mut a, b| {
                                                a.push_str(&b);
                                                a.push_str(", ");
                                                a
                                            })
                                    );
                                    continue;
                                }
                            }
                        }
                    });
                }
                _ => {}
            }
        }

        self.scope = Some(ModuleScope {
            modules: scope_modules,
            enums: scope_enums,
            structs: scope_structs,
            impls: scope_impls,
            imports: vec![], // Will be filled in by resolve_imports()
        });
    }

    #[allow(dead_code)]
    fn resolve_imports(&mut self) {
        let imports = &mut self.scope.as_mut().unwrap().imports;

        let items = match self.source.as_ref().unwrap() {
            ModuleSource::File(file) => &file.items,
            ModuleSource::ModuleInFile(items) => items,
        };

        for item in items.iter() {
            if let syn::Item::Use(item_use) = item {
                let flattened_imports = flatten_use_tree(&item_use.tree);

                for import in flattened_imports {
                    imports.push(Import {
                        path: import,
                        visibility: syn_vis_to_visibility(&item_use.vis),
                    });
                }
            }
        }
    }

    pub fn collect_structs<'a>(&'a self, container: &mut HashMap<String, &'a Struct>) {
        let scope = self.scope.as_ref().unwrap();
        for scope_struct in &scope.structs {
            container.insert(scope_struct.ident.to_string(), scope_struct);
        }
        for scope_module in &scope.modules {
            scope_module.collect_structs(container);
        }
    }

    pub fn collect_structs_to_vec(&self) -> HashMap<String, &Struct> {
        let mut ans = HashMap::new();
        self.collect_structs(&mut ans);
        ans
    }

    pub fn collect_enums<'a>(&'a self, container: &mut HashMap<String, &'a Enum>) {
        let scope = self.scope.as_ref().unwrap();
        for scope_enum in &scope.enums {
            container.insert(scope_enum.ident.to_string(), scope_enum);
        }
        for scope_module in &scope.modules {
            scope_module.collect_enums(container);
        }
    }

    pub fn collect_enums_to_vec(&self) -> HashMap<String, &Enum> {
        let mut ans = HashMap::new();
        self.collect_enums(&mut ans);
        ans
    }

    pub fn collect_impls<'a>(&'a self, container: &mut HashMap<String, Vec<Impl>>) {
        let scope = self.scope.as_ref().unwrap();
        for scope_impl in &scope.impls {
            let k = scope_impl.trait_.to_string();
            let v = container.get_mut(&k);
            if let Some(l) = v {
                l.push(scope_impl.to_owned());
            } else {
                container.insert(k, vec![scope_impl.to_owned()]);
            }
        }
        for scope_module in &scope.modules {
            scope_module.collect_impls(container);
        }
    }
    pub fn collect_impls_to_vec(&self) -> HashMap<String, Vec<Impl>> {
        let mut ans = HashMap::new();
        self.collect_impls(&mut ans);
        for (_, v) in ans.iter_mut() {
            v.sort();
        }
        ans
    }
}

fn flatten_use_tree_rename_abort_warning(use_tree: &UseTree) {
    debug!("WARNING: flatten_use_tree() found an import rename (use a::b as c). flatten_use_tree() will now abort.");
    debug!("WARNING: This happened while parsing {:?}", use_tree);
    debug!("WARNING: This use statement will be ignored.");
}

/// Takes a use tree and returns a flat list of use paths (list of string tokens)
///
/// Example:
///     use a::{b::c, d::e};
/// becomes
///     [
///         ["a", "b", "c"],
///         ["a", "d", "e"]
///     ]
///
/// Warning: As of writing, import renames (import a::b as c) are silently
/// ignored.
fn flatten_use_tree(use_tree: &UseTree) -> Vec<Vec<String>> {
    // Vec<(path, is_complete)>
    let mut result = vec![(vec![], false)];

    let mut counter: usize = 0;

    loop {
        counter += 1;

        if counter > 10000 {
            panic!("flatten_use_tree: Use statement complexity limit exceeded. This is probably a bug.");
        }

        // If all paths are complete, break from the loop
        if result.iter().all(|result_item| result_item.1) {
            break;
        }

        let mut items_to_push = Vec::new();

        for path_tuple in &mut result {
            let path = &mut path_tuple.0;
            let is_complete = &mut path_tuple.1;

            if *is_complete {
                continue;
            }

            let mut tree_cursor = use_tree;

            for path_item in path.iter() {
                match tree_cursor {
                    UseTree::Path(use_path) => {
                        let ident = use_path.ident.to_string();
                        if *path_item != ident {
                            panic!("This ident did not match the one we already collected. This is a bug.");
                        }
                        tree_cursor = use_path.tree.as_ref();
                    }
                    UseTree::Group(use_group) => {
                        let mut moved_tree_cursor = false;

                        for tree in use_group.items.iter() {
                            match tree {
                                UseTree::Path(use_path) => {
                                    if path_item == &use_path.ident.to_string() {
                                        tree_cursor = use_path.tree.as_ref();
                                        moved_tree_cursor = true;
                                        break;
                                    }
                                }
                                // Since we're not matching UseTree::Group here, a::b::{{c}, {d}} might
                                // break. But also why would anybody do that
                                _ => unreachable!(),
                            }
                        }

                        if !moved_tree_cursor {
                            unreachable!();
                        }
                    }
                    _ => unreachable!(),
                }
            }

            match tree_cursor {
                UseTree::Name(use_name) => {
                    path.push(use_name.ident.to_string());
                    *is_complete = true;
                }
                UseTree::Path(use_path) => {
                    path.push(use_path.ident.to_string());
                }
                UseTree::Glob(_) => {
                    path.push("*".to_string());
                    *is_complete = true;
                }
                UseTree::Group(use_group) => {
                    // We'll modify the first one in-place, and make clones for
                    // all subsequent ones
                    let mut first: bool = true;
                    // Capture the path in this state, since we're about to
                    // modify it
                    let path_copy = path.clone();
                    for tree in use_group.items.iter() {
                        let mut new_path_tuple = if first {
                            None
                        } else {
                            let new_path = path_copy.clone();
                            items_to_push.push((new_path, false));
                            Some(items_to_push.iter_mut().last().unwrap())
                        };

                        match tree {
                            UseTree::Path(use_path) => {
                                let ident = use_path.ident.to_string();

                                if first {
                                    path.push(ident);
                                } else {
                                    new_path_tuple.unwrap().0.push(ident);
                                }
                            }
                            UseTree::Name(use_name) => {
                                let ident = use_name.ident.to_string();

                                if first {
                                    path.push(ident);
                                    *is_complete = true;
                                } else {
                                    let path_tuple = new_path_tuple.as_mut().unwrap();
                                    path_tuple.0.push(ident);
                                    path_tuple.1 = true;
                                }
                            }
                            UseTree::Glob(_) => {
                                if first {
                                    path.push("*".to_string());
                                    *is_complete = true;
                                } else {
                                    let path_tuple = new_path_tuple.as_mut().unwrap();
                                    path_tuple.0.push("*".to_string());
                                    path_tuple.1 = true;
                                }
                            }
                            UseTree::Group(_) => {
                                panic!(
                                    "Directly-nested use groups ({}) are not supported by flutter_rust_bridge. Use {} instead.",
                                    "use a::{{b}, c}",
                                    "a::{b, c}"
                                );
                            }
                            // UseTree::Group(_) => panic!(),
                            UseTree::Rename(_) => {
                                flatten_use_tree_rename_abort_warning(use_tree);
                                return vec![];
                            }
                        }

                        first = false;
                    }
                }
                UseTree::Rename(_) => {
                    flatten_use_tree_rename_abort_warning(use_tree);
                    return vec![];
                }
            }
        }

        for item in items_to_push {
            result.push(item);
        }
    }

    result.into_iter().map(|val| val.0).collect()
}

fn get_impl_trait_from_attrs(self_ty: &Ident, attrs: &[Attribute]) -> Vec<Impl> {
    let mut scope_impls = vec![];
    for a in attrs.iter() {
        for tt_a in a.tokens.clone().into_iter() {
            if let quote::__private::TokenTree::Group(g) = tt_a {
                for tt_g in g.stream().into_iter() {
                    if let quote::__private::TokenTree::Ident(trait_) = tt_g {
                        scope_impls.push(Impl {
                            self_ty: self_ty.clone(),
                            trait_,
                        });
                    }
                }
            }
        }
    }
    scope_impls
}
