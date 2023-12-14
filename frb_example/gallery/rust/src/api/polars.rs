use crate::ignore_me::polars_related::create_df_iris;
use polars_core::prelude::*;
use polars_lazy::prelude::*;

// This demo is a minimal version of https://github.com/Desdaemon/polars_dart
// Refer to that repository for more details (though may have not migrated to V2 yet)

pub fn hello_polars() -> String {
    // demo from https://pola-rs.github.io/polars/
    let df = create_df_iris()
        .lazy()
        .filter(col("sepal_length").gt(lit(5)))
        .group_by(vec![col("species")])
        .agg([col("*").sum()])
        .collect()
        .unwrap();
    format!("{}", df)
}
