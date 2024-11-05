#[flutter_rust_bridge::frb(sync)] // Synchronous mode for simplicity of the demo
pub fn greet(name: String) -> String {
    format!("Hello, {name}!")
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}

use flutter_rust_bridge::frb;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TrainingPlan {
    pub(crate) weeks: u8,
}

impl TrainingPlan {
    #[frb(sync)]
    pub fn test_deserialize(content: String) -> Result<Self, String> {
        serde_json::from_str(&content).map_err(|e| e.to_string())
    }
}
