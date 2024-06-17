use crate::api::media_element::MyMediaElement;
use extend::ext;
use flutter_rust_bridge::for_generated::anyhow;
use flutter_rust_bridge::{frb, DartFnFuture};
use web_audio_api::context::{AudioContext, BaseAudioContext};
use web_audio_api::node::*;
use web_audio_api::{AudioBuffer, AudioParam, Event};

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

    fn set_sink_id(&self, sink_id: String) -> anyhow::Result<()> {
        self.set_sink_id_sync(sink_id)
            .map_err(|e| anyhow::anyhow!("{:?}", e))
    }

    fn set_on_state_change(&self, callback: impl Fn(Event) -> DartFnFuture<()> + Send + 'static) {
        self.set_onstatechange(|event| {
            flutter_rust_bridge::spawn(async move { callback(event).await });
        })
    }
}

macro_rules! handle_audio_node_trait_impls_override {
    ($name:ident) => {
        #[ext]
        pub impl $name {
            fn frb_override_connect(&self, dest: &dyn AudioNode) {
                self.connect(dest);
            }

            // NOTE: The `set_onprocessorerror` by web-audio-api is ignored,
            // while this one has a different name (note the "_")
            fn set_on_processor_error(
                &self,
                callback: impl Fn(String) -> DartFnFuture<()> + Send + 'static,
            ) {
                self.set_onprocessorerror(Box::new(|event| {
                    flutter_rust_bridge::spawn(async move { callback(event.message).await });
                }))
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

#[ext]
pub impl Event {
    #[frb(sync, getter)]
    fn type_(&self) -> String {
        self.type_.to_owned()
    }
}

#[ext]
pub impl AudioScheduledSourceNode {
    fn set_on_ended(&self, callback: impl Fn(Event) -> DartFnFuture<()> + Send + 'static) {
        self.set_onended(|event| {
            flutter_rust_bridge::spawn(async move { callback(event).await });
        })
    }
}
