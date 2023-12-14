use polars_core::prelude::*;
use polars_lazy::prelude::*;

// This demo is a minimal version of https://github.com/Desdaemon/polars_dart
// Refer to that repository for more details (though may have not migrated to V2 yet)

// Thanks for the `iris` dataset
static SAMPLE_DATASET: &'static str = include_str!("../ignore_me/iris.csv");

pub fn hello_polars() -> String {
    // demo from https://pola-rs.github.io/polars/
    let df = LazyCsvReader::new("docs/data/iris.csv")
        .has_header(true)
        .finish()?
        .filter(col("sepal_length").gt(lit(5)))
        .group_by(vec![col("species")])
        .agg([col("*").sum()])
        .collect()?;
    format!("{}", df)
}
