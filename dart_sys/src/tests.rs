#[allow(unused_imports)]
use super::*;

#[test]
fn bindgen_test_layout_Dart_EmbedderInformation() {
    assert_eq!(
        ::std::mem::size_of::<Dart_EmbedderInformation>(),
        32usize,
        concat!("Size of: ", stringify!(Dart_EmbedderInformation))
    );
    assert_eq!(
        ::std::mem::align_of::<Dart_EmbedderInformation>(),
        8usize,
        concat!("Alignment of ", stringify!(Dart_EmbedderInformation))
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_EmbedderInformation>())).version as *const _ as usize
            }
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_EmbedderInformation),
            "::",
            stringify!(version)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_EmbedderInformation>())).name as *const _ as usize
            }
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_EmbedderInformation),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_EmbedderInformation>())).current_rss as *const _
                    as usize
            }
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_EmbedderInformation),
            "::",
            stringify!(current_rss)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_EmbedderInformation>())).max_rss as *const _ as usize
            }
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_EmbedderInformation),
            "::",
            stringify!(max_rss)
        )
    );
}

#[test]
fn bindgen_test_layout_Dart_CObject() {
    assert_eq!(
        ::std::mem::size_of::<Dart_CObject>(),
        48usize,
        concat!("Size of: ", stringify!(Dart_CObject))
    );
    assert_eq!(
        ::std::mem::align_of::<Dart_CObject>(),
        8usize,
        concat!("Alignment of ", stringify!(Dart_CObject))
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_CObject>())).type_ as *const _ as usize
            }
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_CObject),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_CObject>())).value as *const _ as usize
            }
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_CObject),
            "::",
            stringify!(value)
        )
    );
}

#[test]
fn bindgen_test_layout_Dart_CObjectValue() {
    assert_eq!(
        ::std::mem::size_of::<Dart_CObjectValue>(),
        40usize,
        concat!("Size of: ", stringify!(Dart_CObjectValue))
    );
    assert_eq!(
        ::std::mem::align_of::<Dart_CObjectValue>(),
        8usize,
        concat!("Alignment of ", stringify!(Dart_CObjectValue))
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_CObjectValue>())).as_bool as *const _ as usize
            }
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_CObjectValue),
            "::",
            stringify!(as_bool)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_CObjectValue>())).as_int32 as *const _ as usize
            }
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_CObjectValue),
            "::",
            stringify!(as_int32)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_CObjectValue>())).as_int64 as *const _ as usize
            }
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_CObjectValue),
            "::",
            stringify!(as_int64)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_CObjectValue>())).as_double as *const _ as usize
            }
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_CObjectValue),
            "::",
            stringify!(as_double)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_CObjectValue>())).as_string as *const _ as usize
            }
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_CObjectValue),
            "::",
            stringify!(as_string)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_CObjectValue>())).as_send_port as *const _ as usize
            }
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_CObjectValue),
            "::",
            stringify!(as_send_port)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_CObjectValue>())).as_capability as *const _ as usize
            }
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_CObjectValue),
            "::",
            stringify!(as_capability)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_CObjectValue>())).as_array as *const _ as usize
            }
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_CObjectValue),
            "::",
            stringify!(as_array)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_CObjectValue>())).as_typed_data as *const _ as usize
            }
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_CObjectValue),
            "::",
            stringify!(as_typed_data)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_CObjectValue>())).as_external_typed_data as *const _
                    as usize
            }
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_CObjectValue),
            "::",
            stringify!(as_external_typed_data)
        )
    );
}

#[test]
fn bindgen_test_layout_Dart_ExternalTypedData() {
    assert_eq!(
        ::std::mem::size_of::<Dart_ExternalTypedData>(),
        40usize,
        concat!("Size of: ", stringify!(Dart_ExternalTypedData))
    );
    assert_eq!(
        ::std::mem::align_of::<Dart_ExternalTypedData>(),
        8usize,
        concat!("Alignment of ", stringify!(Dart_ExternalTypedData))
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_ExternalTypedData>())).type_ as *const _ as usize
            }
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_ExternalTypedData),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_ExternalTypedData>())).length as *const _ as usize
            }
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_ExternalTypedData),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_ExternalTypedData>())).data as *const _ as usize
            }
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_ExternalTypedData),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_ExternalTypedData>())).peer as *const _ as usize
            }
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_ExternalTypedData),
            "::",
            stringify!(peer)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_ExternalTypedData>())).callback as *const _ as usize
            }
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_ExternalTypedData),
            "::",
            stringify!(callback)
        )
    );
}

