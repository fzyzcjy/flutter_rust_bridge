use crate::codegen::ir::func::IrFuncOwnerInfo;
use crate::codegen::ir::ty::boxed::IrTypeBoxed;
use crate::codegen::ir::ty::dart_opaque::IrTypeDartOpaque;
use crate::codegen::ir::ty::delegate::{IrTypeDelegate, IrTypeDelegateMap, IrTypeDelegateSet, IrTypeDelegateStreamSink, IrTypeDelegateTime};
use crate::codegen::ir::ty::dynamic::IrTypeDynamic;
use crate::codegen::ir::ty::general_list::ir_list;
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::{Boxed, DartOpaque, Delegate, Dynamic};
use crate::codegen::parser::type_parser::unencodable::{splay_segments, SplayedSegment};
use crate::codegen::parser::type_parser::TypeParserWithContext;
use crate::if_then_some;
use anyhow::bail;
use itertools::Itertools;
use quote::quote;
use syn::{parse_str, Type};

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_type_path_data_concrete(
        &mut self,
        last_segment: &SplayedSegment,
        splayed_segments: &[SplayedSegment],
    ) -> anyhow::Result<Option<IrType>> {
        let non_last_segments = (splayed_segments.split_last().unwrap().1.iter())
            .map(|segment| segment.0)
            .join("::");
        let check_prefix =
            |matcher: &str| non_last_segments == matcher || non_last_segments.is_empty();

        Ok(Some(match last_segment {
            ("Self", []) => self.parse_type_self()?,

            ("Duration", []) if check_prefix("chrono") => Delegate(IrTypeDelegate::Time(IrTypeDelegateTime::Duration)),
            ("NaiveDateTime", []) if check_prefix("chrono") => Delegate(IrTypeDelegate::Time(IrTypeDelegateTime::Naive)),
            ("DateTime", args) if check_prefix("chrono") => self.parse_datetime(args)?,

            ("Uuid", []) if check_prefix("uuid") => Delegate(IrTypeDelegate::Uuid),
            ("String", []) => Delegate(IrTypeDelegate::String),
            ("Backtrace", []) => Delegate(IrTypeDelegate::Backtrace),

            ("DartAbi", []) => Dynamic(IrTypeDynamic),
            ("DartDynamic", []) => Dynamic(IrTypeDynamic),

            ("DartOpaque", []) => DartOpaque(IrTypeDartOpaque {}),

            ("ZeroCopyBuffer", _) => bail!("`ZeroCopyBuffer<T>` is no longer needed, since zero-copy is automatically utilized, just directly use `T` instead."),
            // (
            //     "ZeroCopyBuffer",
            //     Some(Generic([PrimitiveList(IrTypePrimitiveList { primitive })])),
            // ) => Delegate(IrTypeDelegate::ZeroCopyBufferVecPrimitive(
            //     primitive.clone(),
            // )),

            ("Box", [inner]) => {
                let inner = self.parse_type(inner)?;
                match inner {
                    IrType::RustAutoOpaque(ty_raw) => self.transform_rust_auto_opaque(
                        &ty_raw,
                        |raw| format!("Box<{raw}>"),
                    )?,
                    _ => Boxed(IrTypeBoxed {
                        exist_in_real_api: true,
                        inner: Box::new(inner),
                    })
                }
            },

            ("Vec", [element]) => ir_list(self.parse_type(element)?, true),

            ("HashMap", [key, value]) => {
                let key  = self.parse_type(key)?;
                let value  = self.parse_type(value)?;
                Delegate(IrTypeDelegate::Map(IrTypeDelegateMap {
                    key: Box::new(key.clone()),
                    value: Box::new(value.clone()),
                    element_delegate: self.create_ir_record(vec![key, value]),
                }))
            },
            ("HashSet", [inner]) => Delegate(IrTypeDelegate::Set(IrTypeDelegateSet {
                inner: Box::new(self.parse_type(inner)?),
            })),

            ("StreamSink", [inner ]) => Delegate(IrTypeDelegate::StreamSink(IrTypeDelegateStreamSink {
                inner: Box::new(self.parse_type(inner)?),
                codec: None,
            })),
            ("StreamSink", [inner, codec ]) => Delegate(IrTypeDelegate::StreamSink(IrTypeDelegateStreamSink {
                inner: Box::new(self.parse_type(inner)?),
                codec: Some(quote!(#codec).to_string().parse()?),
            })),

            _ => return Ok(None),
        }))
    }

    fn parse_type_self(&mut self) -> anyhow::Result<IrType> {
        let enum_or_struct_name = if_then_some!(let IrFuncOwnerInfo::Method(info) = self.context.owner.as_ref().unwrap(), info.enum_or_struct_name.name.clone()).unwrap();
        self.parse_type(&parse_str::<Type>(&enum_or_struct_name)?)
    }

    // the function signature is not covered while the whole body is covered - looks like a bug in coverage tool
    // frb-coverage:ignore-start
    fn parse_datetime(&mut self, args: &[Type]) -> anyhow::Result<IrType> {
        // frb-coverage:ignore-end
        let inner = self.parse_type(&args[0])?;
        if let IrType::RustAutoOpaque(inner) = &inner {
            return Ok(match splay_segments(&inner.raw.segments).last().unwrap() {
                ("Utc", []) => Delegate(IrTypeDelegate::Time(IrTypeDelegateTime::Utc)),
                ("Local", []) => Delegate(IrTypeDelegate::Time(IrTypeDelegateTime::Local)),
                // This will stop the whole generator and tell the users, so we do not care about testing it
                // frb-coverage:ignore-start
                _ => bail!("Invalid DateTime generic: {args:?}"),
            });
        }
        bail!("Invalid DateTime generic: {args:?}")
        // frb-coverage:ignore-end
    }
}
