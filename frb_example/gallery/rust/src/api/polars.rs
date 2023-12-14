use crate::ignore_me::polars_related;
use flutter_rust_bridge::frb;
use polars_core::prelude::*;
use polars_lazy::prelude::*;

// This demo is a minimal version of https://github.com/Desdaemon/polars_dart
// Refer to that repository for more details (though may have not migrated to V2 yet)

// TODO rm
pub fn hello_polars() -> String {
    // demo from https://pola-rs.github.io/polars/
    let df = polars_related::create_df_iris()
        .lazy()
        .filter(col("sepal_length").gt(lit(5)))
        .group_by(vec![col("species")])
        .agg([col("*").sum()])
        .collect()
        .unwrap();
    format!("{}", df)
}

pub(crate) type PDataFrame = polars_core::prelude::DataFrame;
pub(crate) type PLazyFrame = polars_lazy::prelude::LazyFrame;

#[frb(opaque)]
pub struct DataFrame(PDataFrame);

impl DataFrame {
    pub fn lazy(self) -> LazyFrame {
        LazyFrame(self.0.lazy())
    }
}

#[frb(opaque)]
pub struct LazyFrame(PLazyFrame);

impl LazyFrame {
    pub fn filter(self, predicate: Expr) -> LazyFrame {
        Self(self.0.filter(predicate))
    }

    pub fn group_by() {
        todo!()
    }

    pub fn agg() {
        todo!()
    }

    pub fn collect() {
        todo!()
    }
}

pub fn read_sample_dataset() -> DataFrame {
    DataFrame(polars_related::create_df_iris())
}