#[test]
fn bindgen_test_layout_Dart_TypedData() {
    assert_eq!(
        ::std::mem::size_of::<Dart_TypedData>(),
        24usize,
        concat!("Size of: ", stringify!(Dart_TypedData))
    );
    assert_eq!(
        ::std::mem::align_of::<Dart_TypedData>(),
        8usize,
        concat!("Alignment of ", stringify!(Dart_TypedData))
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_TypedData>())).type_ as *const _ as usize
            }
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_TypedData),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_TypedData>())).length as *const _ as usize
            }
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_TypedData),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_TypedData>())).values as *const _ as usize
            }
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_TypedData),
            "::",
            stringify!(values)
        )
    );
}

#[test]
fn bindgen_test_layout_Dart_Array() {
    assert_eq!(
        ::std::mem::size_of::<Dart_Array>(),
        16usize,
        concat!("Size of: ", stringify!(Dart_Array))
    );
    assert_eq!(
        ::std::mem::align_of::<Dart_Array>(),
        8usize,
        concat!("Alignment of ", stringify!(Dart_Array))
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_Array>())).length as *const _ as usize
            }
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_Array),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_Array>())).values as *const _ as usize
            }
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_Array),
            "::",
            stringify!(values)
        )
    );
}

#[test]
fn bindgen_test_layout_Dart_Capability() {
    assert_eq!(
        ::std::mem::size_of::<Dart_Capability>(),
        8usize,
        concat!("Size of: ", stringify!(Dart_Capability))
    );
    assert_eq!(
        ::std::mem::align_of::<Dart_Capability>(),
        8usize,
        concat!("Alignment of ", stringify!(Dart_Capability))
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_Capability>())).id as *const _ as usize
            }
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_Capability),
            "::",
            stringify!(id)
        )
    );
}

#[test]
fn bindgen_test_layout_Dart_SendPort() {
    assert_eq!(
        ::std::mem::size_of::<Dart_SendPort>(),
        16usize,
        concat!("Size of: ", stringify!(Dart_SendPort))
    );
    assert_eq!(
        ::std::mem::align_of::<Dart_SendPort>(),
        8usize,
        concat!("Alignment of ", stringify!(Dart_SendPort))
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_SendPort>())).id as *const _ as usize
            }
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_SendPort),
            "::",
            stringify!(id)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_SendPort>())).origin_id as *const _ as usize
            }
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_SendPort),
            "::",
            stringify!(origin_id)
        )
    );
}

#[test]
fn bindgen_test_layout_Dart_SourceFile() {
    assert_eq!(
        ::std::mem::size_of::<Dart_SourceFile>(),
        16usize,
        concat!("Size of: ", stringify!(Dart_SourceFile))
    );
    assert_eq!(
        ::std::mem::align_of::<Dart_SourceFile>(),
        8usize,
        concat!("Alignment of ", stringify!(Dart_SourceFile))
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_SourceFile>())).uri as *const _ as usize
            }
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_SourceFile),
            "::",
            stringify!(uri)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_SourceFile>())).source as *const _ as usize
            }
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_SourceFile),
            "::",
            stringify!(source)
        )
    );
}

#[test]
fn bindgen_test_layout_Dart_KernelCompilationResult() {
    assert_eq!(
        ::std::mem::size_of::<Dart_KernelCompilationResult>(),
        32usize,
        concat!("Size of: ", stringify!(Dart_KernelCompilationResult))
    );
    assert_eq!(
        ::std::mem::align_of::<Dart_KernelCompilationResult>(),
        8usize,
        concat!("Alignment of ", stringify!(Dart_KernelCompilationResult))
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_KernelCompilationResult>())).status as *const _ as usize
            }
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_KernelCompilationResult),
            "::",
            stringify!(status)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_KernelCompilationResult>())).error as *const _ as usize
            }
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_KernelCompilationResult),
            "::",
            stringify!(error)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_KernelCompilationResult>())).kernel as *const _ as usize
            }
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_KernelCompilationResult),
            "::",
            stringify!(kernel)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_KernelCompilationResult>())).kernel_size as *const _
                    as usize
            }
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_KernelCompilationResult),
            "::",
            stringify!(kernel_size)
        )
    );
}

