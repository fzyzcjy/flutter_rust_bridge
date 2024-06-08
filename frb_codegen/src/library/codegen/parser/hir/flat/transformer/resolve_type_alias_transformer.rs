use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::ir::hir::flat::type_alias::HirFlatTypeAlias;
use crate::codegen::parser::mir::parser::ty::misc::convert_ident_str;
use itertools::Itertools;
use std::collections::HashMap;
use syn::Type;
use topological_sort::TopologicalSort;

pub(crate) fn transform(mut pack: HirFlatPack) -> anyhow::Result<HirFlatPack> {
    let map_raw = (pack.types.iter())
        .map(|x| (x.ident.clone(), x.target.clone()))
        .collect();
    let map_transformed = resolve_type_aliases(map_raw);
    let vec_transformed = (map_transformed.into_iter())
        .map(|(ident, target)| HirFlatTypeAlias { ident, target })
        .collect_vec();

    pack.types = vec_transformed;

    Ok(pack)
}

// See https://github.com/fzyzcjy/flutter_rust_bridge/pull/929 for more details of the algorithm
pub(crate) fn resolve_type_aliases(src: HashMap<String, Type>) -> HashMap<String, Type> {
    // Some types that cannot be Handled.
    // Filter something like `BareFn( TypeBareFn...`
    // Filter something like `Ptr( TypePtr { star_token: Star,`
    let mut ret: HashMap<String, Type> = src
        .iter()
        .filter_map(|(k, v)| match convert_ident_str(v) {
            Some(_) => None,
            None => Some((k.to_owned(), v.to_owned())),
        })
        .collect();

    let string_src = src
        .iter()
        // Filter some types that cannot be Handled.
        .filter_map(|(k, v)| convert_ident_str(v).map(|v| (k, v)));

    let mut ts = TopologicalSort::<String>::new();

    string_src.for_each(|(k, v)| {
        // k and v switch orders here
        ts.add_dependency(v, k.clone());
    });

    // remove base type, like i32 in `type Id = i32`.
    // You might worry about the type, which cannot be handled and isn't in ts.
    // Case:
    // ```
    // type UnsafeAlias = unsafe{};
    // type NestAlias = UnsafeAlias;
    // ```
    // 1. pop_base: ret = [{UnsafeAlias, unsafe}]
    // 2. init_condition:
    //    2.1. ("UnsafeAlias","NestAlias") in ts,
    //    2.2. do pop_all, pop UnsafeAlias, only "NestAlias" in ts.
    //    2.3. do pop, and then handle NestAlias.
    //    src.get("NestAlias") => UnsafeAlias,
    //    ret.insert("NestAlias") = ret.get("UnsafeAlias")
    ts.pop_all();
    // build init_condition
    ts.pop_all().into_iter().for_each(|k| {
        let v_src = src.get(&k).unwrap().to_owned();
        let v_str = convert_ident_str(&v_src).unwrap();

        ret.insert(
            k,
            if ret.contains_key(&v_str) {
                // only happen if v_src cannot handle
                ret.get(&v_str).unwrap().clone()
            } else {
                v_src
            },
        );
    });

    while let Some(k) = ts.pop() {
        let v_src = src.get(&k).unwrap().to_owned();
        let v_str = convert_ident_str(&v_src).unwrap();
        let v_ret = ret
            .get(&v_str)
            .unwrap_or_else(|| panic!("{:?},\n{:?},\n{:?},\n{:?}", src, ts, ret, k))
            .to_owned();
        ret.insert(k, v_ret);
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use syn::{parse_str, Type};

    #[test]
    fn test_topo_resolve_primary_type_with_nest() {
        let input = HashMap::from([
            ("id".to_string(), parse_str::<Type>("i32").unwrap()),
            ("UserId".to_string(), parse_str::<Type>("id").unwrap()),
        ]);
        let expect = HashMap::from([
            ("id".to_string(), parse_str::<Type>("i32").unwrap()),
            ("UserId".to_string(), parse_str::<Type>("i32").unwrap()),
        ]);
        let output = resolve_type_aliases(input);
        assert_eq!(output, expect);
    }

    #[test]
    fn test_topo_resolve_primary_type() {
        let input = HashMap::from([("id".to_string(), parse_str::<Type>("i32").unwrap())]);
        let expect = HashMap::from([("id".to_string(), parse_str::<Type>("i32").unwrap())]);

        let output = resolve_type_aliases(input);
        assert_eq!(output, expect);
    }

    #[test]
    fn test_topo_resolve3_unhandle_case() {
        let input = HashMap::from([("DartPostCObjectFnType".to_string(), parse_str::<Type>(r#"unsafe extern "C" fn(port_id: DartPort, message: *mut std::ffi::c_void) -> bool"#).unwrap())]);
        let expect = HashMap::from([("DartPostCObjectFnType".to_string(), parse_str::<Type>(r#"unsafe extern "C" fn(port_id: DartPort, message: *mut std::ffi::c_void) -> bool"#).unwrap())]);
        let output = resolve_type_aliases(input);
        assert_eq!(output, expect);
    }

    #[test]
    fn test_topo_resolve_unhandle_case_with_nest() {
        let input = HashMap::from([
            ("DartPostCObjectFnType".to_string(), parse_str::<Type>(r#"unsafe extern "C" fn(port_id: DartPort, message: *mut std::ffi::c_void) -> bool"#).unwrap()),
            (
                "DartPostCObjectFnTypeAlias".to_string(),
                parse_str::<Type>("DartPostCObjectFnType").unwrap(),
            )
        ]);
        let expect = HashMap::from([
            ("DartPostCObjectFnType".to_string(), parse_str::<Type>(r#"unsafe extern "C" fn(port_id: DartPort, message: *mut std::ffi::c_void) -> bool"#).unwrap()),
            ("DartPostCObjectFnTypeAlias".to_string(), parse_str::<Type>(r#"unsafe extern "C" fn(port_id: DartPort, message: *mut std::ffi::c_void) -> bool"#).unwrap()),
        ]);
        let output = resolve_type_aliases(input);
        assert_eq!(&output, &expect);
    }
}
