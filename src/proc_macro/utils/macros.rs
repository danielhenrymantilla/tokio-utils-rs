macro_rules! pub_in_crate {(
    $(
        use $(::)? $($ident:ident ::)+ {
            $($tt:tt)*
        };
    )*
) => (
    $(
        pub(in crate)
        use $($ident ::)+ {
            $($tt)*
        };
    )*
)}

macro_rules! mk_render {
    (@with_dollar
        [$dol:tt]
        $ret:ident
    ) => (
        let mut $ret = TokenStream::new();
        macro_rules! render_spanned {(
            $dol span:expr =>
            $dol($dol tt:tt)*
        ) => (
            $ret.extend(TokenStream::from({
                let span = $dol span;
                quote_spanned! { span=>
                    $dol($dol tt)*
                }
            }))
        )}
        macro_rules! render {(
            $dol($dol tt:tt)*
        ) => (
            render_spanned! { Span::call_site() =>
                $dol($dol tt)*
            }
        )}
    );

    (
        $ret:ident
    ) => (
        mk_render!(@with_dollar [$]
            $ret
        )
    );
}
