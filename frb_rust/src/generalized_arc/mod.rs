pub(crate) mod base_arc;
pub(crate) mod map_based_arc;
pub(crate) mod std_arc;

#[cfg(test)]
mod tests {
    use crate::base_arc_generate_tests;

    base_arc_generate_tests!();
}
