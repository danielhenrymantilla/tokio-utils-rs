#![allow(unused_imports)]
pub_in_crate! {
    use ::std::{*,
        iter::FromIterator,
    };
    use ::proc_macro::{
        TokenStream,
    };
    use ::proc_macro2::{
        TokenStream as TokenStream2,
        Span,
    };
    use ::proc_macro_hack::{
        proc_macro_hack as proc_macro_,
    };
    use ::proc_quote::{
        quote,
        quote_spanned,
    };
    use ::syn::{*,
    };
}
