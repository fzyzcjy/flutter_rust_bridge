use flutter_rust_bridge::frb;

#[frb(ignore)]
pub struct MediaDeviceInfo;

// #[frb(external)]
// impl MediaDeviceInfo {
//     #[frb(ignore)]
//     fn group_id() {}
// }

#[frb(ignore)]
fn enumerate_devices_sync() {}
