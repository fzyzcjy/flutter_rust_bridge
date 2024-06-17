use flutter_rust_bridge::frb;
use web_audio_api::worklet::AudioWorkletNode;

#[frb(external)]
impl AudioWorkletNode {
    #[frb(ignore)]
    pub fn port() {}

    #[frb(ignore)]
    pub fn set_onprocessorerror() {}

    #[frb(ignore)]
    pub fn disconnect_dest() {}

    #[frb(ignore)]
    pub fn disconnect_dest_from_output() {}

    #[frb(ignore)]
    pub fn disconnect_dest_from_output_to_input() {}
}

#[frb(external)]
pub trait AudioWorkletProcessor {
    #[frb(ignore)]
    fn constructor();

    #[frb(ignore)]
    fn onmessage();
}
