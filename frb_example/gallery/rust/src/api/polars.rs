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

#[frb(opaque)]
pub struct DataFrame(polars_core::prelude::DataFrame);

impl DataFrame {
    pub fn lazy(self) -> LazyFrame {
        LazyFrame(self.0.lazy())
    }
}

#[frb(opaque)]
pub struct LazyFrame(polars_lazy::prelude::LazyFrame);

impl LazyFrame {
    pub fn filter(self, predicate: Expr) -> LazyFrame {
        Self(self.0.filter(predicate))
    }

    pub fn group_by(self, expr: Vec<Expr>) -> LazyGroupBy {
        LazyGroupBy(self.0.group_by(expr))
    }

    pub fn collect(self) -> anyhow::Result<DataFrame> {
        Ok(DataFrame(self.0.collect()?))
    }
}

#[frb(opaque)]
pub struct LazyGroupBy(polars_lazy::prelude::LazyGroupBy);

impl LazyGroupBy {
    pub fn agg(self, expr: Vec<Expr>) -> LazyFrame {
        LazyFrame(self.0.agg(expr))
    }
}

pub fn read_sample_dataset() -> DataFrame {
    DataFrame(polars_related::create_df_iris())
}
