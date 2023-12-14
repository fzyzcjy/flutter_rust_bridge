use crate::ignore_me::polars_related;
use flutter_rust_bridge::frb;
use polars_core::prelude::*;
use polars_lazy::prelude::*;

// This demo is a minimal version of https://github.com/Desdaemon/polars_dart
// Refer to that repository for more details (though may have not migrated to V2 yet)

#[frb(opaque)]
pub struct DataFrame(polars_core::prelude::DataFrame);

impl DataFrame {
    pub fn lazy(self) -> LazyFrame {
        LazyFrame(self.0.lazy())
    }

    pub fn write_json(&self) -> anyhow::Result<String> {
        todo!()
    }
}

#[frb(opaque)]
pub struct LazyFrame(polars_lazy::prelude::LazyFrame);

impl LazyFrame {
    pub fn filter(self, predicate: Expr) -> LazyFrame {
        Self(self.0.filter(predicate.0))
    }

    pub fn group_by(self, expr: Vec<Expr>) -> LazyGroupBy {
        LazyGroupBy(
            self.0
                .group_by(expr.into_iter().map(|x| x.0).collect::<Vec<_>>()),
        )
    }

    pub fn collect(self) -> anyhow::Result<DataFrame> {
        Ok(DataFrame(self.0.collect()?))
    }
}

#[frb(opaque)]
pub struct LazyGroupBy(polars_lazy::prelude::LazyGroupBy);

impl LazyGroupBy {
    pub fn agg(self, expr: Vec<Expr>) -> LazyFrame {
        LazyFrame(
            self.0
                .agg(expr.into_iter().map(|x| x.0).collect::<Vec<_>>()),
        )
    }
}

pub fn read_sample_dataset() -> DataFrame {
    DataFrame(polars_related::create_df_iris())
}

// Instead of opaque, we can also use the translatable types mode
#[frb(opaque)]
pub struct Expr(polars_lazy::prelude::Expr);

impl Expr {
    pub fn gt(self, other: Expr) -> Expr {
        Expr(self.0.gt(other.0))
    }

    pub fn sum(self) -> Expr {
        Expr(self.0.sum())
    }
}

pub fn col(name: String) -> Expr {
    Expr(polars_lazy::prelude::col(&name))
}

pub fn lit(t: i32) -> Expr {
    Expr(polars_lazy::prelude::lit(t))
}
