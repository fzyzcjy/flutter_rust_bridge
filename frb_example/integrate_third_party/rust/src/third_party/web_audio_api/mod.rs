pub mod context;
pub mod media_devices;
pub mod node;
pub mod worklet;

use crate::{handle_audio_node_trait_impls_marker, handle_getter_audio_param};
use flutter_rust_bridge::frb;
use web_audio_api::{AudioBuffer, AudioListener, AudioParam, AudioRenderCapacity};

#[frb(external)]
impl AudioRenderCapacity {
    #[frb(ignore)]
    pub fn set_onupdate() {}
}

#[frb(external)]
impl AudioBuffer {
    #[frb(ignore)]
    pub fn copy_from_channel() {}
    #[frb(ignore)]
    pub fn copy_from_channel_with_offset() {}
    #[frb(ignore)]
    pub fn copy_to_channel() {}
    #[frb(ignore)]
    pub fn copy_to_channel_with_offset() {}
}

#[frb(external)]
impl AudioParam {
    #[frb(sync, getter)]
    pub fn value() {}

    #[frb(sync, setter)]
    pub fn set_value() {}
}

#[frb(ignore)]
pub struct ErrorEvent;

#[frb(ignore)]
pub struct MediaElement;

handle_audio_node_trait_impls_marker!(AudioParam);

handle_getter_audio_param!(AudioListener; position_x, position_y, position_z, forward_x, forward_y, forward_z, up_x, up_y, up_z);