#[test]
fn bindgen_test_layout_Dart_NativeArgument_Value() {
    assert_eq!(
        ::std::mem::size_of::<Dart_NativeArgument_Value>(),
        16usize,
        concat!("Size of: ", stringify!(Dart_NativeArgument_Value))
    );
    assert_eq!(
        ::std::mem::align_of::<Dart_NativeArgument_Value>(),
        8usize,
        concat!("Alignment of ", stringify!(Dart_NativeArgument_Value))
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_NativeArgument_Value>())).as_bool as *const _ as usize
            }
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_NativeArgument_Value),
            "::",
            stringify!(as_bool)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_NativeArgument_Value>())).as_int32 as *const _ as usize
            }
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_NativeArgument_Value),
            "::",
            stringify!(as_int32)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_NativeArgument_Value>())).as_uint32 as *const _ as usize
            }
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_NativeArgument_Value),
            "::",
            stringify!(as_uint32)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_NativeArgument_Value>())).as_int64 as *const _ as usize
            }
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_NativeArgument_Value),
            "::",
            stringify!(as_int64)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_NativeArgument_Value>())).as_uint64 as *const _ as usize
            }
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_NativeArgument_Value),
            "::",
            stringify!(as_uint64)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_NativeArgument_Value>())).as_double as *const _ as usize
            }
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_NativeArgument_Value),
            "::",
            stringify!(as_double)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_NativeArgument_Value>())).as_string as *const _ as usize
            }
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_NativeArgument_Value),
            "::",
            stringify!(as_string)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_NativeArgument_Value>())).as_native_fields as *const _
                    as usize
            }
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_NativeArgument_Value),
            "::",
            stringify!(as_native_fields)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_NativeArgument_Value>())).as_instance as *const _
                    as usize
            }
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_NativeArgument_Value),
            "::",
            stringify!(as_instance)
        )
    );
}

#[test]
fn bindgen_test_layout_Dart_NativeFields() {
    assert_eq!(
        ::std::mem::size_of::<Dart_NativeFields>(),
        16usize,
        concat!("Size of: ", stringify!(Dart_NativeFields))
    );
    assert_eq!(
        ::std::mem::align_of::<Dart_NativeFields>(),
        8usize,
        concat!("Alignment of ", stringify!(Dart_NativeFields))
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_NativeFields>())).num_fields as *const _ as usize
            }
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_NativeFields),
            "::",
            stringify!(num_fields)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_NativeFields>())).values as *const _ as usize
            }
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_NativeFields),
            "::",
            stringify!(values)
        )
    );
}

#[test]
fn bindgen_test_layout_Dart_NativeString() {
    assert_eq!(
        ::std::mem::size_of::<Dart_NativeString>(),
        16usize,
        concat!("Size of: ", stringify!(Dart_NativeString))
    );
    assert_eq!(
        ::std::mem::align_of::<Dart_NativeString>(),
        8usize,
        concat!("Alignment of ", stringify!(Dart_NativeString))
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_NativeString>())).dart_str as *const _ as usize
            }
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_NativeString),
            "::",
            stringify!(dart_str)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_NativeString>())).peer as *const _ as usize
            }
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_NativeString),
            "::",
            stringify!(peer)
        )
    );
}

#[test]
fn bindgen_test_layout__Dart_NativeArgument_Descriptor() {
    assert_eq!(
        ::std::mem::size_of::<_Dart_NativeArgument_Descriptor>(),
        2usize,
        concat!("Size of: ", stringify!(_Dart_NativeArgument_Descriptor))
    );
    assert_eq!(
        ::std::mem::align_of::<_Dart_NativeArgument_Descriptor>(),
        1usize,
        concat!("Alignment of ", stringify!(_Dart_NativeArgument_Descriptor))
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<_Dart_NativeArgument_Descriptor>())).type_ as *const _
                    as usize
            }
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_Dart_NativeArgument_Descriptor),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<_Dart_NativeArgument_Descriptor>())).index as *const _
                    as usize
            }
        },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(_Dart_NativeArgument_Descriptor),
            "::",
            stringify!(index)
        )
    );
}

