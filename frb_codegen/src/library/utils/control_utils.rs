// https://stackoverflow.com/questions/63644071/idiomatic-way-to-filter-values-matching-a-pattern-in-an-iterator
#[doc(hidden)] // only to be used within crate, not for end users
#[macro_export]
macro_rules! if_then_some {
    ($cond: expr, $val: expr) => {
        if $cond {
            Some($val)
        } else {
            None
        }
    };
    (let $pattern:pat = $expr: expr, $val: expr) => {
        if let $pattern = $expr {
            Some($val)
        } else {
            None
        }
    };
}
