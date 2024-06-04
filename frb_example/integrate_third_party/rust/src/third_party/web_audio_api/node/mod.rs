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
}

#[frb(external)]
impl AudioBufferSourceNode {
    #[frb(ignore)]
    pub fn buffer() {}
}

#[frb(external)]
impl BiquadFilterNode {
    #[frb(ignore)]
    pub fn get_frequency_response() {}
}

#[frb(external)]
impl ConvolverNode {
    #[frb(ignore)]
    pub fn buffer() {}
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

macro_rules! handle_audio_node {
    ($name:ident) => {
        #[frb(external)]
        impl $name {
            #[frb(ignore)]
            pub fn set_onprocessorerror() {}
        }
    };
}

handle_audio_node!(AnalyserNode);
handle_audio_node!(AudioBufferSourceNode);
handle_audio_node!(AudioDestinationNode);
handle_audio_node!(BiquadFilterNode);
handle_audio_node!(ChannelMergerNode);
handle_audio_node!(ChannelSplitterNode);
handle_audio_node!(ConstantSourceNode);
handle_audio_node!(ConvolverNode);
handle_audio_node!(DelayNode);
handle_audio_node!(DynamicsCompressorNode);
handle_audio_node!(GainNode);
handle_audio_node!(IIRFilterNode);
handle_audio_node!(MediaElementAudioSourceNode);
handle_audio_node!(MediaStreamAudioDestinationNode);
handle_audio_node!(MediaStreamAudioSourceNode);
handle_audio_node!(MediaStreamTrackAudioSourceNode);
handle_audio_node!(OscillatorNode);
handle_audio_node!(PannerNode);
handle_audio_node!(ScriptProcessorNode);
handle_audio_node!(StereoPannerNode);
handle_audio_node!(WaveShaperNode);
