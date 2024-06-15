use ouroboros::self_referencing;

#[self_referencing]
pub struct LeafNodeIter {
    node: String,
    #[borrows(node)]
    #[covariant]
    iter: std::slice::Iter<'this, String>,
}

pub fn main() -> anyhow::Result<()> {
    Ok(())
}
