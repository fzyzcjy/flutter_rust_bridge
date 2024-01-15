// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `optional.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

use crate::api::pseudo_manual::misc_example_twin_rust_async::WeekdaysTwinRustAsync;
use crate::api::pseudo_manual::newtype_pattern_twin_rust_async::NewTypeIntTwinRustAsync;

pub async fn handle_optional_return_twin_rust_async(left: f64, right: f64) -> Option<f64> {
    if right == 0. {
        None
    } else {
        Some(left / right)
    }
}

#[derive(Default, Debug, Clone)]
pub struct ElementTwinRustAsync {
    pub tag: Option<String>,
    pub text: Option<String>,
    pub attributes: Option<Vec<AttributeTwinRustAsync>>,
    pub children: Option<Vec<ElementTwinRustAsync>>,
}

#[derive(Debug, Clone)]
pub struct AttributeTwinRustAsync {
    pub key: String,
    pub value: String,
}

pub async fn handle_optional_struct_twin_rust_async(
    document: Option<String>,
) -> Option<ElementTwinRustAsync> {
    document.map(|inner| ElementTwinRustAsync {
        tag: Some("div".to_owned()),
        attributes: Some(vec![AttributeTwinRustAsync {
            key: "id".to_owned(),
            value: "root".to_owned(),
        }]),
        children: Some(vec![ElementTwinRustAsync {
            tag: Some("p".to_owned()),
            children: Some(vec![ElementTwinRustAsync {
                text: Some(inner),
                ..Default::default()
            }]),
            ..Default::default()
        }]),
        ..Default::default()
    })
}

#[derive(Debug)]
pub struct ExoticOptionalsTwinRustAsync {
    pub int32: Option<i32>,
    pub int64: Option<i64>,
    pub float64: Option<f64>,
    pub boolean: Option<bool>,
    pub zerocopy: Option<Vec<u8>>,
    pub int8list: Option<Vec<i8>>,
    pub uint8list: Option<Vec<u8>>,
    pub int32list: Option<Vec<i32>>,
    pub float32list: Option<Vec<f32>>,
    pub float64list: Option<Vec<f64>>,
    pub attributes: Option<Vec<AttributeTwinRustAsync>>,
    pub attributes_nullable: Vec<Option<AttributeTwinRustAsync>>,
    pub nullable_attributes: Option<Vec<Option<AttributeTwinRustAsync>>>,
    pub newtypeint: Option<NewTypeIntTwinRustAsync>,
}

pub async fn handle_optional_increment_twin_rust_async(
    opt: Option<ExoticOptionalsTwinRustAsync>,
) -> Option<ExoticOptionalsTwinRustAsync> {
    fn manipulate_list<T>(src: Option<Vec<T>>, push_value: T) -> Option<Vec<T>> {
        let mut list = src.unwrap_or_default();
        list.push(push_value);
        Some(list)
    }

    opt.map(|mut opt| ExoticOptionalsTwinRustAsync {
        int32: Some(opt.int32.unwrap_or(0) + 1),
        int64: Some(opt.int64.unwrap_or(0) + 1),
        float64: Some(opt.float64.unwrap_or(0.) + 1.),
        boolean: Some(!opt.boolean.unwrap_or(false)),
        int8list: manipulate_list(opt.int8list, 0),
        uint8list: manipulate_list(opt.uint8list, 0),
        int32list: manipulate_list(opt.int32list, 0),
        float32list: manipulate_list(opt.float32list, 0.),
        float64list: manipulate_list(opt.float64list, 0.),
        attributes: Some({
            let mut list = opt.attributes.unwrap_or_default();
            list.push(AttributeTwinRustAsync {
                key: "some-attrib".to_owned(),
                value: "some-value".to_owned(),
            });
            list
        }),
        nullable_attributes: Some({
            let mut list = opt.nullable_attributes.unwrap_or_default();
            list.push(None);
            list
        }),
        newtypeint: Some({
            let mut val = opt.newtypeint.unwrap_or(NewTypeIntTwinRustAsync(0));
            val.0 += 1;
            val
        }),
        attributes_nullable: {
            opt.attributes_nullable.push(None);
            opt.attributes_nullable
        },
        zerocopy: Some({
            let mut list = opt.zerocopy.unwrap_or_default();
            list.push(0);
            list
        }),
    })
}

pub async fn handle_increment_boxed_optional_twin_rust_async(opt: Option<Box<f64>>) -> f64 {
    match opt {
        Some(e) => *e + 1.,
        None => 42.,
    }
}

pub struct OptVecsTwinRustAsync {
    pub i32: Vec<Option<i32>>,
    pub enums: Vec<Option<WeekdaysTwinRustAsync>>,
    pub strings: Vec<Option<String>>,
    pub buffers: Vec<Option<Vec<i32>>>,
}

pub async fn handle_vec_of_opts_twin_rust_async(opt: OptVecsTwinRustAsync) -> OptVecsTwinRustAsync {
    fn handle<T>(mut opts: Vec<Option<T>>) -> Vec<Option<T>> {
        opts.push(None);
        opts
    }
    OptVecsTwinRustAsync {
        i32: handle(opt.i32),
        enums: handle(opt.enums),
        strings: handle(opt.strings),
        buffers: handle(opt.buffers),
    }
}

// Option<Box<T>> can't be sent to Dart,
// but instead can be received by Rust.
pub async fn handle_option_box_arguments_twin_rust_async(
    i8box: Option<Box<i8>>,
    u8box: Option<Box<u8>>,
    i32box: Option<Box<i32>>,
    i64box: Option<Box<i64>>,
    f64box: Option<Box<f64>>,
    boolbox: Option<Box<bool>>,
    structbox: Option<Box<ExoticOptionalsTwinRustAsync>>,
) -> String {
    format!(
        "handle_option_box_arguments({:?})",
        (i8box, u8box, i32box, i64box, f64box, boolbox, structbox)
    )
}

// TODO move it to a non-auto-generated test
// #[frb(sync)]
// pub async fn sync_option_twin_rust_async() -> anyhow::Result<Option<String>> {
//     Ok(Some("42".to_owned()))
// }
//
// #[frb(sync)]
// pub async fn sync_option_null_twin_rust_async() -> anyhow::Result<Option<String>> {
//     Ok(None)
// }
