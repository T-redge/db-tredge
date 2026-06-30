use std::str::FromStr;

use proc_macro::{Ident, Span, TokenStream, TokenTree};

#[proc_macro]
pub fn struct_name(item: TokenStream) -> TokenStream {
    let mut ident = vec![];
    for id in item {
        match id {
            TokenTree::Ident(i) => {
                ident.push(i);
            }
            _ => {}
        }
    }
    TokenStream::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let x = struct_name!();
        assert_eq!(x, "TablePerson")
    }
}
