use std::borrow::Cow;

use crate::api_types::ApiType::*;
use crate::api_types::*;
use crate::others::*;

pub const HANDLER_NAME: &str = "FLUTTER_RUST_BRIDGE_HANDLER";

pub struct Output {
    pub code: String,
    pub extern_func_names: Vec<String>,
}

pub fn generate(api_file: &ApiFile, rust_wire_mod: &str) -> Output {
    let mut generator = Generator::new();
    let code = generator.generate(api_file, rust_wire_mod);

    Output {
        code,
        extern_func_names: generator.extern_func_collector.names,
    }
}

struct Generator {
    extern_func_collector: ExternFuncCollector,
}

impl Generator {
    fn new() -> Self {
        Self {
            extern_func_collector: ExternFuncCollector::new(),
        }
    }

    fn generate(&mut self, api_file: &ApiFile, rust_wire_mod: &str) -> String {
        let distinct_input_types = api_file.distinct_types(true, false);
        let distinct_output_types = api_file.distinct_types(false, true);

        let wire_funcs = api_file
            .funcs
            .iter()
            .map(|f| self.generate_wire_func(f))
            .collect::<Vec<_>>();
        let wire_structs = distinct_input_types
            .iter()
            .map(|ty| self.generate_wire_struct(ty, api_file))
            .collect::<Vec<_>>();
        let allocate_funcs = distinct_input_types
            .iter()
            .map(|f| self.generate_allocate_funcs(f))
            .collect::<Vec<_>>();
        let wire2api_funcs = distinct_input_types
            .iter()
            .map(|ty| self.generate_wire2api_func(ty, api_file))
            .collect::<Vec<_>>();
        let new_with_nullptr_funcs = distinct_input_types
            .iter()
            .map(|ty| self.generate_new_with_nullptr_func(ty, api_file))
            .collect::<Vec<_>>();
        let impl_intodart = distinct_output_types
            .iter()
            .map(|ty| self.generate_impl_intodart(ty, api_file))
            .collect::<Vec<_>>();

        format!(
            r#"#![allow(non_camel_case_types, unused, clippy::redundant_closure, clippy::useless_conversion)]
        {}

        use crate::{}::*;
        use flutter_rust_bridge::*;

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

        impl<T, S> Wire2Api<Option<T>> for *mut S
        where
            *mut S: Wire2Api<T>
        {{
            fn wire2api(self) -> Option<T> {{
                if self.is_null() {{
                    None
                }} else {{
                    Some(self.wire2api())
                }}
            }}
        }}

        {}

        // Section: impl NewWithNullPtr

        pub trait NewWithNullPtr {{
            fn new_with_null_ptr() -> Self;
        }}

        impl<T> NewWithNullPtr for *mut T {{
            fn new_with_null_ptr() -> Self {{
                std::ptr::null_mut()
            }}
        }}

        {}

        // Section: impl IntoDart
        {}

        // Section: executor
        {}

        "#,
            CODE_HEADER,
            rust_wire_mod,
            wire_funcs.join("\n\n"),
            wire_structs.join("\n\n"),
            allocate_funcs.join("\n\n"),
            wire2api_funcs.join("\n\n"),
            new_with_nullptr_funcs.join("\n\n"),
            impl_intodart.join("\n\n"),
            self.generate_executor(api_file),
        )
    }

