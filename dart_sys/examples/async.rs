#![crate_type = "cdylib"]

use dart_sys as ffi;

use rand::{prelude::StdRng, Rng, SeedableRng};
use std::ffi::CStr;
use std::mem::MaybeUninit;

#[allow(non_snake_case)]
#[no_mangle]
unsafe extern "C" fn dart_rust_async_ffi_Init(
    parent_library: ffi::Dart_Handle,
) -> ffi::Dart_Handle {
    if ffi::Dart_IsError(parent_library) {
        return parent_library;
    }

    let result_code = ffi::Dart_SetNativeResolver(parent_library, Some(ResloveName), None);
    if ffi::Dart_IsError(result_code) {
        result_code
    } else {
        ffi::Dart_Null()
    }
}

#[allow(non_snake_case)]
#[no_mangle]
unsafe extern "C" fn ResloveName(
    name: ffi::Dart_Handle,
    _argc: std::os::raw::c_int,
    _auto_setup_scope: *mut bool,
) -> ffi::Dart_NativeFunction {
    if !ffi::Dart_IsString(name) {
        return None;
    }
    let mut result: ffi::Dart_NativeFunction = None;
    let mut cname = MaybeUninit::<*const libc::c_char>::uninit();
    HandleError(ffi::Dart_StringToCString(name, cname.as_mut_ptr()));

    let cname = CStr::from_ptr(cname.assume_init());
    if cname.to_bytes() == b"RandomArray_ServicePort" {
        result = Some(randomArrayServicePort);
    } else {
        println!("Unknown function! {:?}", cname);
    }

    result
}

#[allow(non_snake_case)]
#[no_mangle]
unsafe extern "C" fn HandleError(handle: ffi::Dart_Handle) -> ffi::Dart_Handle {
    if ffi::Dart_IsError(handle) {
        ffi::Dart_PropagateError(handle);
    }
    handle
}

fn random_array(seed: u32, length: i32) -> Option<Vec<u8>> {
    if length <= 0 || length > 10000000 {
        return None;
    }
    let mut rng = StdRng::seed_from_u64(seed as _);
    let values: Vec<u8> = (0..length).map(move |_| rng.gen()).collect();

    println!("Created random array");

    Some(values)
}

unsafe extern "C" fn wrapped_random_array(
    _dest_port_id: ffi::Dart_Port,
    message: *mut ffi::Dart_CObject,
) {
    let reply_port_id;
    if (*message).type_ == ffi::Dart_CObject_Type::Array && 3 == (*message).value.as_array.length {
        // Use .as_array and .as_int32 to access the data in the Dart_CObject.
        let param0 = (*(*message).value.as_array.values).offset(0);
        let param1 = (*(*message).value.as_array.values).offset(1);
        let param2 = (*(*message).value.as_array.values).offset(2);
        if (*param0).type_ == ffi::Dart_CObject_Type::Int32
            && (*param1).type_ == ffi::Dart_CObject_Type::Int32
            && (*param2).type_ == ffi::Dart_CObject_Type::SendPort
        {
            let seed = (*param0).value.as_int32;
            let length = (*param1).value.as_int32;
            reply_port_id = (*param2).value.as_send_port.id;
            let values = random_array(seed as _, length);

            if let Some(mut values) = values {
                let mut result: ffi::Dart_CObject = ffi::Dart_CObject {
                    type_: ffi::Dart_CObject_Type::TypedData,
                    value: ffi::Dart_CObjectValue {
                        as_typed_data: ffi::Dart_TypedData {
                            type_: ffi::Dart_TypedData_Type::Uint8,
                            length: values.len() as isize,
                            values: values.as_mut_ptr() as *mut _,
                        },
                    },
                };

                if ffi::Dart_PostCObject(reply_port_id, &mut result) {
                    let mut error: ffi::Dart_CObject = ffi::Dart_CObject {
                        type_: ffi::Dart_CObject_Type::Null,
                        value: ffi::Dart_CObjectValue { as_bool: false },
                    };

                    ffi::Dart_PostCObject(reply_port_id, &mut error);
                }
                drop(values);
                // It is OK that result is destroyed when function exits.
                // Dart_PostCObject has copied its data.
                return;
            }
        }
    }
    eprintln!("Invalid message received, cannot proceed. Aborting the process.");
    std::process::abort();
}

#[no_mangle]
unsafe extern "C" fn randomArrayServicePort(arguments: ffi::Dart_NativeArguments) {
    ffi::Dart_EnterScope();
    ffi::Dart_SetReturnValue(arguments, ffi::Dart_Null());
    let service_port = ffi::Dart_NewNativePort(
        b"RandomArrayService\0".as_ptr() as *const _,
        Some(wrapped_random_array),
        true,
    );
    if service_port != ffi::ILLEGAL_PORT {
        // https://github.com/dart-lang/sdk/blob/9a683de40dd5d0ab623b2a105295ea58964d6afc/runtime/include/dart_api.h#L1173
        let send_port = ffi::Dart_NewSendPort(service_port);
        ffi::Dart_SetReturnValue(arguments, send_port);
    }
    ffi::Dart_ExitScope();
}
