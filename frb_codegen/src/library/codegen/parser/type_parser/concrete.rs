use crate::codegen::ir::ty::boxed::IrTypeBoxed;
use crate::codegen::ir::ty::dart_opaque::IrTypeDartOpaque;
use crate::codegen::ir::ty::delegate::{
    IrTypeDelegate, IrTypeDelegateMap, IrTypeDelegateSet, IrTypeDelegateTime,
};
use crate::codegen::ir::ty::dynamic::IrTypeDynamic;
use crate::codegen::ir::ty::general_list::ir_list;
use crate::codegen::ir::ty::unencodable::IrTypeUnencodable;
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::{Boxed, DartOpaque, Delegate, Dynamic, Unencodable};
use crate::codegen::parser::type_parser::unencodable::ArgsRefs::Generic;
use crate::codegen::parser::type_parser::unencodable::{splay_segments, SplayedSegment};
use crate::codegen::parser::type_parser::{ParsingLocation, TypeParserWithContext};
use anyhow::bail;

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_type_path_data_concrete(
        &mut self,
        last_segment: &SplayedSegment,
    ) -> anyhow::Result<Option<IrType>> {
        Ok(Some(match last_segment {
            ("Duration", None) => Delegate(IrTypeDelegate::Time(IrTypeDelegateTime::Duration)),
            ("NaiveDateTime", None) => Delegate(IrTypeDelegate::Time(IrTypeDelegateTime::Naive)),
            ("DateTime", Some(Generic(args))) => parse_datetime(args)?,

            ("Uuid", None) => Delegate(IrTypeDelegate::Uuid),
            ("String", None) => Delegate(IrTypeDelegate::String),
            ("Backtrace", None) => Delegate(IrTypeDelegate::Backtrace),

            ("DartAbi", None) => Dynamic(IrTypeDynamic),
            ("DartDynamic", None) => Dynamic(IrTypeDynamic),

            ("DartOpaque", None) => DartOpaque(IrTypeDartOpaque {}),

            ("ZeroCopyBuffer", _) => bail!("`ZeroCopyBuffer<T>` is no longer needed, since zero-copy is automatically utilized, just directly use `T` instead."),
            // (
            //     "ZeroCopyBuffer",
            //     Some(Generic([PrimitiveList(IrTypePrimitiveList { primitive })])),
            // ) => Delegate(IrTypeDelegate::ZeroCopyBufferVecPrimitive(
            //     primitive.clone(),
            // )),

            ("Box", Some(Generic([inner]))) => Boxed(IrTypeBoxed {
                exist_in_real_api: true,
                inner: Box::new(inner.clone()),
            }),

            ("Vec", Some(Generic([element]))) => {
                let strict_dart_type = match self.context.location {
                    ParsingLocation::Param => TODO,
                    ParsingLocation::Return => true,
                    // frb-coverage:ignore-start
                    ParsingLocation::Misc => unreachable!(),
                    // frb-coverage:ignore-end
                };
                ir_list(element.to_owned(), strict_dart_type)
            }

            ("HashMap", Some(Generic([key, value]))) => Delegate(IrTypeDelegate::Map(IrTypeDelegateMap {
                key: Box::new(key.clone()),
                value: Box::new(value.clone()),
                element_delegate: self.create_ir_record(vec![
                    key.clone(),
                    value.clone(),
                ]),
            })),
            ("HashSet", Some(Generic([inner]))) => Delegate(IrTypeDelegate::Set(IrTypeDelegateSet {
                inner: Box::new(inner.clone()),
            })),

            _ => return Ok(None),
        }))
    }
}

// the function signature is not covered while the whole body is covered - looks like a bug in coverage tool
// frb-coverage:ignore-start
fn parse_datetime(args: &[IrType]) -> anyhow::Result<IrType> {
    // frb-coverage:ignore-end
    if let [Unencodable(IrTypeUnencodable { segments, .. })] = args {
        return Ok(match splay_segments(segments).last().unwrap() {
            ("Utc", None) => Delegate(IrTypeDelegate::Time(IrTypeDelegateTime::Utc)),
            ("Local", None) => Delegate(IrTypeDelegate::Time(IrTypeDelegateTime::Local)),
            // This will stop the whole generator and tell the users, so we do not care about testing it
            // frb-coverage:ignore-start
            _ => bail!("Invalid DateTime generic: {args:?}"),
        });
    }
    bail!("Invalid DateTime generic: {args:?}")
    // frb-coverage:ignore-end
}
