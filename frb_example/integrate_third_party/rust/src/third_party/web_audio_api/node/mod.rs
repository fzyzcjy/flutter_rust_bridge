use flutter_rust_bridge::frb;
use web_audio_api::node::*;

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

    #[frb(ignore)]
    pub fn set_onprocessorerror() {}
}

#[frb(external)]
impl AudioBufferSourceNode {
    #[frb(ignore)]
    pub fn buffer() {}

    #[frb(ignore)]
    pub fn set_onprocessorerror() {}
}

#[frb(external)]
impl AudioDestinationNode {
    #[frb(ignore)]
    pub fn set_onprocessorerror() {}
}

#[frb(external)]
impl BiquadFilterNode {
    #[frb(ignore)]
    pub fn get_frequency_response() {}

    #[frb(ignore)]
    pub fn set_onprocessorerror() {}
}

#[frb(external)]
impl ChannelMergerNode {
    #[frb(ignore)]
    pub fn set_onprocessorerror() {}
}

#[frb(external)]
impl ChannelSplitterNode {
    #[frb(ignore)]
    pub fn set_onprocessorerror() {}
}

#[frb(external)]
impl ConstantSourceNode {
    #[frb(ignore)]
    pub fn set_onprocessorerror() {}
}

#[frb(external)]
impl ConvolverNode {
    #[frb(ignore)]
    pub fn buffer() {}

    #[frb(ignore)]
    pub fn set_onprocessorerror() {}
}

#[frb(external)]
impl DelayNode {
    #[frb(ignore)]
    pub fn set_onprocessorerror() {}
}

#[frb(external)]
impl DynamicsCompressorNode {
    #[frb(ignore)]
    pub fn set_onprocessorerror() {}
}

#[frb(external)]
impl GainNode {
    #[frb(ignore)]
    pub fn set_onprocessorerror() {}
}

#[frb(external)]
impl IIRFilterNode {
    #[frb(ignore)]
    pub fn get_frequency_response() {}

    #[frb(ignore)]
    pub fn set_onprocessorerror() {}
}

#[frb(external)]
impl MediaElementAudioSourceNode {
    #[frb(ignore)]
    pub fn set_onprocessorerror() {}
}

#[frb(external)]
impl MediaStreamAudioDestinationNode {
    #[frb(ignore)]
    pub fn set_onprocessorerror() {}
}

#[frb(external)]
impl MediaStreamAudioSourceNode {
    #[frb(ignore)]
    pub fn set_onprocessorerror() {}
}

#[frb(external)]
impl MediaStreamTrackAudioSourceNode {
    #[frb(ignore)]
    pub fn set_onprocessorerror() {}
}

#[frb(external)]
impl OscillatorNode {
    #[frb(ignore)]
    pub fn set_onprocessorerror() {}
}

#[frb(external)]
impl PannerNode {
    #[frb(ignore)]
    pub fn disconnect_dest() {}

    #[frb(ignore)]
    pub fn disconnect_dest_from_output() {}

    #[frb(ignore)]
    pub fn disconnect_dest_from_output_to_input() {}

    #[frb(ignore)]
    pub fn set_onprocessorerror() {}
}

#[frb(ignore)]
    pub fn set_onprocessorerror() {}
}

#[frb(external)]
impl ScriptProcessorNode {
    #[frb(ignore)]
    pub fn disconnect_dest() {}

    #[frb(ignore)]
    pub fn disconnect_dest_from_output() {}

    #[frb(ignore)]
    pub fn disconnect_dest_from_output_to_input() {}

    #[frb(ignore)]
    pub fn set_onprocessorerror() {}
}

#[frb(external)]
impl StereoPannerNode {
    #[frb(ignore)]
    pub fn disconnect_dest() {}

    #[frb(ignore)]
    pub fn disconnect_dest_from_output() {}

    #[frb(ignore)]
    pub fn set_onprocessorerror() {}
}

#[frb(external)]
impl WaveShaperNode {
    #[frb(ignore)]
    pub fn curve() {}

    #[frb(ignore)]
    pub fn disconnect_dest_from_output() {}

    #[frb(ignore)]
    pub fn disconnect_dest_from_output_to_input() {}

    #[frb(ignore)]
    pub fn set_onprocessorerror() {}
}
