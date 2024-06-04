use flutter_rust_bridge::frb;
use web_audio_api::node::{
    AnalyserNode, AudioBufferSourceNode, BiquadFilterNode, ConvolverNode, IIRFilterNode,
    WaveShaperNode,
};

#[frb(external)]
impl AnalyserNode {
    #[frb(ignore)]
    pub fn get_float_time_domain_data() {}

    #[frb(ignore)]
    pub fn get_byte_time_domain_data() {}

    #[frb(ignore)]
    pub fn get_float_frequency_data() {}

    #[frb(ignore)]
    pub fn get_byte_frequency_data() {}
}

#[frb(external)]
impl BiquadFilterNode {
    #[frb(ignore)]
    pub fn get_frequency_response() {}
}

#[frb(external)]
impl IIRFilterNode {
    #[frb(ignore)]
    pub fn get_frequency_response() {}
}

#[frb(external)]
impl WaveShaperNode {
    #[frb(ignore)]
    pub fn curve() {}
}

#[frb(external)]
impl ConvolverNode {
    #[frb(ignore)]
    pub fn buffer() {}
}

#[frb(external)]
impl AudioBufferSourceNode {
    #[frb(ignore)]
    pub fn buffer() {}
}
