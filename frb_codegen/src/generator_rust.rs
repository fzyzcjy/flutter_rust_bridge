use crate::api_types::ApiType::*;
use crate::api_types::*;
use crate::generator_common::*;

pub fn generate(api_file: &ApiFile, rust_wire_stem: &str) -> String {
    let wire_funcs = api_file
        .funcs
        .iter()
        .map(generate_wire_func)
        .collect::<Vec<_>>();
    let wire_structs = api_file
        .distinct_types()
        .iter()
        .map(|ty| generate_wire_struct(ty, api_file))
        .collect::<Vec<_>>();
    let allocate_funcs = api_file
        .distinct_types()
        .iter()
        .map(generate_allocate_funcs)
        .collect::<Vec<_>>();
    let wire2api_funcs = api_file
        .distinct_types()
        .iter()
        .map(|ty| generate_wire2api_func(ty, api_file))
        .collect::<Vec<_>>();
    let new_with_nullptr_funcs = api_file
        .distinct_types()
        .iter()
        .map(|ty| generate_new_with_nullptr_func(ty, api_file))
        .collect::<Vec<_>>();
    let impl_intodart = api_file
        .distinct_types()
        .iter()
        .map(|ty| generate_impl_intodart(ty, api_file))
        .collect::<Vec<_>>();

    format!(
        "#![allow(non_camel_case_types, clippy::redundant_closure, clippy::useless_conversion)]
        {}

        use crate::{}::*;
        use flutter_rust_bridge::support;

        // Section: wire functions

        {}

        // Section: wire structs

        {}

        // Section: allocate functions

        {}

        // Section: impl Wire2Api

        pub trait Wire2Api<T> {{
            fn wire2api(self) -> T;
        }}

        {}

        // Section: impl NewWithNullPtr

        pub trait NewWithNullPtr {{
            fn new_with_null_ptr() -> Self;
        }}

        {}

        // Section: impl IntoDart 
        {}
        ",
        CODE_HEADER,
        rust_wire_stem,
        wire_funcs.join("\n\n"),
        wire_structs.join("\n\n"),
        allocate_funcs.join("\n\n"),
        wire2api_funcs.join("\n\n"),
        new_with_nullptr_funcs.join("\n\n"),
        impl_intodart.join("\n\n"),
    )
}

fn generate_wire_func(func: &ApiFunc) -> String {
    // println!("generate_wire_func: {}", func.name);
    format!(
        "#[no_mangle]
        pub extern \"C\" fn {}(port: i64, {}) {{
            {}
            support::wrap_wire_func(port, move || {}({}));
        }}",
        func.wire_func_name(),
        func.inputs
            .iter()
            .map(|field| format!(
                "{}: {}{}",
                field.name.rust_style(),
                field.ty.rust_wire_modifier(),
                field.ty.rust_wire_type()
            ))
            .collect::<Vec<_>>()
            .join(", "),
        func.inputs
            .iter()
            .map(|field| format!(
                "let api_{} = {}.wire2api();",
                field.name.rust_style(),
                field.name.rust_style()
            ))
            .collect::<Vec<_>>()
            .join(""),
        func.name,
        func.inputs
            .iter()
            .map(|field| format!("api_{}", field.name.rust_style()))
            .collect::<Vec<_>>()
            .join(", "),
    )
}

fn generate_wire_struct(ty: &ApiType, api_file: &ApiFile) -> String {
    // println!("generate_wire_struct: {:?}", ty);
    let fields = match ty {
        PrimitiveList(list) => vec![
            format!("ptr: *mut {}", list.primitive.rust_wire_type()),
            "len: i32".to_string(),
        ],
        GeneralList(list) => vec![
            format!("ptr: *mut {}", list.inner.rust_wire_type()),
            "len: i32".to_string(),
        ],
        StructRef(s) => s
            .get(api_file)
            .fields
            .iter()
            .map(|field| {
                format!(
                    "{}: {}{}",
                    field.name.rust_style(),
                    field.ty.rust_wire_modifier(),
                    field.ty.rust_wire_type()
                )
            })
            .collect(),
        Primitive(_) | Delegate(_) | Boxed(_) => return "".to_string(),
    };

    format!(
        r###"
        #[repr(C)]
        #[derive(Clone)]
        pub struct {} {{
            {}
        }}
        "###,
        ty.rust_wire_type(),
        fields.join(",\n"),
    )
}

