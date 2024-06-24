use flutter_rust_bridge::frb;
use web_audio_api::context::{AudioContext, AudioContextRenderSizeCategory};

#[frb(external)]
impl AudioContext {
    #[frb(sync)]
    pub fn new() {}

    #[frb(ignore)]
    pub fn create_media_element_source() {}

    #[frb(ignore)]
    pub fn set_sink_id_sync() {}

    #[frb(ignore)]
    pub fn resume() {}

    #[frb(ignore)]
    pub fn base() {}
}

#[frb(external)]
pub trait BaseAudioContext {
    #[frb(ignore)]
    fn base() {}
}

#[frb(external)]
impl Default for AudioContextRenderSizeCategory {
    #[frb(ignore)]
    fn default() -> Self {}
}

#[frb(non_opaque)]
pub struct AudioContextOptions;

#[frb(non_opaque)]
pub enum AudioContextLatencyCategory {}
