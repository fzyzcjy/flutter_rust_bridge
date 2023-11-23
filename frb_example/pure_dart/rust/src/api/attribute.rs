use flutter_rust_bridge::frb;
use log::info;

#[frb]
#[derive(Debug, Clone)]
pub struct Customized {
    pub final_field: String,
    #[frb(non_final)]
    pub non_final_field: Option<String>,
}

pub fn handle_customized_struct(val: Customized) {
    info!("{:#?}", val);
}

#[frb]
#[derive(Debug)]
pub enum KitchenSink {
    Empty,
    #[frb(unimpl_variant_attr)]
    Primitives {
        #[frb(default = -1)]
        int32: i32,
        #[frb(unimpl_deprecated)]
        float64: f64,
        boolean: bool,
    },
    Nested(
        i32,
        #[frb(default = "KitchenSink.empty()")] Box<KitchenSink>,
    ),
    Optional(#[frb(default = -1)] Option<i32>, Option<i32>),
    Buffer(ZeroCopyBuffer<Vec<u8>>),
    Enums(#[frb(default = "Weekdays.Sunday")] Weekdays),
}

#[frb(unimpl_fn_attr)]
pub fn handle_enum_struct(val: KitchenSink) -> KitchenSink {
    use KitchenSink::*;
    use Weekdays::*;
    let inc = |x| x + 1;
    match val {
        Primitives {
            int32,
            float64,
            boolean,
        } => Primitives {
            int32: int32 + 1,
            float64: float64 + 1.,
            boolean: !boolean,
        },
        Nested(val, nested) => Nested(inc(val), Box::new(handle_enum_struct(*nested))),
        Optional(a, b) => Optional(a.map(inc), b.map(inc)),
        Buffer(ZeroCopyBuffer(mut buf)) => {
            buf.push(1);
            Buffer(ZeroCopyBuffer(buf))
        }
        Enums(day) => Enums(match day {
            Monday => Tuesday,
            Tuesday => Wednesday,
            Wednesday => Thursday,
            Thursday => Friday,
            Friday => Saturday,
            Saturday => Sunday,
            Sunday => Monday,
        }),
        _ => val,
    }
}
