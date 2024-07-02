use flutter_rust_bridge::frb;
use web_audio_api::node::*;

#[frb(external)]
impl AnalyserNode {
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

#[frb(external)]
impl MediaStreamAudioDestinationNode {
    #[frb(proxy)]
    pub fn stream() {}
}

#[frb(external)]
#[frb(generate_implementor_enum)]
pub trait AudioNode {
    // This returns borrowed type, but users usually already has the context (otherwise they seem
    // cannot create this node). Thus, we ignore this function currently.
    #[frb(ignore)]
    fn context();
    #[frb(ignore)]
    fn set_onprocessorerror();
    #[frb(ignore)]
    fn connect_from_output_to_input();
    #[frb(ignore)]
    fn disconnect_dest();
    #[frb(ignore)]
    fn disconnect_dest_from_output();
    #[frb(ignore)]
    fn disconnect_dest_from_output_to_input();

    // The trait definition and implementation has conflicting parameter names ("v" vs "value"),
    // thus we ignore it temporarily. To fix it, maybe we should PR the web-audio-api
    // library to ensure parameter names agree.
    #[frb(ignore)]
    fn set_channel_count();
    #[frb(ignore)]
    fn set_channel_count_mode();
    #[frb(ignore)]
    fn set_channel_interpretation();
}

#[macro_export]
macro_rules! handle_audio_node_trait_impls_marker {
    ($name:ident) => {
        #[frb(external)]
        impl $name {
            // Please refer to comments in `AudioNode`
            #[frb(ignore)]
            pub fn context() {}
            #[frb(ignore)]
            pub fn set_onprocessorerror() {}
            #[frb(ignore)]
            pub fn connect_from_output_to_input() {}
            #[frb(ignore)]
            pub fn disconnect_dest() {}
            #[frb(ignore)]
            pub fn disconnect_dest_from_output() {}
            #[frb(ignore)]
            pub fn disconnect_dest_from_output_to_input() {}
        }
    };
}

handle_audio_node_trait_impls_marker!(AnalyserNode);
handle_audio_node_trait_impls_marker!(AudioBufferSourceNode);
handle_audio_node_trait_impls_marker!(AudioDestinationNode);
handle_audio_node_trait_impls_marker!(BiquadFilterNode);
handle_audio_node_trait_impls_marker!(ChannelMergerNode);
handle_audio_node_trait_impls_marker!(ChannelSplitterNode);
handle_audio_node_trait_impls_marker!(ConstantSourceNode);
handle_audio_node_trait_impls_marker!(ConvolverNode);
handle_audio_node_trait_impls_marker!(DelayNode);
handle_audio_node_trait_impls_marker!(DynamicsCompressorNode);
handle_audio_node_trait_impls_marker!(GainNode);
handle_audio_node_trait_impls_marker!(IIRFilterNode);
handle_audio_node_trait_impls_marker!(MediaElementAudioSourceNode);
handle_audio_node_trait_impls_marker!(MediaStreamAudioDestinationNode);
handle_audio_node_trait_impls_marker!(MediaStreamAudioSourceNode);
handle_audio_node_trait_impls_marker!(MediaStreamTrackAudioSourceNode);
handle_audio_node_trait_impls_marker!(OscillatorNode);
handle_audio_node_trait_impls_marker!(PannerNode);
handle_audio_node_trait_impls_marker!(ScriptProcessorNode);
handle_audio_node_trait_impls_marker!(StereoPannerNode);
handle_audio_node_trait_impls_marker!(WaveShaperNode);

#[macro_export]
macro_rules! handle_getter_audio_param {
    ($struct_name:ident ; $($func_name:ident),+) => {
        #[frb(external)]
        impl $struct_name {
            $(
                #[frb(proxy, sync, getter)]
                pub fn $func_name() {}
            )+
        }
    };
}

handle_getter_audio_param!(AudioBufferSourceNode; playback_rate, detune);
handle_getter_audio_param!(BiquadFilterNode; gain, frequency, detune, q);
handle_getter_audio_param!(ConstantSourceNode; offset);
handle_getter_audio_param!(DelayNode; delay_time);
handle_getter_audio_param!(DynamicsCompressorNode; attack, knee, ratio, release, threshold);
handle_getter_audio_param!(GainNode; gain);
handle_getter_audio_param!(OscillatorNode; frequency, detune);
handle_getter_audio_param!(PannerNode; position_x, position_y, position_z, orientation_x, orientation_y, orientation_z);
handle_getter_audio_param!(StereoPannerNode; pan);
