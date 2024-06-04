use extend::ext;
use flutter_rust_bridge::for_generated::anyhow;
use web_audio_api::context::{AudioContext, BaseAudioContext};
use web_audio_api::AudioBuffer;

#[ext]
pub impl AudioContext {
    fn frb_override_decode_audio_data_sync(
        &self,
        input_path: String,
    ) -> anyhow::Result<AudioBuffer> {
        let input = std::fs::File::open(input_path)?;
        self
            .decode_audio_data_sync(input)
            .map_err(|e| anyhow::anyhow!("{:?}", e))
    }

    // TODO only to test overriding an existing function, not to be really used (and should be reverted later)!
    fn frb_override_output_latency(&self) -> String {
        "hello this is dummy override".to_owned()
    }
}