    fn generate_executor(&mut self, api_file: &ApiFile) -> String {
        if api_file.has_executor {
            "/* nothing since executor detected */".to_string()
        } else {
            format!(
                "support::lazy_static! {{
                pub static ref {}: support::DefaultHandler = Default::default();
            }}
            ",
                HANDLER_NAME
            )
        }
    }

    fn generate_wire_func(&mut self, func: &ApiFunc) -> String {
        let params = [
            vec!["port: i64".to_string()],
            func.inputs
                .iter()
                .map(|field| {
                    format!(
                        "{}: {}{}",
                        field.name.rust_style(),
                        field.ty.rust_wire_modifier(),
                        field.ty.rust_wire_type()
                    )
                })
                .collect::<Vec<_>>(),
        ]
        .concat();

        let inner_func_params = [
            match func.mode {
                ApiFuncMode::Normal | ApiFuncMode::Sync => vec![],
                ApiFuncMode::Stream => vec!["task_callback.stream_sink()".to_string()],
            },
            func.inputs
                .iter()
                .map(|field| format!("api_{}", field.name.rust_style()))
                .collect::<Vec<_>>(),
        ]
        .concat();

        let wrap_info_obj = format!(
            "WrapInfo{{ debug_name: \"{}\", port, mode: FfiCallMode::{} }}",
            func.name,
            func.mode.ffi_call_mode(),
        );

        let (handler_func_name, return_type) = match func.mode {
            ApiFuncMode::Sync => (
                "wrap_sync",
                Some(func.output.rust_wire_modifier() + &func.output.rust_wire_type()),
            ),
            ApiFuncMode::Normal | ApiFuncMode::Stream => ("wrap", None),
        };

        self.extern_func_collector.generate(
            &func.wire_func_name(),
            &params
                .iter()
                .map(std::ops::Deref::deref)
                .collect::<Vec<_>>(),
            return_type.as_deref(),
            &format!(
                "
                {}.{}({}, move || {{
                    {}
                    move |task_callback| {}({})
                }});
                ",
                HANDLER_NAME,
                handler_func_name,
                wrap_info_obj,
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
                inner_func_params.join(", "),
            ),
        )
    }

    fn generate_wire_struct(&mut self, ty: &ApiType, api_file: &ApiFile) -> String {
        // println!("generate_wire_struct: {:?}", ty);
        let fields = match ty {
            PrimitiveList(list) => vec![
                format!("ptr: *mut {}", list.primitive.rust_wire_type()),
                "len: i32".to_string(),
            ],
            GeneralList(list) => vec![
                format!(
                    "ptr: *mut {}{}",
                    list.inner.optional_ptr_modifier(),
                    list.inner.rust_wire_type()
                ),
                "len: i32".to_string(),
            ],
            StructRef(s) => {
                let s = s.get(api_file);
                s.fields
                    .iter()
                    .map(|field| {
                        format!(
                            "{}: {}{}",
                            field.name.rust_style(),
                            field.ty.rust_wire_modifier(),
                            field.ty.rust_wire_type()
                        )
                    })
                    .collect()
            }
            Primitive(_) | Delegate(_) | Boxed(_) | Optional(_) => return "".to_string(),
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

    fn generate_allocate_funcs(&mut self, ty: &ApiType) -> String {
        // println!("generate_allocate_funcs: {:?}", ty);

        match ty {
            Primitive(_) | Delegate(_) | Optional(_) => "".to_string(),
            PrimitiveList(list) => self.extern_func_collector.generate(
                &format!("new_{}", list.safe_ident()),
                &["len: i32"],
                Some(&format!("{}{}", list.rust_wire_modifier(), list.rust_wire_type())),
                &format!(
                    "let ans = {} {{ ptr: support::new_leak_vec_ptr(Default::default(), len), len }};
                support::new_leak_box_ptr(ans)",
                    list.rust_wire_type(),
                ),
            ),
            GeneralList(list) => self.extern_func_collector.generate(
                &format!("new_{}", ty.safe_ident()),
                &["len: i32"],
                Some(&[
                    list.rust_wire_modifier().as_str(),
                    list.rust_wire_type().as_str()
                ].concat()),
                &format!(
                    "let wrap = {} {{ ptr: support::new_leak_vec_ptr(<{}{}>::new_with_null_ptr(), len), len }};
                    support::new_leak_box_ptr(wrap)",
                    list.rust_wire_type(),
                    list.inner.optional_ptr_modifier(),
                    list.inner.rust_wire_type()
                ),
            ),
            StructRef(_) => "".to_string(),
            Boxed(b) => {
                match &b.inner {
                    Primitive(prim) => {
                        self.extern_func_collector.generate(
                            &format!("new_{}", ty.safe_ident()),
                            &[&format!("value: {}", prim.rust_wire_type())],
                            Some(&format!("*mut {}", prim.rust_wire_type())),
                            "support::new_leak_box_ptr(value)",
                        )
                    }
                    inner => {
                        self.extern_func_collector.generate(
                            &format!("new_{}", ty.safe_ident()),
                            &[],
                            Some(&[ty.rust_wire_modifier(), ty.rust_wire_type()].concat()),
                            &format!(
                                "support::new_leak_box_ptr({}::new_with_null_ptr())",
                                inner.rust_wire_type()
                            ),
                        )
                    }
                }
            }
        }
    }

    fn generate_wire2api_func(&mut self, ty: &ApiType, api_file: &ApiFile) -> String {
        // println!("generate_wire2api_func: {:?}", ty);
        let body: Cow<str> = match ty {
            Primitive(_) => "self".into(),
            Delegate(d) => match d {
                ApiTypeDelegate::String => "let vec: Vec<u8> = self.wire2api();
                String::from_utf8_lossy(&vec).into_owned()"
                    .into(),
                ApiTypeDelegate::SyncReturnVecU8 => "/*unsupported*/".into(),
                ApiTypeDelegate::ZeroCopyBufferVecPrimitive(_) => {
                    "ZeroCopyBuffer(self.wire2api())".into()
                }
            },
            PrimitiveList(_) => "unsafe {
                let wrap = support::box_from_leak_ptr(self);
                support::vec_from_leak_ptr(wrap.ptr, wrap.len)
            }"
            .into(),
            GeneralList(_) => "
            let vec = unsafe {
                let wrap = support::box_from_leak_ptr(self);
                support::vec_from_leak_ptr(wrap.ptr, wrap.len)
            };
            vec.into_iter().map(Wire2Api::wire2api).collect()"
                .into(),
            Boxed(_) => "let wrap = unsafe { support::box_from_leak_ptr(self) };
            (*wrap).wire2api().into()"
                .into(),
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
                                String::new()
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
                .into()
            }
            // handled by common impl
            Optional(_) => return String::new(),
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

    fn generate_new_with_nullptr_func(&mut self, ty: &ApiType, api_file: &ApiFile) -> String {
        match ty {
            StructRef(st) => self
                .generate_new_with_nullptr_func_for_struct(st.get(api_file), &ty.rust_wire_type()),
            Primitive(_) | Delegate(_) | PrimitiveList(_) | GeneralList(_) | Boxed(_)
            | Optional(_) => String::new(),
        }
    }

    fn generate_impl_intodart(&mut self, ty: &ApiType, api_file: &ApiFile) -> String {
        // println!("generate_impl_intodart: {:?}", ty);
        match ty {
            StructRef(s) => self.generate_impl_intodart_for_struct(s.get(api_file)),
            Primitive(_) | Delegate(_) | PrimitiveList(_) | GeneralList(_) | Boxed(_)
            | Optional(_) => "".to_string(),
        }
    }

    fn generate_new_with_nullptr_func_for_struct(
        &self,
        s: &ApiStruct,
        rust_wire_type: &str,
    ) -> String {
        let body = {
            s.fields
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
                .join("\n")
        };
        format!(
            r#"impl NewWithNullPtr for {} {{
                    fn new_with_null_ptr() -> Self {{
                        Self {{ {} }}
                    }}
                }}"#,
            rust_wire_type, body,
        )
    }

    fn generate_impl_intodart_for_struct(&mut self, s: &ApiStruct) -> String {
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
            }}
            impl support::IntoDartExceptPrimitive for {} {{}}
            ",
            s.name, body, s.name,
        )
    }
}

struct ExternFuncCollector {
    names: Vec<String>,
}

impl ExternFuncCollector {
    fn new() -> Self {
        ExternFuncCollector { names: vec![] }
    }

    fn generate(
        &mut self,
        func_name: &str,
        params: &[&str],
        return_type: Option<&str>,
        body: &str,
    ) -> String {
        self.names.push(func_name.to_string());

        format!(
            r#"
                #[no_mangle]
                pub extern "C" fn {}({}) {} {{
                    {}
                }}
            "#,
            func_name,
            params.join(", "),
            return_type.map_or("".to_string(), |r| format!("-> {}", r)),
            body,
        )
    }
}
