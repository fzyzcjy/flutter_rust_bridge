use flutter_rust_bridge::frb;

#[frb(dart_code = "
    int testDartCode() {
        return 3;
    }
")]
pub struct Hello;
