use crate::api::media_element::MyMediaElement;
use extend::ext;
use flutter_rust_bridge::for_generated::anyhow;
use web_audio_api::context::{AudioContext, BaseAudioContext};
use web_audio_api::node::*;
use web_audio_api::{AudioBuffer, AudioParam};
use flutter_rust_bridge::DartFnFuture;

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

    fn frb_override_create_media_element_source(
        &self,
        media_element: &mut MyMediaElement,
    ) -> MediaElementAudioSourceNode {
        self.create_media_element_source(&mut media_element.0.lock().unwrap())
    }
}

macro_rules! handle_audio_node_trait_impls_override {
    ($name:ident) => {
        #[ext]
        pub impl $name {
            fn frb_override_connect(&self, dest: &dyn AudioNode) {
                self.connect(dest);
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

// TODO use macro
#[ext(name = GainNodeSetOnprocessorErrorExt)]
pub impl GainNode {
    fn frb_override_set_onprocessorerror(
        &self,
        callback: impl Fn(String) -> DartFnFuture<()> + Send + 'static,
    ) {
        self.set_onprocessorerror(Box::new(|event| {
            flutter_rust_bridge::spawn(async move {
                callback(event.message).await;
            });
        }))
    }
}
