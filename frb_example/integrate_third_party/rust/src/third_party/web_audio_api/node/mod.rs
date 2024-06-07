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

#[frb(external)]
pub trait AudioNode {
    #[frb(ignore)]
    fn set_onprocessorerror();
    #[frb(ignore)]
    fn connect();
    #[frb(ignore)]
    fn connect_from_output_to_input();
    #[frb(ignore)]
    fn disconnect_dest();
    #[frb(ignore)]
    fn disconnect_dest_from_output();
    #[frb(ignore)]
    fn disconnect_dest_from_output_to_input();
}

#[macro_export]
macro_rules! handle_audio_node_trait_impls {
    ($name:ident) => {
        #[frb(external)]
        impl $name {
            #[frb(ignore)]
            pub fn set_onprocessorerror() {}

            #[frb(ignore)]
            pub fn connect() {}
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

handle_audio_node_trait_impls!(AnalyserNode);
handle_audio_node_trait_impls!(AudioBufferSourceNode);
handle_audio_node_trait_impls!(AudioDestinationNode);
handle_audio_node_trait_impls!(BiquadFilterNode);
handle_audio_node_trait_impls!(ChannelMergerNode);
handle_audio_node_trait_impls!(ChannelSplitterNode);
handle_audio_node_trait_impls!(ConstantSourceNode);
handle_audio_node_trait_impls!(ConvolverNode);
handle_audio_node_trait_impls!(DelayNode);
handle_audio_node_trait_impls!(DynamicsCompressorNode);
handle_audio_node_trait_impls!(GainNode);
handle_audio_node_trait_impls!(IIRFilterNode);
handle_audio_node_trait_impls!(MediaElementAudioSourceNode);
handle_audio_node_trait_impls!(MediaStreamAudioDestinationNode);
handle_audio_node_trait_impls!(MediaStreamAudioSourceNode);
handle_audio_node_trait_impls!(MediaStreamTrackAudioSourceNode);
handle_audio_node_trait_impls!(OscillatorNode);
handle_audio_node_trait_impls!(PannerNode);
handle_audio_node_trait_impls!(ScriptProcessorNode);
handle_audio_node_trait_impls!(StereoPannerNode);
handle_audio_node_trait_impls!(WaveShaperNode);

#[macro_export]
macro_rules! handle_getter_audio_param {
    ($struct_name:ident ; $($func_name:ident),+) => {
        #[frb(external)]
        impl $struct_name {
            $(
                #[frb(ignore)]
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
