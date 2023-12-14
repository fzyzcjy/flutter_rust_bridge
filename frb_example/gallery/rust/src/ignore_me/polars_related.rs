use polars_core::prelude::*;

pub fn create_df_iris() -> DataFrame {
    df!(
        "integer" => &[1, 2, 3, 4, 5],
        "float" => &[4.0, 5.0, 6.0, 7.0, 8.0],
    )
    .unwrap()
}
