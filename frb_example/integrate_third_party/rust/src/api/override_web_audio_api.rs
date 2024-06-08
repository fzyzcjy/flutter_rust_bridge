use crate::frb_generated::AudioNodeImplementor;
use extend::ext;
use flutter_rust_bridge::for_generated::anyhow;
use web_audio_api::context::{AudioContext, BaseAudioContext};
use web_audio_api::node::*;
use web_audio_api::{AudioBuffer, AudioParam};

#[ext]
pub impl AudioContext {
    fn frb_override_decode_audio_data_sync(
        &self,
        input_path: String,
    ) -> anyhow::Result<AudioBuffer> {
        let input = std::fs::File::open(input_path)?;
        self.decode_audio_data_sync(input)
            .map_err(|e| anyhow::anyhow!("{:?}", e))
    }
}

macro_rules! handle_audio_node_trait_impls_override {
    ($name:ident) => {
        #[ext]
        pub impl $name {
            fn frb_override_connect(&self, dest: AudioNodeImplementor) {
                let dest = dest.blocking_read();
                self.connect(&*dest);
            }
        }
    };
}

handle_audio_node_trait_impls_override!(AudioParam);
handle_audio_node_trait_impls_override!(AnalyserNode);
handle_audio_node_trait_impls_override!(AudioBufferSourceNode);
handle_audio_node_trait_impls_override!(AudioDestinationNode);
handle_audio_node_trait_impls_override!(BiquadFilterNode);
handle_audio_node_trait_impls_override!(ChannelMergerNode);
handle_audio_node_trait_impls_override!(ChannelSplitterNode);
handle_audio_node_trait_impls_override!(ConstantSourceNode);
handle_audio_node_trait_impls_override!(ConvolverNode);
handle_audio_node_trait_impls_override!(DelayNode);
handle_audio_node_trait_impls_override!(DynamicsCompressorNode);
handle_audio_node_trait_impls_override!(GainNode);
handle_audio_node_trait_impls_override!(IIRFilterNode);
handle_audio_node_trait_impls_override!(MediaElementAudioSourceNode);
handle_audio_node_trait_impls_override!(MediaStreamAudioDestinationNode);
handle_audio_node_trait_impls_override!(MediaStreamAudioSourceNode);
handle_audio_node_trait_impls_override!(MediaStreamTrackAudioSourceNode);
handle_audio_node_trait_impls_override!(OscillatorNode);
handle_audio_node_trait_impls_override!(PannerNode);
handle_audio_node_trait_impls_override!(ScriptProcessorNode);
handle_audio_node_trait_impls_override!(StereoPannerNode);
handle_audio_node_trait_impls_override!(WaveShaperNode);

macro_rules! handle_getter_audio_param_override {
    ($struct_name:ident ; $($func_name:ident),+) => {
        #[ext]
        pub impl $struct_name {
            $(
                pub fn TODO() {}
            )+
        }
    };
}

handle_getter_audio_param_override!(AudioListener; position_x, position_y, position_z, forward_x, forward_y, forward_z, up_x, up_y, up_z);
handle_getter_audio_param_override!(AudioBufferSourceNode; playback_rate, detune);
handle_getter_audio_param_override!(BiquadFilterNode; gain, frequency, detune, q);
handle_getter_audio_param_override!(ConstantSourceNode; offset);
handle_getter_audio_param_override!(DelayNode; delay_time);
handle_getter_audio_param_override!(DynamicsCompressorNode; attack, knee, ratio, release, threshold);
handle_getter_audio_param_override!(GainNode; gain);
handle_getter_audio_param_override!(OscillatorNode; frequency, detune);
handle_getter_audio_param_override!(PannerNode; position_x, position_y, position_z, orientation_x, orientation_y, orientation_z);
handle_getter_audio_param_override!(StereoPannerNode; pan);