#[test]
fn bindgen_test_layout_Dart_InitializeParams() {
    assert_eq!(
        ::std::mem::size_of::<Dart_InitializeParams>(),
        136usize,
        concat!("Size of: ", stringify!(Dart_InitializeParams))
    );
    assert_eq!(
        ::std::mem::align_of::<Dart_InitializeParams>(),
        8usize,
        concat!("Alignment of ", stringify!(Dart_InitializeParams))
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_InitializeParams>())).version as *const _ as usize
            }
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_InitializeParams),
            "::",
            stringify!(version)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_InitializeParams>())).vm_snapshot_data as *const _
                    as usize
            }
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_InitializeParams),
            "::",
            stringify!(vm_snapshot_data)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_InitializeParams>())).vm_snapshot_instructions
                    as *const _ as usize
            }
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_InitializeParams),
            "::",
            stringify!(vm_snapshot_instructions)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_InitializeParams>())).create_group as *const _ as usize
            }
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_InitializeParams),
            "::",
            stringify!(create_group)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_InitializeParams>())).initialize_isolate as *const _
                    as usize
            }
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_InitializeParams),
            "::",
            stringify!(initialize_isolate)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_InitializeParams>())).shutdown_isolate as *const _
                    as usize
            }
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_InitializeParams),
            "::",
            stringify!(shutdown_isolate)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_InitializeParams>())).cleanup_isolate as *const _
                    as usize
            }
        },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_InitializeParams),
            "::",
            stringify!(cleanup_isolate)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_InitializeParams>())).cleanup_group as *const _ as usize
            }
        },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_InitializeParams),
            "::",
            stringify!(cleanup_group)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_InitializeParams>())).thread_exit as *const _ as usize
            }
        },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_InitializeParams),
            "::",
            stringify!(thread_exit)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_InitializeParams>())).file_open as *const _ as usize
            }
        },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_InitializeParams),
            "::",
            stringify!(file_open)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_InitializeParams>())).file_read as *const _ as usize
            }
        },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_InitializeParams),
            "::",
            stringify!(file_read)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_InitializeParams>())).file_write as *const _ as usize
            }
        },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_InitializeParams),
            "::",
            stringify!(file_write)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_InitializeParams>())).file_close as *const _ as usize
            }
        },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_InitializeParams),
            "::",
            stringify!(file_close)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_InitializeParams>())).entropy_source as *const _
                    as usize
            }
        },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_InitializeParams),
            "::",
            stringify!(entropy_source)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_InitializeParams>())).get_service_assets as *const _
                    as usize
            }
        },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_InitializeParams),
            "::",
            stringify!(get_service_assets)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_InitializeParams>())).start_kernel_isolate as *const _
                    as usize
            }
        },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_InitializeParams),
            "::",
            stringify!(start_kernel_isolate)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_InitializeParams>())).code_observer as *const _ as usize
            }
        },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_InitializeParams),
            "::",
            stringify!(code_observer)
        )
    );
}

#[test]
fn bindgen_test_layout_Dart_CodeObserver() {
    assert_eq!(
        ::std::mem::size_of::<Dart_CodeObserver>(),
        16usize,
        concat!("Size of: ", stringify!(Dart_CodeObserver))
    );
    assert_eq!(
        ::std::mem::align_of::<Dart_CodeObserver>(),
        8usize,
        concat!("Alignment of ", stringify!(Dart_CodeObserver))
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_CodeObserver>())).data as *const _ as usize
            }
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_CodeObserver),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_CodeObserver>())).on_new_code as *const _ as usize
            }
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_CodeObserver),
            "::",
            stringify!(on_new_code)
        )
    );
}

#[test]
fn bindgen_test_layout_Dart_IsolateFlags() {
    assert_eq!(
        ::std::mem::size_of::<Dart_IsolateFlags>(),
        24usize,
        concat!("Size of: ", stringify!(Dart_IsolateFlags))
    );
    assert_eq!(
        ::std::mem::align_of::<Dart_IsolateFlags>(),
        8usize,
        concat!("Alignment of ", stringify!(Dart_IsolateFlags))
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_IsolateFlags>())).version as *const _ as usize
            }
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_IsolateFlags),
            "::",
            stringify!(version)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_IsolateFlags>())).enable_asserts as *const _ as usize
            }
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_IsolateFlags),
            "::",
            stringify!(enable_asserts)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_IsolateFlags>())).use_field_guards as *const _ as usize
            }
        },
        5usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_IsolateFlags),
            "::",
            stringify!(use_field_guards)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_IsolateFlags>())).use_osr as *const _ as usize
            }
        },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_IsolateFlags),
            "::",
            stringify!(use_osr)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_IsolateFlags>())).obfuscate as *const _ as usize
            }
        },
        7usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_IsolateFlags),
            "::",
            stringify!(obfuscate)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_IsolateFlags>())).entry_points as *const _ as usize
            }
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_IsolateFlags),
            "::",
            stringify!(entry_points)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_IsolateFlags>())).load_vmservice_library as *const _
                    as usize
            }
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_IsolateFlags),
            "::",
            stringify!(load_vmservice_library)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_IsolateFlags>())).unsafe_trust_strong_mode_types
                    as *const _ as usize
            }
        },
        17usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_IsolateFlags),
            "::",
            stringify!(unsafe_trust_strong_mode_types)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_IsolateFlags>())).copy_parent_code as *const _ as usize
            }
        },
        18usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_IsolateFlags),
            "::",
            stringify!(copy_parent_code)
        )
    );
}

