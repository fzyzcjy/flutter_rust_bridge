use log::info;

#[derive(Debug)]
pub struct NewTypeInt(pub i64);

pub fn handle_newtype(arg: NewTypeInt) -> NewTypeInt {
    info!("handle_newtype({:?})", &arg);
    NewTypeInt(arg.0 * 2)
}

// TODO use auto generated sync code instead
// pub fn handle_newtype_sync(arg: NewTypeInt) -> SyncReturn<NewTypeInt> {
//     info!("handle_newtype_sync({:?})", &arg);
//     SyncReturn(NewTypeInt(arg.0 * 2))
// }
