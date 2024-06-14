use crate::codegen::ir::hir::flat::struct_or_enum::HirFlatStructOrEnum;
use crate::codegen::ir::hir::misc::syn_item_struct_or_enum::SynItemStructOrEnum;
use crate::codegen::ir::hir::misc::visibility::HirVisibility;
use crate::codegen::ir::mir::ty::rust_auto_opaque_implicit::MirTypeRustAutoOpaqueImplicitReason;
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::parser::mir::parser::attribute::FrbAttributes;
use crate::codegen::parser::mir::parser::ty::unencodable::SplayedSegment;
use crate::library::codegen::ir::mir::ty::MirTypeTrait;
use crate::utils::basic_code::general_code::GeneralDartCode;
use crate::utils::basic_code::parser::parse_dart_code;
use crate::utils::crate_name::CrateName;
use crate::utils::namespace::{Namespace, NamespacedName};
use log::debug;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use syn::Type;
use crate::codegen::parser::mir::parser::ty::TypeParserParsingContext;

pub(super) trait EnumOrStructParser<Id, Obj, Item: SynItemStructOrEnum>
where
    Id: From<NamespacedName> + Clone + PartialEq + Eq + Hash,
{
    fn parse(
        &mut self,
        last_segment: &SplayedSegment,
        override_opaque: Option<bool>,
    ) -> anyhow::Result<Option<MirType>> {
        let output = self.parse_impl(last_segment, override_opaque)?;
        self.handle_dart_code(&output);
        Ok(output.map(|(ty, _)| ty))
    }

    fn parse_impl(
        &mut self,
        last_segment: &SplayedSegment,
        override_opaque: Option<bool>,
    ) -> anyhow::Result<Option<(MirType, FrbAttributes)>> {
        let (name, _) = last_segment;
        // let name = external_impl::parse_name_or_original(name)?;

        if let Some(src_object) = self.src_objects().get(*name) {
            let src_object = (*src_object).clone();

            let namespace = &src_object.name.namespace;
            let namespaced_name = NamespacedName::new(namespace.clone(), name.to_string());

            let attrs = FrbAttributes::parse(src_object.src.attrs())?;
            let attrs_opaque = override_opaque.or(attrs.opaque());
            if attrs_opaque == Some(true) {
                debug!("Treat {name} as opaque since attribute says so");
                return Ok(Some((
                    self.parse_opaque(&namespaced_name, &src_object)?,
                    attrs,
                )));
            }

            let ident: Id = namespaced_name.clone().into();

            if (self.parser_info().parsing_or_parsed_objects).insert(namespaced_name.clone()) {
                let (name, wrapper_name) = compute_name_and_wrapper_name(
                    &namespaced_name.namespace,
                    &src_object.name.name,
                    src_object.mirror,
                );
                let parsed_object = self.parse_inner_impl(&src_object, name, wrapper_name)?;
                (self.parser_info().object_pool).insert(ident.clone(), parsed_object);
            }

            if attrs_opaque.is_none()
                && (self.parser_info().object_pool.get(&ident))
                    .map_or(false, |obj| Self::compute_default_opaque(obj))
            {
                debug!("Treat {name} as opaque by compute_default_opaque");
                return Ok(Some((
                    self.parse_opaque(&namespaced_name, &src_object)?,
                    attrs,
                )));
            }

            return Ok(Some((self.construct_output(ident)?, attrs)));
        }

        Ok(None)
    }

    fn handle_dart_code(&mut self, raw_output: &Option<(MirType, FrbAttributes)>) {
        if let Some((ty, attrs)) = &raw_output {
            let dart_code = attrs.dart_code();
            if dart_code.is_empty() {
                return;
            }

            let dart_code_typed = parse_dart_code(&dart_code);

            let keys = match ty {
                MirType::RustAutoOpaqueImplicit(ty) => vec![ty.safe_ident(), ty.inner.safe_ident()],
                ty => vec![ty.safe_ident()],
            };

            for key in keys {
                self.dart_code_of_type()
                    .insert(key, dart_code_typed.clone());
            }
        }
    }

    fn parse_opaque(
        &mut self,
        namespaced_name: &NamespacedName,
        src_object: &HirFlatStructOrEnum<Item>,
    ) -> anyhow::Result<MirType> {
        self.parse_type_rust_auto_opaque_implicit(
            Some(namespaced_name.namespace.clone()),
            &syn::parse_str(&namespaced_name.name)?,
            Some(MirTypeRustAutoOpaqueImplicitReason::StructOrEnumRequireOpaque),
            Some(parse_struct_or_enum_should_ignore(
                src_object,
                &namespaced_name.namespace.crate_name(),
                self.context(),
            )),
        )
    }

    fn parse_inner_impl(
        &mut self,
        src_object: &HirFlatStructOrEnum<Item>,
        name: NamespacedName,
        wrapper_name: Option<String>,
    ) -> anyhow::Result<Obj>;

    fn construct_output(&self, ident: Id) -> anyhow::Result<MirType>;

    fn src_objects(&self) -> &HashMap<String, &HirFlatStructOrEnum<Item>>;

    fn parser_info(&mut self) -> &mut EnumOrStructParserInfo<Id, Obj>;

    fn dart_code_of_type(&mut self) -> &mut HashMap<String, GeneralDartCode>;

    fn parse_type_rust_auto_opaque_implicit(
        &mut self,
        namespace: Option<Namespace>,
        ty: &Type,
        reason: Option<MirTypeRustAutoOpaqueImplicitReason>,
        override_ignore: Option<bool>,
    ) -> anyhow::Result<MirType>;

    fn context(&self) -> &TypeParserParsingContext;

    fn compute_default_opaque(obj: &Obj) -> bool;
}

#[derive(Clone, Debug, Default)]
pub(super) struct EnumOrStructParserInfo<Id, Obj> {
    parsing_or_parsed_objects: HashSet<NamespacedName>,
    pub(super) object_pool: HashMap<Id, Obj>,
}

impl<Id, Obj> EnumOrStructParserInfo<Id, Obj> {
    pub fn new() -> Self {
        Self {
            parsing_or_parsed_objects: HashSet::new(),
            object_pool: HashMap::new(),
        }
    }
}

fn compute_name_and_wrapper_name(
    namespace: &Namespace,
    name: &str,
    mirror: bool,
) -> (NamespacedName, Option<String>) {
    let namespaced_name = NamespacedName::new(namespace.clone(), name.to_owned());
    let wrapper_name = if mirror {
        Some(format!("FrbWrapper<{}>", namespaced_name.rust_style()))
    } else {
        None
    };
    (namespaced_name, wrapper_name)
}

pub(crate) fn parse_struct_or_enum_should_ignore<Item: SynItemStructOrEnum>(
    src_object: &HirFlatStructOrEnum<Item>,
    crate_name: &CrateName,
    context: &TypeParserParsingContext,
) -> bool {
    let attrs = FrbAttributes::parse(src_object.src.attrs()).unwrap();

    attrs.ignore()
        // For third party crates, if a struct is not public, then it is impossible to utilize it,
        // thus we ignore it.
        || ((!crate_name.is_self_crate())  && src_object.visibility != HirVisibility::Public)
        // If enable lifetime, even if there is generics, it may be just lifetime generics, so we should not ignore them blindly
        || (!context.enable_lifetime && !src_object.src.generics().params.is_empty())
}
