use crate::api_types::ApiType::{Boxed, StructRef};
use crate::api_types::{ApiField, ApiFile, ApiFunc, ApiTypeBoxed};
use log::debug;

pub fn transform(src: ApiFile) -> ApiFile {
    let dst_funcs = src
        .funcs
        .into_iter()
        .map(|src_func| ApiFunc {
            name: src_func.name.clone(),
            inputs: src_func
                .inputs
                .into_iter()
                .map(transform_func_input_add_boxed)
                .collect(),
            output: src_func.output,
        })
        .collect();

    ApiFile {
        funcs: dst_funcs,
        struct_pool: src.struct_pool,
    }
}

fn transform_func_input_add_boxed(input: ApiField) -> ApiField {
    if let StructRef(_) = &input.ty {
        debug!(
            "transform_func_input_add_boxed wrap Boxed to field={:?}",
            input
        );
        ApiField {
            ty: Boxed(Box::new(ApiTypeBoxed {
                exist_in_real_api: false, // <--
                inner: input.ty.clone(),
            })),
            name: input.name.clone(),
        }
    } else {
        input.clone()
    }
}
