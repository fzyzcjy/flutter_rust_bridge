/// a fake module, for testing import statement(even it is not used) in regular API rs file.
/// currently, if there is no valid content in a rust file,
/// frb would panic, so some dummy content is added here.

#[allow(unused)]
fn dummy_func() {}
