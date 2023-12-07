use enum_dispatch::enum_dispatch;

#[enum_dispatch(WireDartTransferGeneratorTrait)]
pub(crate) enum WireDartTransferGenerator {
    Cst(WireDartTransferCstGenerator),
    // TODO more
}

#[enum_dispatch]
pub(crate) trait WireDartTransferGeneratorTrait {}
