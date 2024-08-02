use crate::api::media_element::MyMediaElement;
use crate::frb_generated::FLUTTER_RUST_BRIDGE_HANDLER;
use extend::ext;
use flutter_rust_bridge::for_generated::anyhow;
use flutter_rust_bridge::{frb, BaseAsyncRuntime, DartFnFuture};
use std::sync::Arc;
use web_audio_api::context::{AudioContext, BaseAudioContext, OfflineAudioContext};
use web_audio_api::media_recorder::{BlobEvent, MediaRecorder};
use web_audio_api::media_streams::{MediaStream, MediaStreamTrack};
use web_audio_api::node::*;
use web_audio_api::{
    AudioBuffer, AudioParam, AudioProcessingEvent, ErrorEvent, Event, OfflineAudioCompletionEvent,
};

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

    fn set_on_state_change(
        &self,
        callback: impl Fn(Event) -> DartFnFuture<()> + Send + Sync + 'static,
    ) {
        let callback = Arc::new(callback);
        self.set_onstatechange(move |event| {
            let callback_cloned = callback.clone();
            FLUTTER_RUST_BRIDGE_HANDLER
                .async_runtime()
                .spawn(async move { callback_cloned(event).await });
        })
    }
}

#[ext(name = AnalyserNodeMiscExt)]
pub impl AnalyserNode {
    fn frb_override_get_byte_time_domain_data(&mut self, len: usize) -> Vec<u8> {
        let mut bins = vec![0; len];
        self.get_byte_time_domain_data(&mut bins);
        bins
    }

    fn frb_override_get_float_time_domain_data(&mut self, len: usize) -> Vec<f32> {
        let mut bins = vec![0.0; len];
        self.get_float_time_domain_data(&mut bins);
        bins
    }
}

#[ext(name = AudioBufferExt)]
pub impl AudioBuffer {
    fn frb_override_get_channel_data(&self, channel_number: usize) -> Vec<f32> {
        self.get_channel_data(channel_number).to_vec()
    }

    fn frb_override_copy_from_channel(&self, channel_number: usize) -> Vec<f32> {
        self.get_channel_data(channel_number).to_vec()
    }

    fn set_channel_data(&mut self, source: &[f32], channel_number: usize) {
        self.copy_to_channel(source, channel_number);
    }

    fn frb_override_copy_to_channel(&mut self, source: &[f32], channel_number: usize) {
        self.copy_to_channel(source, channel_number);
    }

    fn frb_override_copy_to_channel_with_offset(
        &mut self,
        source: &[f32],
        channel_number: usize,
        offset: usize,
    ) {
        self.copy_to_channel_with_offset(source, channel_number, offset);
    }

    fn get_at(&self, channel_number: usize, index: usize) -> f32 {
        self.get_channel_data(channel_number)[index]
    }

    fn set_at(&mut self, channel_number: usize, index: usize, value: f32) {
        self.get_channel_data_mut(channel_number)[index] = value
    }
}

#[ext]
pub impl OfflineAudioContext {
    fn set_on_complete(
        &self,
        callback: impl Fn(OfflineAudioCompletionEvent) -> DartFnFuture<()> + Send + Sync + 'static,
    ) {
        self.set_oncomplete(move |event| {
            FLUTTER_RUST_BRIDGE_HANDLER
                .async_runtime()
                .spawn(async move { callback(event).await });
        })
    }
}

#[ext(name = MediaRecorderMiscExt)]
pub impl MediaRecorder {
    // NOTE: The original name was `set_ondataavailable` and here the new name has `_`
    fn set_on_data_available(
        &self,
        callback: impl Fn(BlobEvent) -> DartFnFuture<()> + Send + 'static + std::marker::Sync,
    ) {
        let cb = Arc::new(callback);
        self.set_ondataavailable(move |event| {
            let callback_cloned = cb.clone();
            FLUTTER_RUST_BRIDGE_HANDLER
                .async_runtime()
                .spawn(async move { callback_cloned(event).await });
        })
    }

    fn set_on_stop(&self, callback: impl Fn(Event) -> DartFnFuture<()> + Send + 'static) {
        self.set_onstop(Box::new(|event| {
            FLUTTER_RUST_BRIDGE_HANDLER
                .async_runtime()
                .spawn(async move { callback(event).await });
        }))
    }

    fn set_on_error(&self, callback: impl FnOnce(ErrorEvent) + std::marker::Send + 'static) {
        self.set_onerror(move |event| {
            FLUTTER_RUST_BRIDGE_HANDLER
                .async_runtime()
                .spawn(async move { callback(event) });
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
                    FLUTTER_RUST_BRIDGE_HANDLER
                        .async_runtime()
                        .spawn(async move { callback(event.message).await });
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

macro_rules! handle_audio_scheduled_source_node_trait_impls_override {
    ($name:ident) => {
        paste::paste! {
            #[ext(name=[<$name ScheduledSourceNodeMiscExt>])]
            pub impl $name {
                // NOTE: The original name was `set_onended` and here the new name has `_`
                fn set_on_ended(
                    &self,
                    callback: impl Fn(Event) -> DartFnFuture<()> + Send + 'static,
                ) {
                    self.set_onended(Box::new(|event| {
                        FLUTTER_RUST_BRIDGE_HANDLER.async_runtime().spawn(async move { callback(event).await });
                    }))
                }
            }
        }
    };
}

handle_audio_scheduled_source_node_trait_impls_override!(ConstantSourceNode);
handle_audio_scheduled_source_node_trait_impls_override!(OscillatorNode);
handle_audio_scheduled_source_node_trait_impls_override!(AudioBufferSourceNode);

#[ext]
pub impl Event {
    #[frb(sync, getter)]
    fn type_(&self) -> String {
        self.type_.to_owned()
    }
}

#[ext(name = ScriptProcessorNodeMiscExt)]
pub impl ScriptProcessorNode {
    // NOTE: The original name was `set_onaudioprocess` and here the new name has `_`
    fn frb_override_set_onaudioprocess(
        &self,
        callback: impl Fn(AudioProcessingEvent) -> DartFnFuture<()> + Send + 'static + std::marker::Sync,
    ) {
        let callback = Arc::new(callback);
        self.set_onaudioprocess(move |event| {
            let callback_cloned = callback.clone();
            FLUTTER_RUST_BRIDGE_HANDLER
                .async_runtime()
                .spawn(async move { callback_cloned(event).await });
        })
    }
}

#[ext(name = AudioBufferSourceNodeMiscExt)]
pub impl AudioBufferSourceNode {
    // calls the regular fn `setBuffer()` after cloning the argument.
    fn frb_override_set_buffer(&mut self, audio_buffer: &AudioBuffer) {
        let clone = audio_buffer.clone();
        self.set_buffer(clone)
    }
}

#[ext]
pub impl MediaStream {
    fn frb_override_get_tracks(&self) -> Vec<MediaStreamTrack> {
        self.get_tracks().to_owned()
    }
}

#[ext(name = WaveShaperNodeMiscExt)]
pub impl WaveShaperNode {
    fn frb_override_curve(&self) -> Option<Vec<f32>> {
        self.curve().map(|x| x.to_owned())
    }
}
