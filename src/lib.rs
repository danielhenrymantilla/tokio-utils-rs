#[::proc_macro_hack::proc_macro_hack]
pub use proc_macro::future_match;

#[doc(hidden)]
pub use ::tokio::{self,
    prelude::{Async, Future},
};

#[doc(hidden)]
pub use ::core::result::Result;

#[macro_export]
macro_rules! future_chain {
    (
        $end:expr
    ) => (
        $end
    );

    (
        $(let _: $T:ty =)? $end:expr;
    ) => (
        $crate::tokio::prelude::Future::map($end, |_ $(: $T)?| {})
    );

    (
        $(let $pat:pat =)? $one:expr;
        $( $rest:tt )*
    ) => (
        $crate::tokio::prelude::Future::and_then(
            $one,
            move |$crate::future_chain!(@parens $( $pat )? )|
            $crate::future_chain! {
                $($rest)+
            },
        )
    );

    (@parens
        $($tt:tt)+
    ) => (
        $($tt)+
    );

    (@parens
    ) => (
        ()
    );
}

pub
fn future_unreachable<T, E> ()
-> ::tokio::prelude::future::FutureResult<T, E>
{
    unreachable!()
}
