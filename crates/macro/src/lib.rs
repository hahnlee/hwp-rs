use proc_macro::Literal;
use proc_macro::TokenStream;
use proc_macro::TokenTree;

fn as_literal(token: &TokenTree) -> &Literal {
    match token {
        TokenTree::Literal(literal) => Some(literal),
        _ => None,
    }
    .unwrap()
}

#[proc_macro]
pub fn make_4chid(token: TokenStream) -> TokenStream {
    let literals = token
        .into_iter()
        .filter(|x| match x {
            TokenTree::Literal(_) => true,
            _ => false,
        })
        .collect::<Vec<TokenTree>>();

    if literals.len() != 4 {
        panic!("make_4chid의 인자는 4개여야합니다");
    }

    let a = as_literal(&literals[0]).to_string().chars().nth(1).unwrap();
    let b = as_literal(&literals[1]).to_string().chars().nth(1).unwrap();
    let c = as_literal(&literals[2]).to_string().chars().nth(1).unwrap();
    let d = as_literal(&literals[3]).to_string().chars().nth(1).unwrap();

    let ctrl_id = ((a as u32) << 24) | ((b as u32) << 16) | ((c as u32) << 8) | (d as u32);

    format!("{ctrl_id}").parse().unwrap()
}
