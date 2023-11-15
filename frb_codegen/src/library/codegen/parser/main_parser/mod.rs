use crate::codegen::ir::pack::IrPack;
use crate::codegen::parser::type_parser::TypeParser;
use crate::codegen::parser::ParserResult;
use syn::ItemFn;

pub(crate) struct MainParser<'a> {
    type_parser: TypeParser<'a>,
}

impl<'a> MainParser<'a> {
    pub(crate) fn new(type_parser: TypeParser<'a>) -> Self {
        MainParser { type_parser }
    }

    pub(crate) fn parse(
        mut self,
        source_rust_content: &str,
        src_fns: Vec<ItemFn>,
    ) -> ParserResult<IrPack> {
        todo!()
    }
}
