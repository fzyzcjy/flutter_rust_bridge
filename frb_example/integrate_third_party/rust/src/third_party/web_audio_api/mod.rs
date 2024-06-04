pub mod context;
pub mod media_devices;
pub mod media_element;
pub mod node;
pub mod worklet;

use flutter_rust_bridge::frb;
use web_audio_api::{AudioBuffer, AudioRenderCapacity};

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

#[frb(ignore)]
pub struct ErrorEvent;