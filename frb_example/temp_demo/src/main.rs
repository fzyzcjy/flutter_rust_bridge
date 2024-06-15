use self_cell::self_cell;

#[derive(Debug, Eq, PartialEq)]
struct Ast<'a>(pub Vec<&'a str>);

self_cell!(
    struct AstCell {
        owner: String,

        #[covariant]
        dependent: Ast,
    }

    impl {Debug, Eq, PartialEq}
);

fn build_ast_cell(code: &str) -> AstCell {
    // Create owning String on stack.
    let pre_processed_code = code.trim().to_string();

    // Move String into AstCell, then build Ast inplace.
    AstCell::new(pre_processed_code, |code| {
        Ast(code.split(' ').filter(|word| word.len() > 1).collect())
    })
}

fn main() {
    let ast_cell = build_ast_cell("fox = cat + dog");

    println!("ast_cell -> {:?}", &ast_cell);
    println!("ast_cell.borrow_owner() -> {:?}", ast_cell.borrow_owner());
    println!(
        "ast_cell.borrow_dependent().0[1] -> {:?}",
        ast_cell.borrow_dependent().0[1]
    );
}
