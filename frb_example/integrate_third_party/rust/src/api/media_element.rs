use delegate_attr::delegate;
use flutter_rust_bridge::frb;
use std::sync::Mutex;

// TODO: Dart rename `MyMediaElement` -> `MediaElement`
#[frb(opaque)]
pub struct MyMediaElement(pub(crate) Mutex<web_audio_api::MediaElement>);

#[delegate(self.0.lock().unwrap())]
impl MyMediaElement {
    pub fn current_time(&self) -> f64 {}

    pub fn set_current_time(&self, value: f64) {}

    pub fn loop_(&self) -> bool {}

    pub fn set_loop(&self, value: bool) {}

    pub fn play(&self) {}

    pub fn pause(&self) {}

    pub fn paused(&self) -> bool {}

    pub fn playback_rate(&self) -> f64 {}

    pub fn set_playback_rate(&self, value: f64) {}
}

impl MyMediaElement {
    #[frb(sync)]
    pub fn new(file: String) -> anyhow::Result<MyMediaElement> {
        Ok(MyMediaElement(Mutex::new(
            web_audio_api::MediaElement::new(file).map_err(|err| anyhow::anyhow!("{:?}", err))?,
        )))
    }
}
