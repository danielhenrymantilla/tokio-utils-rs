extern crate proc_macro;

#[macro_use]
mod utils;
pub(in self) use utils::prelude;

use crate::prelude::*;

#[proc_macro_] pub
fn future_match (input: TokenStream) -> TokenStream
{
    let root: Ident = parse_str(&
        ::proc_macro_crate::crate_name("tokio-utils")
            .map(borrow::Cow::<'static, str>::from)
            .unwrap_or_else(|_| "tokio_utils".into())
    ).unwrap();
    let mut match_expr = parse_macro_input!(input as ExprMatch);
    mk_render!(ret);
    #[allow(non_snake_case)]
    let _i = Vec::from_iter(
        (1 ..= match_expr.arms.len()).map(|i| {
            Ident::new(&format!("_{}", i), Span::call_site())
        })
    );
    render! {
        pub
        enum BranchN <#(#_i),*> {
            #(
                #_i (#_i, ),
            )*
        }

        impl <Item, Error, #(#_i),*> ::#root::Future for BranchN<#(#_i),*>
        where
            #(
                #_i : ::#root::Future<Item = Item, Error = Error>,
            )*
        {
            type Item = Item;

            type Error = Error;

            fn poll (self: &'_ mut Self) -> ::#root::Result<::#root::Async<Self::Item>, Self::Error>
            {
                match self {
                    #(
                        | &mut BranchN::#_i(ref mut inner, ) => inner.poll(),
                    )*
                }
            }
        }
    }
    match_expr
        .arms
        .iter_mut()
        .zip(&_i)
        .for_each(|(arm, _i)| {
            let body = &*arm.body;
            *arm.body = parse_quote! {
                BranchN::#_i(#body, )
            };
        });
    render! {
        #match_expr
    }
    let ret = TokenStream2::from(ret);
    TokenStream::from(quote! {
        { #ret }
    })
}
