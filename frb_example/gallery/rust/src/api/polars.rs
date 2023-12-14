use polars_core::prelude::*;
use polars_lazy::prelude::*;

// This demo is a minimal version of https://github.com/Desdaemon/polars_dart
// Refer to that repository for more details (though may have not migrated to V2 yet)

// Thanks for the `iris` dataset
static SAMPLE_DATASET: &'static str = include_str!("../ignore_me/iris.csv");

pub fn hello_polars() -> String {
    let df: DataFrame = df!(
        "integer" => &[1, 2, 3, 4, 5],
        "float" => &[4.0, 5.0, 6.0, 7.0, 8.0],
    )
    .unwrap();
    format!("{}", df)
}
