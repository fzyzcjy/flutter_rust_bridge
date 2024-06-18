use crate::codegen::generator::codec::structs::CodecMode;
use crate::codegen::ir::mir::func::MirFuncOwnerInfo;
use crate::codegen::ir::mir::ty::boxed::MirTypeBoxed;
use crate::codegen::ir::mir::ty::dart_opaque::MirTypeDartOpaque;
use crate::codegen::ir::mir::ty::delegate::{
    MirTypeDelegate, MirTypeDelegateMap, MirTypeDelegateSet, MirTypeDelegateStreamSink,
    MirTypeDelegateTime,
};
use crate::codegen::ir::mir::ty::dynamic::MirTypeDynamic;
use crate::codegen::ir::mir::ty::general_list::mir_list;
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::ir::mir::ty::MirType::{Boxed, DartOpaque, Delegate, Dynamic};
use crate::codegen::parser::mir::parser::ty::path_data::extract_path_data;
use crate::codegen::parser::mir::parser::ty::unencodable::{splay_segments, SplayedSegment};
use crate::codegen::parser::mir::parser::ty::TypeParserWithContext;
use crate::if_then_some;
use anyhow::{bail, Context};
use itertools::Itertools;
use syn::{parse_str, Type};

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_type_path_data_concrete(
        &mut self,
        last_segment: &SplayedSegment,
        splayed_segments: &[SplayedSegment],
    ) -> anyhow::Result<Option<MirType>> {
        let non_last_segments = (splayed_segments.split_last().unwrap().1.iter())
            .map(|segment| segment.0)
            .join("::");
        let check_prefix =
            |matcher: &str| non_last_segments == matcher || non_last_segments.is_empty();

        Ok(Some(match last_segment {
            ("Self", []) => self.parse_type_self()?,

            ("Duration", []) if check_prefix("chrono") => Delegate(MirTypeDelegate::Time(MirTypeDelegateTime::Duration)),
            ("NaiveDateTime", []) if check_prefix("chrono") => Delegate(MirTypeDelegate::Time(MirTypeDelegateTime::Naive)),
            ("DateTime", args) if check_prefix("chrono") => self.parse_datetime(args)?,

            ("Uuid", []) if check_prefix("uuid") => Delegate(MirTypeDelegate::Uuid),
            ("String", []) | ("str", []) => Delegate(MirTypeDelegate::String),
            ("char", []) => Delegate(MirTypeDelegate::Char),
            ("Backtrace", []) => Delegate(MirTypeDelegate::Backtrace),

            ("DartAbi", []) => Dynamic(MirTypeDynamic),
            ("DartDynamic", []) => Dynamic(MirTypeDynamic),

            ("DartOpaque", []) => DartOpaque(MirTypeDartOpaque {}),

            ("ZeroCopyBuffer", _) => bail!("`ZeroCopyBuffer<T>` is no longer needed, since zero-copy is automatically utilized, just directly use `T` instead."),
            // (
            //     "ZeroCopyBuffer",
            //     Some(Generic([PrimitiveList(MirTypePrimitiveList { primitive })])),
            // ) => Delegate(MirTypeDelegate::ZeroCopyBufferVecPrimitive(
            //     primitive.clone(),
            // )),

            ("Box", [inner]) => {
                let inner = self.parse_type(inner)?;
                match inner {
                    MirType::RustAutoOpaqueImplicit(ty_raw) => self.transform_rust_auto_opaque(
                        &ty_raw,
                        |raw| format!("Box<{raw}>"),
                    )?,
                    _ => Boxed(MirTypeBoxed {
                        exist_in_real_api: true,
                        inner: Box::new(inner),
                    })
                }
            },

            ("Vec", [element]) => mir_list(self.parse_type(element)?, true),

            ("HashMap", [key, value]) => {
                let key  = self.parse_type(key)?;
                let value  = self.parse_type(value)?;
                Delegate(MirTypeDelegate::Map(MirTypeDelegateMap {
                    key: Box::new(key.clone()),
                    value: Box::new(value.clone()),
                    element_delegate: self.create_mir_record(vec![key, value]),
                }))
            },
            ("HashSet", [inner]) => Delegate(MirTypeDelegate::Set(MirTypeDelegateSet {
                inner: Box::new(self.parse_type(inner)?),
            })),

            ("StreamSink", [inner ]) => Delegate(MirTypeDelegate::StreamSink(MirTypeDelegateStreamSink {
                inner_ok: Box::new(self.parse_type(inner)?),
                inner_err: stream_sink_err_type(),
                codec: self.context.default_stream_sink_codec,
            })),
            ("StreamSink", [inner, codec ]) => Delegate(MirTypeDelegate::StreamSink(MirTypeDelegateStreamSink {
                inner_ok: Box::new(self.parse_type(inner)?),
                inner_err: stream_sink_err_type(),
                codec: parse_stream_sink_codec(codec)?,
            })),

            _ => return Ok(None),
        }))
    }

    fn parse_type_self(&mut self) -> anyhow::Result<MirType> {
        if self.context.forbid_type_self {
            bail!("{}", ERROR_MESSAGE_FORBID_TYPE_SELF);
        }

        let enum_or_struct_name = if_then_some!(
            let MirFuncOwnerInfo::Method(info) = self.context.owner.as_ref().context("owner is null")?,
            info.owner_ty_name().context("owner_ty_name is null")?.name.clone()
        ).context("name is null")?;
        self.parse_type(&parse_str::<Type>(&enum_or_struct_name)?)
    }

    // the function signature is not covered while the whole body is covered - looks like a bug in coverage tool
    // frb-coverage:ignore-start
    fn parse_datetime(&mut self, args: &[Type]) -> anyhow::Result<MirType> {
        // frb-coverage:ignore-end
        let inner = self.parse_type(&args[0])?;
        if let MirType::RustAutoOpaqueImplicit(inner) = &inner {
            return Ok(match splay_segments(&inner.raw.segments).last().unwrap() {
                ("Utc", []) => Delegate(MirTypeDelegate::Time(MirTypeDelegateTime::Utc)),
                ("Local", []) => Delegate(MirTypeDelegate::Time(MirTypeDelegateTime::Local)),
                // This will stop the whole generator and tell the users, so we do not care about testing it
                // frb-coverage:ignore-start
                _ => bail!("Invalid DateTime generic: {args:?}"),
            });
        }
        bail!("Invalid DateTime generic: {args:?}")
        // frb-coverage:ignore-end
    }
}

fn parse_stream_sink_codec(codec: &Type) -> anyhow::Result<CodecMode> {
    let path = if_then_some!(let Type::Path(path) = codec, path).unwrap();
    let segments = extract_path_data(&path.path)?;
    let ident = &segments.last().unwrap().ident;
    let ident_stripped = ident.strip_suffix("Codec").unwrap();
    (ident_stripped.parse()).with_context(|| format!("raw: {ident_stripped}"))
}

fn stream_sink_err_type() -> Box<MirType> {
    Box::new(MirType::Delegate(MirTypeDelegate::AnyhowException))
}

pub(crate) const ERROR_MESSAGE_FORBID_TYPE_SELF: &str = "ERROR_MESSAGE_FORBID_TYPE_SELF";
