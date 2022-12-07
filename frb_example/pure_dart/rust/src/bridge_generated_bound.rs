use crate::api::*;
use flutter_rust_bridge::Opaque;

pub enum DebugEnum {
    HideData(Opaque<HideData>),
    Log2(Log2),
    MyTreeNode(MyTreeNode),
    Element(Element),
    Note(Note),
    FeatureChrono(FeatureChrono),
    FeatureUuid(FeatureUuid),
    Attribute(Attribute),
    NewTypeInt(NewTypeInt),
    Log(Log),
    Customized(Customized),
    MySize(MySize),
    ExoticOptionals(Box<ExoticOptionals>),
}
impl Debug for DebugEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            DebugEnum::HideData(ref __field0) => __field0.fmt(f),
            DebugEnum::Log2(ref __field0) => __field0.fmt(f),
            DebugEnum::MyTreeNode(ref __field0) => __field0.fmt(f),
            DebugEnum::Element(ref __field0) => __field0.fmt(f),
            DebugEnum::Note(ref __field0) => __field0.fmt(f),
            DebugEnum::FeatureChrono(ref __field0) => __field0.fmt(f),
            DebugEnum::FeatureUuid(ref __field0) => __field0.fmt(f),
            DebugEnum::Attribute(ref __field0) => __field0.fmt(f),
            DebugEnum::NewTypeInt(ref __field0) => __field0.fmt(f),
            DebugEnum::Log(ref __field0) => __field0.fmt(f),
            DebugEnum::Customized(ref __field0) => __field0.fmt(f),
            DebugEnum::MySize(ref __field0) => __field0.fmt(f),
            DebugEnum::ExoticOptionals(ref __field0) => __field0.fmt(f),
        }
    }
}
