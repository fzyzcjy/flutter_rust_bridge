use flutter_rust_bridge::{DartSafe, RustOpaque};
use std::fmt::Debug;
use std::sync::{Mutex, RwLock};

/// Opaque types
pub trait DartDebug: DartSafe + Debug + Send + Sync {}
impl<T: DartSafe + Debug + Send + Sync> DartDebug for T {}

pub enum EnumOpaque {
    Struct(RustOpaque<HideData>),
    Primitive(RustOpaque<i32>),
    TraitObj(RustOpaque<Box<dyn DartDebug>>),
    Mutex(RustOpaque<Mutex<HideData>>),
    RwLock(RustOpaque<RwLock<HideData>>),
}

/// [`HideData`] has private fields.
pub struct OpaqueNested {
    pub first: RustOpaque<HideData>,
    pub second: RustOpaque<HideData>,
}

pub fn create_opaque() -> RustOpaque<HideData> {
    RustOpaque::new(HideData::new())
}

pub fn create_option_opaque(opaque: Option<RustOpaque<HideData>>) -> Option<RustOpaque<HideData>> {
    opaque
}

pub fn sync_create_opaque() -> SyncReturn<RustOpaque<HideData>> {
    SyncReturn(RustOpaque::new(HideData::new()))
}

pub fn create_array_opaque_enum() -> [EnumOpaque; 5] {
    [
        EnumOpaque::Struct(RustOpaque::new(HideData::new())),
        EnumOpaque::Primitive(RustOpaque::new(42)),
        EnumOpaque::TraitObj(opaque_dyn!("String")),
        EnumOpaque::Mutex(RustOpaque::new(Mutex::new(HideData::new()))),
        EnumOpaque::RwLock(RustOpaque::new(RwLock::new(HideData::new()))),
    ]
}

pub fn run_enum_opaque(opaque: EnumOpaque) -> String {
    match opaque {
        EnumOpaque::Struct(s) => run_opaque(s),
        EnumOpaque::Primitive(p) => format!("{:?}", p.deref()),
        EnumOpaque::TraitObj(t) => format!("{:?}", t.deref()),
        EnumOpaque::Mutex(m) => {
            format!("{:?}", m.lock().unwrap().hide_data())
        }
        EnumOpaque::RwLock(r) => {
            format!("{:?}", r.read().unwrap().hide_data())
        }
    }
}

pub fn run_opaque(opaque: RustOpaque<HideData>) -> String {
    opaque.hide_data()
}

pub fn run_opaque_with_delay(opaque: RustOpaque<HideData>) -> String {
    sleep(Duration::from_millis(1000));
    opaque.hide_data()
}

pub fn opaque_array() -> [RustOpaque<HideData>; 2] {
    [
        RustOpaque::new(HideData::new()),
        RustOpaque::new(HideData::new()),
    ]
}

pub fn sync_create_non_clone() -> SyncReturn<RustOpaque<NonCloneData>> {
    SyncReturn(RustOpaque::new(NonCloneData::new()))
}

#[allow(clippy::redundant_clone)]
pub fn run_non_clone(clone: RustOpaque<NonCloneData>) -> String {
    // Tests whether `.clone()` works even without the generic type wrapped by it
    // implementing Clone.
    clone.clone().hide_data()
}

pub fn create_sync_opaque() -> RustOpaque<NonSendHideData> {
    RustOpaque::new(NonSendHideData::new())
}

pub fn sync_create_sync_opaque() -> SyncReturn<RustOpaque<NonSendHideData>> {
    SyncReturn(RustOpaque::new(NonSendHideData::new()))
}

// OpaqueSyncStruct does not implement Send trait.
//
// pub fn run_opaque(opaque: Opaque<OpaqueSyncStruct>) -> String {
//     data.0.hide_data()
// }

pub fn sync_run_opaque(opaque: RustOpaque<NonSendHideData>) -> SyncReturn<String> {
    SyncReturn(opaque.hide_data())
}

pub fn opaque_array_run(data: [RustOpaque<HideData>; 2]) {
    for i in data {
        i.hide_data();
    }
}

pub fn opaque_vec() -> Vec<RustOpaque<HideData>> {
    vec![
        RustOpaque::new(HideData::new()),
        RustOpaque::new(HideData::new()),
    ]
}

pub fn opaque_vec_run(data: Vec<RustOpaque<HideData>>) {
    for i in data {
        i.hide_data();
    }
}

pub fn create_nested_opaque() -> OpaqueNested {
    OpaqueNested {
        first: RustOpaque::new(HideData::new()),
        second: RustOpaque::new(HideData::new()),
    }
}

pub fn sync_option() -> Result<SyncReturn<Option<String>>> {
    Ok(SyncReturn(Some("42".to_owned())))
}

pub fn sync_option_null() -> Result<SyncReturn<Option<String>>> {
    Ok(SyncReturn(None))
}

pub fn sync_option_rust_opaque() -> Result<SyncReturn<Option<RustOpaque<HideData>>>> {
    Ok(SyncReturn(Some(RustOpaque::new(HideData::new()))))
}

pub fn run_nested_opaque(opaque: OpaqueNested) {
    opaque.first.hide_data();
    opaque.second.hide_data();
}

pub fn unwrap_rust_opaque(opaque: RustOpaque<HideData>) -> Result<String> {
    let data: HideData = opaque
        .try_unwrap()
        .map_err(|_| anyhow::anyhow!("opaque type is shared"))?;
    Ok(data.hide_data())
}

/// Function to check the code generator.
/// FrbOpaqueReturn must be only return type.
/// FrbOpaqueReturn must not be used as an argument.
pub fn frb_generator_test() -> RustOpaque<FrbOpaqueReturn> {
    panic!("dummy code");
}

/// Structure for testing the SyncReturn<RustOpaque> code generator.
/// FrbOpaqueSyncReturn must be only return type.
/// FrbOpaqueSyncReturn must be without wrapper like Option<> Vec<> etc.
pub fn frb_sync_generator_test() -> SyncReturn<RustOpaque<FrbOpaqueSyncReturn>> {
    SyncReturn(RustOpaque::new(FrbOpaqueSyncReturn))
}
