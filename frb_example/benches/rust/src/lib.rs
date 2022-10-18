mod api;
mod bridge_generated;

#[allow(dead_code)]
impl api::Unit {
    pub fn acronym(&self) -> &str {
        match self {
            api::Unit::Milliseconds => "ms",
            api::Unit::Microseconds => "μs",
            api::Unit::Nanoseconds => "ns",
        }
    }
}
