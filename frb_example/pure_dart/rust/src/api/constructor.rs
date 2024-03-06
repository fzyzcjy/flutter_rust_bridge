use flutter_rust_bridge::frb;

pub struct ConstructorTranslatableStructTwinNormal {
    pub one: String,
}

impl ConstructorTranslatableStructTwinNormal {
    pub fn new() -> Self {
        Self {
            one: "hello".to_owned(),
        }
    }
}

#[frb(opaque)]
pub struct ConstructorOpaqueStructTwinNormal {
    pub one: String,
}

impl ConstructorOpaqueStructTwinNormal {
    pub fn new() -> Self {
        Self {
            one: "hello".to_owned(),
        }
    }

    pub fn check(&self) {
        assert_eq!(self.one, "hello");
    }
}
