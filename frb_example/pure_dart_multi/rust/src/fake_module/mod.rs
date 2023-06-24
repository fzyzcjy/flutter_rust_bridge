/// A fake module, for testing `import` statement(even it is not used) in regular API .rs file.
/// Currently, if there is no valid content in a rust file,
/// frb would panic. This panic is expected in release mode, but not
/// in debug mode, so here some dummy content is added.

#[allow(unused)]
fn dummy_func() {}
