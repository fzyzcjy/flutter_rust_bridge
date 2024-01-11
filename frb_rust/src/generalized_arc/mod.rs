pub(crate) mod base_arc;
pub(crate) mod map_based_arc;
pub(crate) mod std_arc;

#[cfg(test)]
mod tests {
    use crate::generalized_arc::base_arc::BaseArc;
    use crate::generalized_arc::map_based_arc::MapBasedArc;
    use crate::generalized_arc::std_arc::StdArc;

    // Do NOT make it `clone` (to test non-clone behavior)
    struct DummyType(String);

    #[test]
    fn test_std_arc() {
        body::<StdArc<DummyType>>();
    }

    #[test]
    fn test_map_based_arc() {
        body::<MapBasedArc<DummyType>>();
    }

    fn body<T: BaseArc<DummyType>>() {
        todo!()
    }
}
