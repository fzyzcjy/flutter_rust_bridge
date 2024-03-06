// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

use log::info;

#[derive(Debug)]
pub struct NewTypeIntTwinNormal(pub i64);

pub fn handle_newtype_twin_normal(arg: NewTypeIntTwinNormal) -> NewTypeIntTwinNormal {
    info!("handle_newtype({:?})", &arg);
    NewTypeIntTwinNormal(arg.0 * 2)
}

// TODO use auto generated sync code instead
// pub fn handle_newtype_sync_twin_normal(arg: NewTypeInt) -> SyncReturn<NewTypeInt> {
//     info!("handle_newtype_sync({:?})", &arg);
//     SyncReturn(NewTypeInt(arg.0 * 2))
// }