#[test]
fn bindgen_test_layout_Dart_QualifiedFunctionName() {
    assert_eq!(
        ::std::mem::size_of::<Dart_QualifiedFunctionName>(),
        24usize,
        concat!("Size of: ", stringify!(Dart_QualifiedFunctionName))
    );
    assert_eq!(
        ::std::mem::align_of::<Dart_QualifiedFunctionName>(),
        8usize,
        concat!("Alignment of ", stringify!(Dart_QualifiedFunctionName))
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_QualifiedFunctionName>())).library_uri as *const _
                    as usize
            }
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_QualifiedFunctionName),
            "::",
            stringify!(library_uri)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_QualifiedFunctionName>())).class_name as *const _
                    as usize
            }
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_QualifiedFunctionName),
            "::",
            stringify!(class_name)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<Dart_QualifiedFunctionName>())).function_name as *const _
                    as usize
            }
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(Dart_QualifiedFunctionName),
            "::",
            stringify!(function_name)
        )
    );
}

#[test]
fn bindgen_test_layout__Lldiv_t() {
    assert_eq!(
        ::std::mem::size_of::<_Lldiv_t>(),
        16usize,
        concat!("Size of: ", stringify!(_Lldiv_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_Lldiv_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_Lldiv_t))
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<_Lldiv_t>())).quot as *const _ as usize
            }
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_Lldiv_t),
            "::",
            stringify!(quot)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<_Lldiv_t>())).rem as *const _ as usize
            }
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_Lldiv_t),
            "::",
            stringify!(rem)
        )
    );
}

#[test]
fn bindgen_test_layout__Mbstatet() {
    assert_eq!(
        ::std::mem::size_of::<_Mbstatet>(),
        8usize,
        concat!("Size of: ", stringify!(_Mbstatet))
    );
    assert_eq!(
        ::std::mem::align_of::<_Mbstatet>(),
        4usize,
        concat!("Alignment of ", stringify!(_Mbstatet))
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<_Mbstatet>()))._Wchar as *const _ as usize
            }
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_Mbstatet),
            "::",
            stringify!(_Wchar)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<_Mbstatet>()))._Byte as *const _ as usize
            }
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(_Mbstatet),
            "::",
            stringify!(_Byte)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<_Mbstatet>()))._State as *const _ as usize
            }
        },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(_Mbstatet),
            "::",
            stringify!(_State)
        )
    );
}

#[test]
fn bindgen_test_layout___crt_locale_pointers() {
    assert_eq!(
        ::std::mem::size_of::<__crt_locale_pointers>(),
        16usize,
        concat!("Size of: ", stringify!(__crt_locale_pointers))
    );
    assert_eq!(
        ::std::mem::align_of::<__crt_locale_pointers>(),
        8usize,
        concat!("Alignment of ", stringify!(__crt_locale_pointers))
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<__crt_locale_pointers>())).locinfo as *const _ as usize
            }
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__crt_locale_pointers),
            "::",
            stringify!(locinfo)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<__crt_locale_pointers>())).mbcinfo as *const _ as usize
            }
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__crt_locale_pointers),
            "::",
            stringify!(mbcinfo)
        )
    );
}

#[test]
fn bindgen_test_layout___crt_locale_data_public() {
    assert_eq!(
        ::std::mem::size_of::<__crt_locale_data_public>(),
        16usize,
        concat!("Size of: ", stringify!(__crt_locale_data_public))
    );
    assert_eq!(
        ::std::mem::align_of::<__crt_locale_data_public>(),
        8usize,
        concat!("Alignment of ", stringify!(__crt_locale_data_public))
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<__crt_locale_data_public>()))._locale_pctype as *const _
                    as usize
            }
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__crt_locale_data_public),
            "::",
            stringify!(_locale_pctype)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<__crt_locale_data_public>()))._locale_mb_cur_max as *const _
                    as usize
            }
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__crt_locale_data_public),
            "::",
            stringify!(_locale_mb_cur_max)
        )
    );
    assert_eq!(
        unsafe {
            #[allow(deref_nullptr)]
            {
                &(*(::std::ptr::null::<__crt_locale_data_public>()))._locale_lc_codepage as *const _
                    as usize
            }
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(__crt_locale_data_public),
            "::",
            stringify!(_locale_lc_codepage)
        )
    );
}
