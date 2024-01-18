use crate::codegen::ir::ty::boxed::IrTypeBoxed;
use crate::codegen::ir::ty::dart_opaque::IrTypeDartOpaque;
use crate::codegen::ir::ty::delegate::{
    IrTypeDelegate, IrTypeDelegateMap, IrTypeDelegateSet, IrTypeDelegateTime,
};
use crate::codegen::ir::ty::dynamic::IrTypeDynamic;
use crate::codegen::ir::ty::general_list::ir_list;
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::{Boxed, DartOpaque, Delegate, Dynamic};
use crate::codegen::parser::type_parser::unencodable::{splay_segments, SplayedSegment};
use crate::codegen::parser::type_parser::TypeParserWithContext;
use anyhow::bail;
use syn::Type;

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_type_path_data_concrete(
        &mut self,
        last_segment: &SplayedSegment,
    ) -> anyhow::Result<Option<IrType>> {
        Ok(Some(match last_segment {
            ("Duration", []) => Delegate(IrTypeDelegate::Time(IrTypeDelegateTime::Duration)),
            ("NaiveDateTime", []) => Delegate(IrTypeDelegate::Time(IrTypeDelegateTime::Naive)),
            ("DateTime", args) => self.parse_datetime(args)?,

            ("Uuid", []) => Delegate(IrTypeDelegate::Uuid),
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
                Boxed(IrTypeBoxed {
                    exist_in_real_api: true,
                    inner: Box::new(inner),
                })
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

            _ => return Ok(None),
        }))
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