fn generate_allocate_funcs(ty: &ApiType) -> String {
    // println!("generate_allocate_funcs: {:?}", ty);
    match ty {
        Primitive(_) => "".to_string(),
        Delegate(_) => "".to_string(),
        PrimitiveList(list) => format!(
            r###"
            #[no_mangle]
            pub extern "C" fn new_{}(len: i32) -> {}{} {{
                let ans = {} {{ ptr: support::new_leak_vec_ptr(Default::default(), len), len }};
                support::new_leak_box_ptr(ans)
            }}
            "###,
            list.safe_ident(),
            list.rust_wire_modifier(),
            list.rust_wire_type(),
            list.rust_wire_type(),
        ),
        GeneralList(list) => format!(
            r###"
            #[no_mangle]
            pub extern "C" fn new_{}(len: i32) -> {}{} {{
                let wrap = {} {{ ptr: support::new_leak_vec_ptr({}::new_with_null_ptr(), len), len }};
                support::new_leak_box_ptr(wrap)
            }}
            "###,
            ty.safe_ident(),
            list.rust_wire_modifier(),
            list.rust_wire_type(),
            list.rust_wire_type(),
            list.inner.rust_wire_type(),
        ),
        StructRef(_) => "".to_string(),
        Boxed(b) => format!(
            r###"
            #[no_mangle]
            pub extern "C" fn new_{}() -> {}{} {{
                support::new_leak_box_ptr({}::new_with_null_ptr())
            }}
            "###,
            ty.safe_ident(),
            ty.rust_wire_modifier(),
            ty.rust_wire_type(),
            b.inner.rust_wire_type(),
        ),
    }
}

fn generate_wire2api_func(ty: &ApiType, api_file: &ApiFile) -> String {
    // println!("generate_wire2api_func: {:?}", ty);
    let body = match ty {
        Primitive(_) => "self".to_string(),
        Delegate(d) => match d {
            ApiTypeDelegate::String => "let vec: Vec<u8> = self.wire2api();
                String::from_utf8_lossy(&vec).into_owned()"
                .to_string(),
        },
        PrimitiveList(_) => "unsafe {
                let wrap = support::box_from_leak_ptr(self);
                support::vec_from_leak_ptr(wrap.ptr, wrap.len)
            }
            "
        .to_string(),
        GeneralList(_) => "let vec = unsafe {
                let wrap = support::box_from_leak_ptr(self);
                support::vec_from_leak_ptr(wrap.ptr, wrap.len)
            };
            vec.into_iter().map(|x| x.wire2api()).collect()
            "
        .to_string(),
        Boxed(_) => "let wrap = unsafe { support::box_from_leak_ptr(self) };
            (*wrap).wire2api().into()"
            .to_string(),
        StructRef(struct_ref) => {
            let api_struct = struct_ref.get(api_file);

            let fields_str = &api_struct
                .fields
                .iter()
                .map(|field| {
                    format!(
                        "{} self.{}.wire2api()",
                        if api_struct.is_fields_named {
                            field.name.rust_style().to_string() + ": "
                        } else {
                            "".to_string()
                        },
                        field.name.rust_style()
                    )
                })
                .collect::<Vec<_>>()
                .join(",");

            if api_struct.is_fields_named {
                format!("{} {{ {} }}", ty.rust_api_type(), fields_str)
            } else {
                format!("{}({})", ty.rust_api_type(), fields_str)
            }
        }
    };

    format!(
        "impl Wire2Api<{}> for {} {{
            fn wire2api(self) -> {} {{
                {}
            }}
        }}
        ",
        ty.rust_api_type(),
        ty.rust_wire_modifier() + &ty.rust_wire_type(),
        ty.rust_api_type(),
        body,
    )
}

fn generate_new_with_nullptr_func(ty: &ApiType, api_file: &ApiFile) -> String {
    let body = match ty {
        StructRef(s) => s
            .get(api_file)
            .fields
            .iter()
            .map(|field| {
                format!(
                    "{}: {},",
                    field.name.rust_style(),
                    if field.ty.rust_wire_is_pointer() {
                        "std::ptr::null_mut()"
                    } else {
                        "Default::default()"
                    }
                )
            })
            .collect::<Vec<_>>()
            .join("\n"),
        Primitive(_) | Delegate(_) | PrimitiveList(_) | GeneralList(_) | Boxed(_) => {
            return String::new();
        }
    };

    format!(
        "impl NewWithNullPtr for {} {{
            fn new_with_null_ptr() -> Self {{
                Self {{
                    {}
                }}
            }}
        }}",
        ty.rust_wire_type(),
        body,
    )
}

fn generate_impl_intodart(ty: &ApiType, api_file: &ApiFile) -> String {
    // println!("generate_impl_intodart: {:?}", ty);
    match ty {
        StructRef(s) => generate_impl_intodart_for_struct(s.get(api_file)),
        Primitive(_) | Delegate(_) | PrimitiveList(_) | GeneralList(_) | Boxed(_) => "".to_string(),
    }
}

fn generate_impl_intodart_for_struct(s: &ApiStruct) -> String {
    // println!("generate_impl_intodart_for_struct: {}", s.name);
    let body = s
        .fields
        .iter()
        .map(|field| {
            format!(
                "self.{}.into_dart()",
                field.name_rust_style(s.is_fields_named)
            )
        })
        .collect::<Vec<_>>()
        .join(",\n");

    format!(
        "impl support::IntoDart for {} {{
            fn into_dart(self) -> support::DartCObject {{
                vec![
                    {}
                ].into_dart()
            }}
        }}",
        s.name, body,
    )
}
